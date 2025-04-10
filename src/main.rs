mod twitch_api;

use serde::Deserialize;
use std::collections::HashSet;
use std::time::Duration;
use thiserror::Error;
use tokio::time::interval;
use tracing::{debug, error, info, warn, Level};
use tracing_subscriber::FmtSubscriber;

// Import the client and its error type
use crate::twitch_api::{ApiError, TwitchClient, User};

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub twitch_client_id: String,
    pub twitch_client_secret: String,
    pub streamers: Vec<String>,
    #[serde(default = "default_check_interval")]
    pub check_interval_seconds: u64,
}

fn default_check_interval() -> u64 {
    60 // Default to 60 seconds
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Configuration loading error: {0}")]
    Config(#[from] config::ConfigError),

    #[error("Failed to get current directory: {0}")]
    CurrentDir(#[from] std::io::Error),

    // Add variant for API errors
    #[error("Twitch API error: {0}")]
    Api(#[from] ApiError),
}

// Make the result type alias use our top-level Error
type Result<T> = std::result::Result<T, Error>;

pub fn load_settings() -> Result<Settings> {
    // Use the Result alias
    let config_file_name = "config.toml";

    info!(
        "Attempting to load configuration from '{}'",
        config_file_name
    );

    let settings = config::Config::builder()
        // Look for `config.toml` in the current directory
        .add_source(config::File::with_name(config_file_name).required(true))
        // Add environment variable overrides (optional)
        // e.g., `APP_TWITCH_CLIENT_ID=...` would override `twitch_client_id`
        .add_source(
            config::Environment::with_prefix("APP")
                .separator("__")
                .ignore_empty(true),
        )
        .build()?;

    // Deserialize the configuration
    let settings: Settings = settings.try_deserialize()?;

    Ok(settings)
}

// Mark main as async using tokio
#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing subscriber
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO) // Log INFO level and above
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Setting default tracing subscriber failed");

    // Load settings
    let settings = load_settings()?;
    info!("Configuration loaded successfully!");

    // Create Twitch client
    info!("Initializing Twitch client...");
    let mut twitch_client = TwitchClient::new(
        settings.twitch_client_id.clone(),
        settings.twitch_client_secret.clone(),
    )?;

    // Authenticate with Twitch
    twitch_client.get_app_access_token().await?;
    info!("Successfully authenticated with Twitch API.");

    // Get initial user data
    let monitored_users: Vec<User> = if settings.streamers.is_empty() {
        info!("No streamers configured to monitor.");
        vec![]
    } else {
        info!("Fetching user info for: {:?}", settings.streamers);
        let users = twitch_client
            .get_users_by_login(&settings.streamers)
            .await?;
        if users.is_empty() {
            warn!("No Twitch users found for the configured login names.");
        } else {
            info!("Successfully fetched info for {} users", users.len());
        }
        users
    };

    if monitored_users.is_empty() {
        info!("Exiting as there are no valid users to monitor.");
        return Ok(());
    }

    let monitored_user_ids: Vec<String> = monitored_users.iter().map(|u| u.id.clone()).collect();
    let mut previously_live_user_ids: HashSet<String> = HashSet::new();

    info!(
        "Starting monitoring loop (checking every {} seconds)",
        settings.check_interval_seconds
    );
    let mut check_interval = interval(Duration::from_secs(settings.check_interval_seconds));

    // Main monitoring loop
    loop {
        check_interval.tick().await;
        debug!("Checking stream statuses...");

        match twitch_client
            .get_streams_by_user_id(&monitored_user_ids)
            .await
        {
            Ok(live_streams) => {
                let currently_live_user_ids: HashSet<String> =
                    live_streams.iter().map(|s| s.user_id.clone()).collect();
                debug!("Currently live: {:?}", currently_live_user_ids);
                debug!("Previously live: {:?}", previously_live_user_ids);

                // Detect streams that just went live
                for stream in &live_streams {
                    if !previously_live_user_ids.contains(&stream.user_id) {
                        info!(
                            "{} just went live playing {}!",
                            stream.user_name, stream.game_name
                        );
                        // TODO: Send notification here
                    }
                }

                // Detect streams that just went offline (optional logging)
                // for user_id in previously_live_user_ids.difference(&currently_live_user_ids) {
                //    if let Some(user) = monitored_users.iter().find(|u| &u.id == user_id) {
                //        debug!("{} went offline.", user.display_name);
                //    }
                // }

                // Update the previous state for the next check
                previously_live_user_ids = currently_live_user_ids;
            }
            Err(ApiError::Request(e)) if e.is_timeout() => {
                warn!("Twitch API request timed out. Retrying next cycle.");
                // Potentially increase backoff here if it happens repeatedly
            }
            Err(ApiError::TwitchError { status, .. }) if status.is_server_error() => {
                warn!(status = %status, "Twitch API server error. Retrying next cycle.");
                // Potentially increase backoff here
            }
            Err(ApiError::MissingToken) => {
                // Attempt to re-authenticate if token is missing/expired
                warn!("App Access Token missing or invalid. Attempting re-authentication...");
                if let Err(auth_err) = twitch_client.get_app_access_token().await {
                    error!(
                        "Failed to re-authenticate with Twitch: {}. Exiting.",
                        auth_err
                    );
                    // Consider more robust retry logic or backoff for persistent auth failures
                    return Err(auth_err.into());
                }
                info!("Successfully re-authenticated.");
                // Skip the rest of this tick, will check streams on the next one
                continue;
            }
            Err(e) => {
                // For other errors (like JSON parsing, non-server HTTP errors), log and potentially exit
                error!("Unhandled error during stream check: {}. Exiting.", e);
                return Err(e.into());
                // Or decide to just log and continue: warn!(...)
            }
        }
    }
    // Loop is infinite, so Ok(()) is unreachable here, but keep it for function signature
    // Ok(())
}

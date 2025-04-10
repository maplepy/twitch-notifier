mod twitch_api;

use serde::Deserialize;
use thiserror::Error;
use tracing::{error, info, Level};
use tracing_subscriber::FmtSubscriber;

// Import the client and its error type
use crate::twitch_api::{ApiError, TwitchClient};

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
    // Return our Result type
    // Initialize tracing subscriber
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO) // Log INFO level and above
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Setting default tracing subscriber failed");

    // Load settings - propagate error with `?`
    let settings = load_settings()?;
    info!("Configuration loaded successfully!");
    info!(streamers = ?settings.streamers, "Monitoring streamers");
    info!(check_interval = %settings.check_interval_seconds, "Check interval (seconds)");

    // Create Twitch client
    info!("Initializing Twitch client...");
    let mut twitch_client = TwitchClient::new(
        settings.twitch_client_id.clone(), // Clone credentials from settings
        settings.twitch_client_secret.clone(),
    )?;

    // Authenticate with Twitch
    if let Err(e) = twitch_client.get_app_access_token().await {
        error!("Failed to authenticate with Twitch: {}", e);
        // Depending on the error type, could potentially retry later
        return Err(e.into()); // Convert ApiError into our main Error type
    }
    info!("Successfully authenticated with Twitch API.");

    // TODO: Implement main monitoring loop here

    Ok(())
}

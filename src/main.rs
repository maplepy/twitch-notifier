mod notifications;
mod twitch_api;

use serde::Deserialize;
use std::collections::HashSet;
use std::time::Duration;
use thiserror::Error;
use tokio::runtime::Runtime;
use tokio::sync::mpsc;
use tokio::time::interval;
use tracing::{debug, error, info, warn, Level};
use tracing_subscriber::FmtSubscriber;
use tray_item::TrayItem;

// Import the client and its error type
use crate::twitch_api::{ApiError, TwitchClient, User};

// For control messages TO the monitor task
#[derive(Debug)]
enum AppMessage {
    Quit,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub twitch_client_id: String,
    pub twitch_client_secret: String,
    pub streamers: Vec<String>,
    #[serde(default = "default_check_interval")]
    pub check_interval_seconds: u64,
    // TODO: Add systray enable/disable config
}

fn default_check_interval() -> u64 {
    60 // Default to 60 seconds
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Configuration error: {0}")]
    Config(config::ConfigError),

    #[error("I/O error: {0}")]
    Io(std::io::Error),

    #[error("Twitch API error: {0}")]
    Api(twitch_api::ApiError),

    #[error("Tray icon error: {0}")]
    Tray(tray_item::TIError),

    #[error("Task join error: {0}")]
    Join(tokio::task::JoinError),

    #[error("Message channel send error")]
    ChannelSend,

    #[error("GTK initialization failed: {0}")]
    GtkInit(#[from] gtk::glib::BoolError), // Add variant for GTK init error
}

// Implement From traits manually
impl From<config::ConfigError> for Error {
    fn from(err: config::ConfigError) -> Self {
        Error::Config(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<twitch_api::ApiError> for Error {
    fn from(err: twitch_api::ApiError) -> Self {
        Error::Api(err)
    }
}

impl From<tray_item::TIError> for Error {
    fn from(err: tray_item::TIError) -> Self {
        Error::Tray(err)
    }
}

impl From<tokio::task::JoinError> for Error {
    fn from(err: tokio::task::JoinError) -> Self {
        Error::Join(err)
    }
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

// This function contains the core async logic
async fn run_monitor(settings: Settings, mut rx_app: mpsc::Receiver<AppMessage>) -> Result<()> {
    // Create Twitch client
    info!("(Monitor Task) Initializing Twitch client...");
    let mut twitch_client = TwitchClient::new(
        settings.twitch_client_id.clone(),
        settings.twitch_client_secret.clone(),
    )?;

    // Authenticate with Twitch
    twitch_client.get_app_access_token().await?;
    info!("(Monitor Task) Successfully authenticated with Twitch API.");

    // Get initial user data
    let monitored_users: Vec<User> = if settings.streamers.is_empty() {
        info!("(Monitor Task) No streamers configured to monitor.");
        vec![]
    } else {
        info!(
            "(Monitor Task) Fetching user info for: {:?}",
            settings.streamers
        );
        let users = twitch_client
            .get_users_by_login(&settings.streamers)
            .await?;
        if users.is_empty() {
            warn!("(Monitor Task) No Twitch users found for the configured login names.");
        } else {
            info!(
                "(Monitor Task) Successfully fetched info for {} users",
                users.len()
            );
        }
        users
    };

    if monitored_users.is_empty() {
        info!("(Monitor Task) Exiting as there are no valid users to monitor.");
        return Ok(());
    }

    let monitored_user_ids: Vec<String> = monitored_users.iter().map(|u| u.id.clone()).collect();
    let mut previously_live_user_ids: HashSet<String> = HashSet::new();

    info!(
        "(Monitor Task) Starting monitoring loop (checking every {} seconds)",
        settings.check_interval_seconds
    );
    let mut check_interval = interval(Duration::from_secs(settings.check_interval_seconds));

    // Main monitoring loop
    loop {
        tokio::select! {
            _ = check_interval.tick() => {
                debug!("(Monitor Task) Checking stream statuses...");
                match twitch_client.get_streams_by_user_id(&monitored_user_ids).await {
                    Ok(live_streams) => {
                        let currently_live_user_ids: HashSet<String> = live_streams.iter().map(|s| s.user_id.clone()).collect();

                        // Detect streams that just went live
                        for stream in &live_streams {
                            if !previously_live_user_ids.contains(&stream.user_id) {
                                info!(
                                    "{} just went live playing {}!",
                                    stream.user_name, stream.game_name
                                );
                                notifications::send_notification(&stream.user_name, &stream.game_name);
                            }
                        }
                        // Update the previous state for the next check
                        previously_live_user_ids = currently_live_user_ids;
                    }
                    Err(ApiError::Request(e)) if e.is_timeout() => {
                        warn!("(Monitor Task) Twitch API request timed out. Retrying next cycle.");
                    }
                    Err(ApiError::TwitchError { status, .. }) if status.is_server_error() => {
                        warn!(status = %status, "(Monitor Task) Twitch API server error. Retrying next cycle.");
                    }
                    Err(ApiError::MissingToken) => {
                        warn!("(Monitor Task) App Access Token missing or invalid. Attempting re-authentication...");
                        if let Err(auth_err) = twitch_client.get_app_access_token().await {
                            error!("(Monitor Task) Failed to re-authenticate with Twitch: {}. Exiting.", auth_err);
                            return Err(auth_err.into());
                        }
                        info!("(Monitor Task) Successfully re-authenticated.");
                        continue;
                    }
                    Err(e) => {
                        error!("(Monitor Task) Unhandled error during stream check: {}. Exiting.", e);
                        // Optionally send an error state to the tray before exiting
                        // tx_tray.send(TrayUpdate::Error(format!("API Error: {}", e))).ok();
                        return Err(e.into());
                    }
                }
            }
            Some(msg) = rx_app.recv() => {
                info!("(Monitor Task) Received message: {:?}", msg);
                match msg {
                    AppMessage::Quit => {
                        info!("(Monitor Task) Quit message received, shutting down.");
                        break; // Exit the loop
                    }
                }
            }
        }
    }
    Ok(())
}

// Main function now sets up tracing, loads config, spawns the async task,
// and runs the systray loop.
fn main() -> Result<()> {
    // Initialize GTK first on the main thread
    gtk::init()?;

    // Initialize tracing subscriber
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Setting default tracing subscriber failed");

    // Load settings (remains synchronous)
    let settings = load_settings()?;
    info!("Configuration loaded successfully!");

    // Create a Tokio runtime for the async task
    let rt = Runtime::new()?;

    // Create ONLY the app control channel
    let (tx_app, rx_app) = mpsc::channel::<AppMessage>(10);

    // Clone settings needed for the monitor task
    let monitor_settings = settings.clone();

    // Spawn the async monitor task onto the Tokio runtime - pass only rx_app
    let monitor_handle = rt.spawn(async move {
        if let Err(e) = run_monitor(monitor_settings, rx_app).await {
            error!("Monitor task failed: {}", e);
        }
    });

    info!("Starting system tray icon...");
    // Revert TrayItem creation to simple mutable variable
    let mut tray = TrayItem::new(
        "Twitch Notifier",
        tray_item::IconSource::Resource("default-icon"),
    )?;
    tray.add_label("Twitch Notifier")?;

    // Revert Quit callback - no Rc needed
    let quit_tx = tx_app.clone();
    tray.add_menu_item("Quit", move || {
        info!("Quit menu item clicked.");
        if quit_tx.blocking_send(AppMessage::Quit).is_err() {
            error!("Failed to send Quit message to monitor task");
        }
        gtk::main_quit();
    })?;

    info!("System tray started. Running GTK main loop.");
    gtk::main();

    // gtk::main() has returned, meaning gtk::main_quit() was called.
    info!("GTK main loop finished.");

    // Wait for the monitor task to finish.
    info!("Waiting for monitor task to shut down...");
    rt.block_on(monitor_handle)?;
    info!("Monitor task finished. Exiting.");

    Ok(())
}

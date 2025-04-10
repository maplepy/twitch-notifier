mod twitch_api;

use serde::Deserialize;
use thiserror::Error;
use tracing::{error, info, Level};
use tracing_subscriber::FmtSubscriber;

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
}

pub fn load_settings() -> Result<Settings, Error> {
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

fn main() {
    // Initialize tracing subscriber
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO) // Log INFO level and above
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Setting default tracing subscriber failed");

    match load_settings() {
        Ok(settings) => {
            info!("Configuration loaded successfully!");
            info!(client_id = %settings.twitch_client_id, "Twitch Client ID"); // Use structured logging
            info!(streamers = ?settings.streamers, "Monitoring streamers");
            info!(check_interval = %settings.check_interval_seconds, "Check interval (seconds)");
        }
        Err(e) => {
            error!("Failed to load configuration: {}", e);
            std::process::exit(1);
        }
    }
}

use serde::Deserialize;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub twitch_client_id: String,
    pub streamers: Vec<String>,
    #[serde(default = "default_poll_interval")]
    pub poll_interval_seconds: u64,
}

fn default_poll_interval() -> u64 {
    60 // Default to 60 seconds
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Configuration loading error: {0}")]
    Config(#[from] config::ConfigError),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error), // Added for potential future use
}

pub fn load_settings() -> Result<Settings, Error> {
    let config_file_name = "config.toml";

    // Base path is the current working directory
    let base_path = std::env::current_dir().expect("Failed to determine current directory");
    let config_path = base_path.join(config_file_name);

    println!(
        "Attempting to load configuration from: {}",
        config_path.display()
    );

    let settings = config::Config::builder()
        // Add configuration source: config.toml (required)
        .add_source(config::File::with_name(config_path.to_str().unwrap()).required(true))
        // Add environment variable overrides (optional)
        // e.g., `APP_TWITCH_CLIENT_ID=...` would override `twitch_client_id`
        .add_source(config::Environment::with_prefix("APP").separator("__"))
        .build()?;

    // Deserialize the configuration
    let settings: Settings = settings.try_deserialize()?;

    Ok(settings)
}

fn main() {
    match load_settings() {
        Ok(settings) => {
            println!("Configuration loaded successfully!");
            println!("Client ID: {}", settings.twitch_client_id);
            println!("Streamers: {:?}", settings.streamers);
            println!("Poll Interval: {} seconds", settings.poll_interval_seconds);
        }
        Err(e) => {
            eprintln!("Failed to load configuration: {}", e);
            // In a real app, you might want to exit gracefully or retry
            std::process::exit(1);
        }
    }
}

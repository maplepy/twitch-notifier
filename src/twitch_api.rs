#![allow(dead_code)] // TODO: Remove this when structs/errors are used
                     // src/twitch_api.rs

// use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE}; // TODO: Uncomment when used
use serde::Deserialize;
use thiserror::Error;
// use tracing::{debug, trace, warn}; // TODO: Uncomment when used

/// Represents the response for getting an App Access Token.
#[derive(Debug, Deserialize)]
pub struct AppAccessTokenResponse {
    pub access_token: String,
    pub expires_in: u64,
    pub token_type: String,
}

/// Represents a Twitch User object from the API.
#[derive(Debug, Deserialize, Clone)] // Clone needed to easily store user info
pub struct User {
    pub id: String,
    pub login: String,
    pub display_name: String,
}

/// Represents the generic wrapper for Twitch API data arrays.
#[derive(Debug, Deserialize)]
pub struct TwitchDataWrapper<T> {
    pub data: Vec<T>,
    // We might need pagination later, but ignore for MVP
    // pub pagination: Option<Pagination>,
}

/// Represents a live Twitch Stream object from the API.
#[derive(Debug, Deserialize, Clone)] // Clone needed to easily store stream state
pub struct Stream {
    pub id: String,
    pub user_id: String,
    pub user_login: String,
    pub user_name: String,
    pub game_id: String,
    pub game_name: String,
    pub title: String,
    #[serde(rename = "type")] // "type" is a keyword in Rust
    pub stream_type: String, // Should be "live" for online streams
    pub viewer_count: u64,
    pub started_at: String, // Consider parsing this to a DateTime object later
}

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),

    #[error("Failed to parse JSON response: {0}")]
    JsonParse(#[from] serde_json::Error),

    #[error("Twitch API returned an error: status={status}, message={message}")]
    TwitchError {
        status: reqwest::StatusCode,
        message: String, // Can deserialize Twitch error response later if needed
    },

    #[error("App Access Token missing or invalid")]
    MissingToken,

    #[error("User not found: {0}")]
    UserNotFound(String),

    #[error("Stream data not found for user ID: {0}")]
    StreamNotFound(String),
}

const TWITCH_API_BASE_URL: &str = "https://api.twitch.tv/helix";
const TWITCH_AUTH_URL: &str = "https://id.twitch.tv/oauth2/token";

/// Client for interacting with the Twitch API.
#[derive(Debug)]
pub struct TwitchClient {
    client: reqwest::Client,
    client_id: String,
    client_secret: String,
    access_token: Option<String>, // Store the App Access Token
}

impl TwitchClient {
    /// Creates a new Twitch API client.
    pub fn new(client_id: String, client_secret: String) -> Result<Self, ApiError> {
        let client = reqwest::Client::builder()
            // Maybe add user agent later
            .build()?; // Propagate reqwest error

        Ok(Self {
            client,
            client_id,
            client_secret,
            access_token: None,
        })
    }

    // TODO: Method to get/refresh App Access Token
    // TODO: Method to get user IDs from logins
    // TODO: Method to get stream status by user IDs
}

// TODO: Add TwitchClient struct and methods

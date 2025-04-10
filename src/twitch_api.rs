#![allow(dead_code)] // TODO: Remove this when structs/errors are used
                     // src/twitch_api.rs

use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION}; // CONTENT_TYPE commented out
use serde::Deserialize;
use thiserror::Error;
#[allow(unused_imports)] // Allow trace for now
use tracing::{debug, info, trace, warn};

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
            .build()?;

        Ok(Self {
            client,
            client_id,
            client_secret,
            access_token: None,
        })
    }

    /// Fetches or refreshes the App Access Token from Twitch.
    pub async fn get_app_access_token(&mut self) -> Result<(), ApiError> {
        // TODO: Check token expiry before fetching a new one
        info!("Fetching new App Access Token from Twitch");

        let params = [
            ("client_id", &self.client_id),
            ("client_secret", &self.client_secret),
            ("grant_type", &"client_credentials".to_string()), // Use constant later
        ];

        let response = self
            .client
            .post(TWITCH_AUTH_URL)
            .form(&params)
            .send()
            .await?;

        if response.status().is_success() {
            let token_response: AppAccessTokenResponse = response.json().await?;
            debug!(
                "Received new token (expires in {}s)",
                token_response.expires_in
            );
            self.access_token = Some(token_response.access_token);
            Ok(())
        } else {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "<Failed to read error body>".to_string());
            warn!(status = %status, error_body = %error_text, "Failed to get App Access Token");
            Err(ApiError::TwitchError {
                status,
                message: error_text,
            })
        }
    }

    /// Gets Twitch User information for a list of login names.
    pub async fn get_users_by_login(&self, logins: &[String]) -> Result<Vec<User>, ApiError> {
        if logins.is_empty() {
            return Ok(vec![]); // Nothing to fetch
        }

        let token = self.access_token.as_deref().ok_or(ApiError::MissingToken)?;

        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", token))
                .expect("Failed to create auth header value"), // Should not fail
        );
        headers.insert(
            "Client-Id",
            HeaderValue::from_str(&self.client_id)
                .expect("Failed to create client ID header value"),
        );

        // Build the URL with query parameters: ?login=user1&login=user2...
        let url = format!("{}/users", TWITCH_API_BASE_URL);
        let query_params: Vec<(String, String)> = logins
            .iter()
            .map(|login| ("login".to_string(), login.clone()))
            .collect();

        debug!(logins = ?logins, "Fetching user data from Twitch API");

        let response = self
            .client
            .get(&url)
            .headers(headers)
            .query(&query_params)
            .send()
            .await?;

        if response.status().is_success() {
            let user_data: TwitchDataWrapper<User> = response.json().await?;
            debug!("Received data for {} users", user_data.data.len());
            Ok(user_data.data)
        } else {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "<Failed to read error body>".to_string());
            warn!(status = %status, error_body = %error_text, "Failed to get user data");
            Err(ApiError::TwitchError {
                status,
                message: error_text,
            })
        }
    }

    // TODO: Method to get stream status by user IDs
}

// TODO: Add TwitchClient struct and methods

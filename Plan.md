# Twitch Notification App - MVP Plan

This plan outlines the Minimum Viable Product (MVP) features for the Rust-based Twitch notification application. The MVP focuses on core functionality: monitoring specified streamers and sending a notification when they go live, including the game they are playing.

## MVP Features

### 1. Configuration Management

- [x] Define configuration file format (e.g., TOML).
- [x] Store the list of streamer usernames to follow.
- [x] Store Twitch API credentials (Client ID **and Client Secret**).
- [x] Implement logic to load configuration at startup.
- [x] Handle errors for missing or malformed configuration files.

### 2. Twitch API Interaction

- [x] Choose and add HTTP client library (e.g., `reqwest`).
- [x] Choose and add JSON parsing library (e.g., `serde` with `serde_json`).
- [x] Define data structures (`struct`s) for relevant Twitch API responses (e.g., user info, stream info) using `serde`.
- [x] Implement logic to get user IDs from usernames.
- [x] Implement logic to fetch stream status for the followed user IDs.
- [x] Store the previous "live" status of each streamer to detect transitions (offline -> live).
- [x] Handle Twitch API authentication (**Client ID + Client Secret -> App Access Token**).
- [x] Handle potential API errors and rate limiting gracefully.

### 3. Desktop Notifications

- [x] Choose and add a desktop notification library (e.g., `notify-rust`).
- [x] Implement a function to send a notification.
- [x] Notification content: "[StreamerName] is live playing [GameName]!"
- [x] Ensure notifications work correctly on Linux (DBus).

### 4. Core Application Loop & Background Execution

- [x] Set up an asynchronous runtime (e.g., `tokio`).
- [x] Implement the main loop:
  - [x] Load configuration.
  - [x] Periodically (e.g., every 60 seconds) query the Twitch API for streamer statuses.
  - [x] Compare current status with previous status.
  - [x] If a streamer transitioned to live, send a notification.
  - [x] Update the stored previous status.
- [x] Add basic logging (e.g., using `tracing` or `log` crates).
- [ ] Ensure the application can run as a background process (e.g., using `nohup`, `systemd`, or potentially detaching itself).

### 5. Project Setup & Build

- [x] Initialize Rust project (`cargo new twitch_notifier --bin`).
- [x] Add initial dependencies to `Cargo.toml`.
- [x] Set up basic error handling (e.g., using `anyhow` or `thiserror`).
- [x] Create a basic `README.md` with setup and usage instructions.

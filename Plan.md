# Twitch Notification App - MVP Plan

This plan outlines the Minimum Viable Product (MVP) features for the Rust-based Twitch notification application. The MVP focuses on core functionality: monitoring specified streamers and sending a notification when they go live, including the game they are playing.

## MVP Features

### 1. Configuration Management

- [ ] Define configuration file format (e.g., TOML).
- [ ] Store the list of streamer usernames to follow.
- [ ] Store Twitch API credentials (Client ID, potentially an App Access Token or logic to obtain one).
- [ ] Implement logic to load configuration at startup.
- [ ] Handle errors for missing or malformed configuration files.

### 2. Twitch API Interaction

- [ ] Choose and add HTTP client library (e.g., `reqwest`).
- [ ] Choose and add JSON parsing library (e.g., `serde` with `serde_json`).
- [ ] Define data structures (`struct`s) for relevant Twitch API responses (e.g., user info, stream info) using `serde`.
- [ ] Implement logic to get user IDs from usernames.
- [ ] Implement logic to fetch stream status for the followed user IDs.
- [ ] Store the previous "live" status of each streamer to detect transitions (offline -> live).
- [ ] Handle Twitch API authentication (Client ID + App Access Token is simplest for read-only).
- [ ] Handle potential API errors and rate limiting gracefully.

### 3. Desktop Notifications

- [ ] Choose and add a desktop notification library (e.g., `notify-rust`).
- [ ] Implement a function to send a notification.
- [ ] Notification content: "[StreamerName] is live playing [GameName]!"
- [ ] Ensure notifications work correctly on Linux (DBus).

### 4. Core Application Loop & Background Execution

- [ ] Set up an asynchronous runtime (e.g., `tokio`).
- [ ] Implement the main loop:
  - [ ] Load configuration.
  - [ ] Periodically (e.g., every 60 seconds) query the Twitch API for streamer statuses.
  - [ ] Compare current status with previous status.
  - [ ] If a streamer transitioned to live, send a notification.
  - [ ] Update the stored previous status.
- [ ] Add basic logging (e.g., using `tracing` or `log` crates).
- [ ] Ensure the application can run as a background process (e.g., using `nohup`, `systemd`, or potentially detaching itself).

### 5. Project Setup & Build

- [ ] Initialize Rust project (`cargo new twitch_notifier --bin`).
- [ ] Add initial dependencies to `Cargo.toml`.
- [ ] Set up basic error handling (e.g., using `anyhow` or `thiserror`).
- [ ] Create a basic `README.md` with setup and usage instructions.

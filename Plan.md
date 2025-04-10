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

## Future Enhancements (Post-MVP)

### 6. System Tray Icon & Menu (New Priority)

- [ ] Add systray crate dependency (e.g., `tray-item`).
- [ ] Create basic systray icon.
- [ ] Implement logic to show/hide the main window (if we add one later) or perform actions (e.g., force check, quit).
- [ ] Ensure systray runs on its own thread or integrates with the async runtime.
- [ ] Add configuration option to enable/disable systray.

### 7. Game Change Notifications (New Priority)

- [ ] Store the previously seen game ID/name for each live stream.
- [ ] In the main loop, compare the current game with the previous game for live streams.
- [ ] Send a different notification format when a streamer changes games (e.g., "[Streamer] is now playing [New Game]!").
- [ ] Update the stored game state.

### 8. Game Filtering (Blacklist/Whitelist)

- [ ] Add configuration options for game blacklist/whitelist mode.
- [ ] Add configuration option for list of games (either to block or allow).
- [ ] Update configuration loading to include filter settings.
- [ ] Modify notification logic to check game name against the filter list before sending.

### 9. Improved Background Execution / Service

- [ ] Create a systemd service unit file for running the notifier reliably.
- [ ] Add instructions to `README.md` for enabling/starting the systemd service.
- [ ] (Optional) Explore self-daemonization options (less common with systemd).

### 10. Refinements & Other

- [ ] Implement token expiry checking and proactive refresh in `TwitchClient`.
- [ ] Add command-line arguments (e.g., for specifying config file path, log level).
- [ ] Improve error handling resilience (e.g., backoff strategies for API errors).
- [ ] Update `README.md` with advanced configuration and usage.
- [ ] Clean up temporary `#[allow(...)]` attributes.

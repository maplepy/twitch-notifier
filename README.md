# Twitch Notifier

A simple Rust application to send desktop notifications when your favorite Twitch streamers go live.

## Features (MVP)

- Monitors a configurable list of Twitch streamers.
- Sends a desktop notification when a streamer starts streaming.
- Includes the game being played in the notification.

## Setup

1.  **Clone the repository:**
    ```bash
    git clone <repository-url> # Replace with actual URL later
    cd twitch-notifier
    ```
2.  **Configure:**

    - Copy `config.example.toml` to `config.toml` (We will create this file later).
    - Edit `config.toml` and add your Twitch Client ID and the usernames of the streamers you want to follow.

    ```toml
    # config.toml
    twitch_client_id = "YOUR_TWITCH_CLIENT_ID"
    streamers = [
        "streamer1",
        "streamer2",
    ]
    ```

    _Note: You need to register an application on the Twitch Developer portal to get a Client ID._

3.  **Build:**
    ```bash
    cargo build --release
    ```

## Usage

```bash
./target/release/twitch_notifier
```

The application will run in the foreground, periodically checking streamer status. Run it in the background using `nohup` or a process manager like `systemd` for continuous monitoring.

## Development

- **Format code:** `cargo fmt`
- **Lint code:** `cargo clippy -- -D warnings` (Treat warnings as errors)
- **Format other files:** `prettier --write .`
- **Generate changelog:** `git cliff --latest --output CHANGELOG.md` (Run after tagging a release)

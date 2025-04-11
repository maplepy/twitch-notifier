# Twitch Notifier 🚀

A simple Rust application to send desktop notifications 🔔 when your favorite Twitch streamers go live.

## ✨ Features

- Monitors a configurable list of Twitch streamers via the Twitch API 👀.
- Uses `libnotify` to send desktop notifications when a monitored streamer goes live 💻.
- Displays the streamer's name and the game they are currently playing in the notification 🎮.
- Periodically checks streamer status in the background ⏰.
- Configurable polling interval (Future) ⚙️.
- Option to customize notification appearance (Future) 🎨.

## 🚀 Setup

### 🔧 Prerequisites

1. **Rust Toolchain:** Install Rust using `rustup`:

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **System Dependencies:** Install required libraries for desktop notifications.

   - **Arch Linux:**
     ```bash
     yay -S libdbusmenu-glib pkgconf
     ```
   - **Debian/Ubuntu:**
     ```bash
     sudo apt update && sudo apt install libdbus-1-dev pkg-config
     ```

3. **Development Tools (Optional but Recommended):**
   - **pre-commit:** For running checks before commits.
     ```bash
     yay -S pre-commit # Arch
     # or: pip install pre-commit
     ```
   - **git-cliff:** For generating the changelog.
     ```bash
     yay -S git-cliff # Arch
     # or: cargo install git-cliff
     ```

### 📦 Installation

1. **Clone the repository:**

   ```bash
   git clone https://github.com/maplepy/twitch-notifier
   cd twitch-notifier
   ```

2. **Install pre-commit hooks (Optional):**

   ```bash
   pre-commit install
   ```

3. **Configure the application:**

   - Copy the example configuration: `cp config.example.toml config.toml`
   - Edit `config.toml`: Add your Twitch Client ID and Client Secret (obtained from the [Twitch Developer portal](https://dev.twitch.tv/console/apps)). It's recommended to name your application "twitch-notifier" when registering. Add the list of streamer usernames you want to monitor.
   - **🚨 IMPORTANT:** Never commit `config.toml` to version control!

   ```toml
   # config.toml
   twitch_client_id = "YOUR_TWITCH_CLIENT_ID"
   twitch_client_secret = "YOUR_TWITCH_CLIENT_SECRET"
   streamers = [
       "streamer_username1",
       "streamer_username2"
   ]
   check_interval_seconds = 60  # Optional, defaults to 60
   ```

4. **Build the application:**
   ```bash
   cargo build --release
   ```

## ▶️ Usage

```bash
./target/release/twitch_notifier
```

The application will run in the foreground, periodically checking streamer status. Run it in the background using `nohup` or a process manager like `systemd` for continuous monitoring.

## 🛠️ Development

- **Format code:** `cargo fmt`
- **Lint code:** `cargo clippy -- -D warnings` (Treat warnings as errors)
- **Format other files:** `prettier --write .`
- **Run checks before commit:** Uses `pre-commit` hooks (automatically installed via `pre-commit install`).

## 🔖 Releasing

This project uses [Conventional Commits](https://www.conventionalcommits.org/) for commit messages and [Semantic Versioning](https://semver.org/) for versioning.

1.  Determine the next version number (e.g., `v0.1.0`).
2.  Update `CHANGELOG.md` using `git-cliff`:
    ```bash
    git cliff --tag vX.Y.Z # Replace vX.Y.Z with the new version
    ```
3.  Review the generated `CHANGELOG.md`.
4.  Commit the changelog:
    ```bash
    git add CHANGELOG.md
    git commit -m "docs: Update CHANGELOG.md for vX.Y.Z"
    ```
5.  Tag the release:
    ```bash
    git tag vX.Y.Z
    ```
6.  Push the commit and tag:
    ```bash
    git push
    git push --tags
    ```

## 📜 Generating Changelog (Manual)

To generate the changelog manually (e.g., to see unreleased changes):

```bash
git cliff --unreleased --output CHANGELOG.md
# Or to just view it without writing:
git cliff --unreleased
```

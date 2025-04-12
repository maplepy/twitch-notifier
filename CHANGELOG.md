# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2025-04-12

### Added

- Game Change Notifications: Send a notification when a monitored streamer changes the game they are playing.
- Stream Title in Notifications: Include the stream's title in the notification body for both "live" and "game change" events.

### Changed

- Notification format for "live" events to accommodate the stream title.
- Notification format for "game change" events.

### Fixed

- Removed unused `HashSet` import in `main.rs`.

### Features

- _(systray)_ Add systray icon and quit functionality (5f6480f)
- Add game change notifications (eade6bd)
- Include stream title in notifications (38a4aff)

### Documentation

- _(plan)_ Mark game change notifications as complete (cd09e00)
- _(plan)_ Update systray task checklist (d0de28e)
- update Features and Setup instructions in README (64e37bb)

## [0.1.0] - 2025-04-11

### Added

- Initial release.
- Configuration loading from `config.toml` (Twitch credentials, streamer list, check interval).
- Periodic checking of Twitch streams using the Twitch API (App Access Token auth).
- Desktop notifications when a monitored streamer goes live, showing streamer name and game name.
- System tray icon with a "Quit" option.
- Basic logging using `tracing`.
- Error handling using `thiserror`.

### Features

- _(twitch_api)_ Implement stream fetching by user ID (26b2ac8)
- _(core)_ Implement main monitoring loop structure (e564da2)
- _(notify)_ Implement desktop notifications (bd796c6)
- _(twitch_api)_ Implement user fetching by login (491c981)
- _(twitch_api)_ Implement app access token fetching (c4ce635)
- _(twitch_api)_ Add client structure and config secret (dea894f)

### Bug Fixes

- _(cliff)_ Use commit ID only in template, remove deprecated remote URL (3c36fc2)

### Refactor

- _(notify)_ Move notification logic to separate module (fd684ed)

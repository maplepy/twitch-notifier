# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-04-11

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

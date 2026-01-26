# Changelog

All notable changes to this project will be documented in this file.

## [0.2.0] - 2025-01-26

### Added
- Initial Node.js release using NAPI-RS
- Native Node.js bindings for nostr-arena core
- Full async/await support with tokio runtime
- TypeScript type definitions

### Features
- `Arena` class with constructor and `init()` method
- `arena.connect()` / `arena.disconnect()` - Relay management
- `arena.create()` / `arena.join()` / `arena.leave()` - Room management
- `arena.reconnect()` - Session recovery
- `arena.sendState()` / `arena.sendReady()` / `arena.startGame()` - Game control
- `arena.tryRecv()` - Non-blocking event polling
- `arena.getRoomQrSvg()` / `arena.getRoomQrDataUrl()` - QR code generation
- `listRooms()` - Room discovery

### Dependencies
- nostr-arena (Rust core via git)
- napi-rs 2.2
- tokio (multi-thread runtime)

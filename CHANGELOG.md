# Changelog

All notable changes to this project will be documented in this file.

## [0.2.0] - 2025-01-26

### Added
- Initial WASM release
- WebAssembly bindings for nostr-arena core
- All Arena methods exposed via wasm-bindgen
- Event types mapped to JavaScript objects
- TypeScript-friendly API with camelCase naming

### Features
- `Arena.init(config)` - Create arena instance
- `arena.connect()` / `arena.disconnect()` - Relay management
- `arena.create()` / `arena.join()` / `arena.leave()` - Room management
- `arena.reconnect()` - Session recovery
- `arena.sendState()` / `arena.sendReady()` / `arena.startGame()` - Game control
- `arena.tryRecv()` - Non-blocking event polling
- `arena.getRoomQRSvg()` / `arena.getRoomQRDataUrl()` - QR code generation
- `Arena.listRooms()` - Room discovery

### Dependencies
- nostr-arena (Rust core via git)
- wasm-bindgen
- serde-wasm-bindgen

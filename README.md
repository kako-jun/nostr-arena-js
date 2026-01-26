# nostr-arena-js

WebAssembly bindings for [nostr-arena](https://github.com/kako-jun/nostr-arena).

Nostr-based real-time multiplayer game arena for JavaScript/TypeScript.

## Installation

```bash
npm install nostr-arena
```

## Quick Start

```typescript
import init, { Arena, ArenaConfig } from 'nostr-arena';

await init();

const config = new ArenaConfig('my-game')
    .setMaxPlayers(4)
    .setStartMode('ready');

const arena = await Arena.init(config);
await arena.connect();

// Create a room
const url = await arena.create();
console.log('Share this URL:', url);

// Poll for events
setInterval(async () => {
    const event = await arena.tryRecv();
    if (event) {
        switch (event.type) {
            case 'playerJoin':
                console.log('Player joined:', event.player.pubkey);
                break;
            case 'gameStart':
                console.log('Game started!');
                break;
        }
    }
}, 100);

// Send your state
await arena.sendState({ score: 100, x: 50, y: 50 });
```

## API

See [nostr-arena documentation](https://github.com/kako-jun/nostr-arena) for full API reference.

## Building

```bash
wasm-pack build --target web
```

## License

MIT

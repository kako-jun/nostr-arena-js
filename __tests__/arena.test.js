const { Arena, ArenaConfig, StartMode } = require('../index.js');

describe('ArenaConfig', () => {
  test('creates config with game id', () => {
    const config = new ArenaConfig('test-game');
    expect(config).toBeDefined();
  });

  test('builder pattern works', () => {
    const config = new ArenaConfig('test-game')
      .relays(['wss://relay.damus.io'])
      .maxPlayers(4)
      .startMode(StartMode.Ready)
      .countdownSeconds(5);
    expect(config).toBeDefined();
  });
});

describe('Arena', () => {
  test('creates arena with config', async () => {
    const config = new ArenaConfig('test-game')
      .relays(['wss://relay.damus.io']);
    const arena = await Arena.create(config);
    expect(arena).toBeDefined();
    expect(typeof arena.publicKey()).toBe('string');
    expect(arena.publicKey()).toHaveLength(64);
  });
});

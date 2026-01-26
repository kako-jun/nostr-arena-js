//! Node.js bindings for nostr-arena

use napi::bindgen_prelude::*;
use napi_derive::napi;
use nostr_arena::{
    Arena as CoreArena, ArenaConfig as CoreConfig, ArenaEvent as CoreEvent, PlayerPresence,
    RoomInfo, RoomStatus, StartMode,
};
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Arena configuration
#[napi(object)]
pub struct JsArenaConfig {
    /// Game identifier
    pub game_id: String,

    /// Relay URLs
    pub relays: Option<Vec<String>>,

    /// Room expiry in milliseconds (0 = no expiry)
    pub room_expiry: Option<u32>,

    /// Maximum number of players
    pub max_players: Option<u32>,

    /// Start mode: "auto", "ready", "countdown", "host"
    pub start_mode: Option<String>,

    /// Countdown seconds (for countdown mode)
    pub countdown_seconds: Option<u32>,

    /// Base URL for room links
    pub base_url: Option<String>,
}

/// Player presence information
#[napi(object)]
pub struct JsPlayerPresence {
    /// Player's public key
    pub pubkey: String,
    /// When player joined (unix timestamp ms)
    pub joined_at: i64,
    /// Last heartbeat (unix timestamp ms)
    pub last_seen: i64,
    /// Whether player is ready
    pub ready: bool,
}

/// Room information
#[napi(object)]
pub struct JsRoomInfo {
    /// Room ID
    pub room_id: String,
    /// Game ID
    pub game_id: String,
    /// Room status
    pub status: String,
    /// Host's public key
    pub host_pubkey: String,
    /// Current player count
    pub player_count: u32,
    /// Maximum players
    pub max_players: u32,
    /// Creation time (unix timestamp ms)
    pub created_at: i64,
    /// Expiry time (unix timestamp ms), null if no expiry
    pub expires_at: Option<i64>,
    /// Random seed
    pub seed: i64,
}

/// Arena event
#[napi(object)]
pub struct JsArenaEvent {
    /// Event type
    pub event_type: String,
    /// Player's public key (for player events)
    pub pubkey: Option<String>,
    /// Player presence (for join events)
    pub player: Option<JsPlayerPresence>,
    /// Game state (for state events)
    pub state: Option<serde_json::Value>,
    /// Reason (for game over events)
    pub reason: Option<String>,
    /// Final score (for game over events)
    pub final_score: Option<i64>,
    /// Seed (for rematch start)
    pub seed: Option<i64>,
    /// Seconds (for countdown)
    pub seconds: Option<u32>,
    /// Remaining seconds (for countdown tick)
    pub remaining: Option<u32>,
    /// Error message
    pub message: Option<String>,
}

/// Arena - Nostr-based multiplayer game matchmaking
#[napi]
pub struct Arena {
    inner: Arc<Mutex<Option<CoreArena<serde_json::Value>>>>,
    config: CoreConfig,
}

#[napi]
impl Arena {
    /// Create a new Arena instance
    #[napi(constructor)]
    pub fn new(config: JsArenaConfig) -> Result<Self> {
        let mut core_config = CoreConfig::new(&config.game_id);

        if let Some(relays) = config.relays {
            core_config = core_config.relays(relays);
        }
        if let Some(expiry) = config.room_expiry {
            core_config = core_config.room_expiry(expiry as u64);
        }
        if let Some(max) = config.max_players {
            core_config = core_config.max_players(max as usize);
        }
        if let Some(mode) = config.start_mode {
            let mode = match mode.as_str() {
                "auto" => StartMode::Auto,
                "ready" => StartMode::Ready,
                "countdown" => StartMode::Countdown,
                "host" => StartMode::Host,
                _ => StartMode::Auto,
            };
            core_config = core_config.start_mode(mode);
        }
        if let Some(secs) = config.countdown_seconds {
            core_config = core_config.countdown_seconds(secs);
        }
        if let Some(url) = config.base_url {
            core_config = core_config.base_url(&url);
        }

        Ok(Self {
            inner: Arc::new(Mutex::new(None)),
            config: core_config,
        })
    }

    /// Initialize the arena (must be called before other methods)
    #[napi]
    pub async fn init(&self) -> Result<()> {
        let arena = CoreArena::new(self.config.clone())
            .await
            .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;

        let mut guard = self.inner.lock().await;
        *guard = Some(arena);
        Ok(())
    }

    /// Get public key
    #[napi]
    pub async fn public_key(&self) -> Result<String> {
        let guard = self.inner.lock().await;
        let arena = guard
            .as_ref()
            .ok_or_else(|| Error::new(Status::GenericFailure, "Arena not initialized"))?;
        Ok(arena.public_key())
    }

    /// Connect to relays
    #[napi]
    pub async fn connect(&self) -> Result<()> {
        let guard = self.inner.lock().await;
        let arena = guard
            .as_ref()
            .ok_or_else(|| Error::new(Status::GenericFailure, "Arena not initialized"))?;
        arena
            .connect()
            .await
            .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))
    }

    /// Disconnect from relays
    #[napi]
    pub async fn disconnect(&self) -> Result<()> {
        let guard = self.inner.lock().await;
        let arena = guard
            .as_ref()
            .ok_or_else(|| Error::new(Status::GenericFailure, "Arena not initialized"))?;
        arena
            .disconnect()
            .await
            .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))
    }

    /// Check if connected
    #[napi]
    pub async fn is_connected(&self) -> Result<bool> {
        let guard = self.inner.lock().await;
        let arena = guard
            .as_ref()
            .ok_or_else(|| Error::new(Status::GenericFailure, "Arena not initialized"))?;
        Ok(arena.is_connected().await)
    }

    /// Create a new room
    #[napi]
    pub async fn create(&self) -> Result<String> {
        let guard = self.inner.lock().await;
        let arena = guard
            .as_ref()
            .ok_or_else(|| Error::new(Status::GenericFailure, "Arena not initialized"))?;
        arena
            .create()
            .await
            .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))
    }

    /// Join an existing room
    #[napi]
    pub async fn join(&self, room_id: String) -> Result<()> {
        let guard = self.inner.lock().await;
        let arena = guard
            .as_ref()
            .ok_or_else(|| Error::new(Status::GenericFailure, "Arena not initialized"))?;
        arena
            .join(&room_id)
            .await
            .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))
    }

    /// Leave the current room
    #[napi]
    pub async fn leave(&self) -> Result<()> {
        let guard = self.inner.lock().await;
        let arena = guard
            .as_ref()
            .ok_or_else(|| Error::new(Status::GenericFailure, "Arena not initialized"))?;
        arena
            .leave()
            .await
            .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))
    }

    /// Delete the room (host only)
    #[napi]
    pub async fn delete_room(&self) -> Result<()> {
        let guard = self.inner.lock().await;
        let arena = guard
            .as_ref()
            .ok_or_else(|| Error::new(Status::GenericFailure, "Arena not initialized"))?;
        arena
            .delete_room()
            .await
            .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))
    }

    /// Reconnect to a room
    #[napi]
    pub async fn reconnect(&self, room_id: String) -> Result<()> {
        let guard = self.inner.lock().await;
        let arena = guard
            .as_ref()
            .ok_or_else(|| Error::new(Status::GenericFailure, "Arena not initialized"))?;
        arena
            .reconnect(&room_id)
            .await
            .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))
    }

    /// Send game state
    #[napi]
    pub async fn send_state(&self, state: serde_json::Value) -> Result<()> {
        let guard = self.inner.lock().await;
        let arena = guard
            .as_ref()
            .ok_or_else(|| Error::new(Status::GenericFailure, "Arena not initialized"))?;
        arena
            .send_state(&state)
            .await
            .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))
    }

    /// Send game over
    #[napi]
    pub async fn send_game_over(&self, reason: String, final_score: Option<i64>) -> Result<()> {
        let guard = self.inner.lock().await;
        let arena = guard
            .as_ref()
            .ok_or_else(|| Error::new(Status::GenericFailure, "Arena not initialized"))?;
        arena
            .send_game_over(&reason, final_score)
            .await
            .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))
    }

    /// Request rematch
    #[napi]
    pub async fn request_rematch(&self) -> Result<()> {
        let guard = self.inner.lock().await;
        let arena = guard
            .as_ref()
            .ok_or_else(|| Error::new(Status::GenericFailure, "Arena not initialized"))?;
        arena
            .request_rematch()
            .await
            .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))
    }

    /// Accept rematch
    #[napi]
    pub async fn accept_rematch(&self) -> Result<()> {
        let guard = self.inner.lock().await;
        let arena = guard
            .as_ref()
            .ok_or_else(|| Error::new(Status::GenericFailure, "Arena not initialized"))?;
        arena
            .accept_rematch()
            .await
            .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))
    }

    /// Send ready signal
    #[napi]
    pub async fn send_ready(&self, ready: bool) -> Result<()> {
        let guard = self.inner.lock().await;
        let arena = guard
            .as_ref()
            .ok_or_else(|| Error::new(Status::GenericFailure, "Arena not initialized"))?;
        arena
            .send_ready(ready)
            .await
            .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))
    }

    /// Start game (host only)
    #[napi]
    pub async fn start_game(&self) -> Result<()> {
        let guard = self.inner.lock().await;
        let arena = guard
            .as_ref()
            .ok_or_else(|| Error::new(Status::GenericFailure, "Arena not initialized"))?;
        arena
            .start_game()
            .await
            .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))
    }

    /// Get room URL
    #[napi]
    pub async fn get_room_url(&self) -> Result<Option<String>> {
        let guard = self.inner.lock().await;
        let arena = guard
            .as_ref()
            .ok_or_else(|| Error::new(Status::GenericFailure, "Arena not initialized"))?;
        Ok(arena.get_room_url().await)
    }

    /// Get room QR code as SVG
    #[napi]
    pub async fn get_room_qr_svg(&self) -> Result<Option<String>> {
        let guard = self.inner.lock().await;
        let arena = guard
            .as_ref()
            .ok_or_else(|| Error::new(Status::GenericFailure, "Arena not initialized"))?;
        Ok(arena.get_room_qr_svg(None).await)
    }

    /// Get room QR code as data URL
    #[napi]
    pub async fn get_room_qr_data_url(&self) -> Result<Option<String>> {
        let guard = self.inner.lock().await;
        let arena = guard
            .as_ref()
            .ok_or_else(|| Error::new(Status::GenericFailure, "Arena not initialized"))?;
        Ok(arena.get_room_qr_data_url(None).await)
    }

    /// Get current players
    #[napi]
    pub async fn players(&self) -> Result<Vec<JsPlayerPresence>> {
        let guard = self.inner.lock().await;
        let arena = guard
            .as_ref()
            .ok_or_else(|| Error::new(Status::GenericFailure, "Arena not initialized"))?;
        let players = arena.players().await;
        Ok(players
            .into_iter()
            .map(|p| JsPlayerPresence {
                pubkey: p.pubkey,
                joined_at: p.joined_at as i64,
                last_seen: p.last_seen as i64,
                ready: p.ready,
            })
            .collect())
    }

    /// Get player count
    #[napi]
    pub async fn player_count(&self) -> Result<u32> {
        let guard = self.inner.lock().await;
        let arena = guard
            .as_ref()
            .ok_or_else(|| Error::new(Status::GenericFailure, "Arena not initialized"))?;
        Ok(arena.player_count().await as u32)
    }

    /// Poll for next event (non-blocking)
    #[napi]
    pub async fn try_recv(&self) -> Result<Option<JsArenaEvent>> {
        let guard = self.inner.lock().await;
        let arena = guard
            .as_ref()
            .ok_or_else(|| Error::new(Status::GenericFailure, "Arena not initialized"))?;

        match arena.try_recv().await {
            Some(event) => Ok(Some(convert_event(event))),
            None => Ok(None),
        }
    }
}

/// List available rooms
#[napi]
pub async fn list_rooms(
    game_id: String,
    relays: Vec<String>,
    status: Option<String>,
    limit: u32,
) -> Result<Vec<JsRoomInfo>> {
    let status_filter = status.as_deref().map(|s| match s {
        "waiting" => RoomStatus::Waiting,
        "playing" => RoomStatus::Playing,
        "finished" => RoomStatus::Finished,
        _ => RoomStatus::Waiting,
    });

    let rooms =
        CoreArena::<serde_json::Value>::list_rooms(&game_id, relays, status_filter, limit as usize)
            .await
            .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;

    Ok(rooms
        .into_iter()
        .map(|r| JsRoomInfo {
            room_id: r.room_id,
            game_id: r.game_id,
            status: format!("{:?}", r.status).to_lowercase(),
            host_pubkey: r.host_pubkey,
            player_count: r.player_count as u32,
            max_players: r.max_players as u32,
            created_at: r.created_at as i64,
            expires_at: r.expires_at.map(|e| e as i64),
            seed: r.seed as i64,
        })
        .collect())
}

fn convert_event(event: CoreEvent<serde_json::Value>) -> JsArenaEvent {
    match event {
        CoreEvent::PlayerJoin(player) => JsArenaEvent {
            event_type: "playerJoin".to_string(),
            pubkey: Some(player.pubkey.clone()),
            player: Some(JsPlayerPresence {
                pubkey: player.pubkey,
                joined_at: player.joined_at as i64,
                last_seen: player.last_seen as i64,
                ready: player.ready,
            }),
            state: None,
            reason: None,
            final_score: None,
            seed: None,
            seconds: None,
            remaining: None,
            message: None,
        },
        CoreEvent::PlayerLeave(pubkey) => JsArenaEvent {
            event_type: "playerLeave".to_string(),
            pubkey: Some(pubkey),
            player: None,
            state: None,
            reason: None,
            final_score: None,
            seed: None,
            seconds: None,
            remaining: None,
            message: None,
        },
        CoreEvent::PlayerState { pubkey, state } => JsArenaEvent {
            event_type: "playerState".to_string(),
            pubkey: Some(pubkey),
            player: None,
            state: Some(state),
            reason: None,
            final_score: None,
            seed: None,
            seconds: None,
            remaining: None,
            message: None,
        },
        CoreEvent::PlayerDisconnect(pubkey) => JsArenaEvent {
            event_type: "playerDisconnect".to_string(),
            pubkey: Some(pubkey),
            player: None,
            state: None,
            reason: None,
            final_score: None,
            seed: None,
            seconds: None,
            remaining: None,
            message: None,
        },
        CoreEvent::PlayerGameOver {
            pubkey,
            reason,
            final_score,
        } => JsArenaEvent {
            event_type: "playerGameOver".to_string(),
            pubkey: Some(pubkey),
            player: None,
            state: None,
            reason: Some(reason),
            final_score,
            seed: None,
            seconds: None,
            remaining: None,
            message: None,
        },
        CoreEvent::RematchRequested(pubkey) => JsArenaEvent {
            event_type: "rematchRequested".to_string(),
            pubkey: Some(pubkey),
            player: None,
            state: None,
            reason: None,
            final_score: None,
            seed: None,
            seconds: None,
            remaining: None,
            message: None,
        },
        CoreEvent::RematchStart(seed) => JsArenaEvent {
            event_type: "rematchStart".to_string(),
            pubkey: None,
            player: None,
            state: None,
            reason: None,
            final_score: None,
            seed: Some(seed as i64),
            seconds: None,
            remaining: None,
            message: None,
        },
        CoreEvent::AllReady => JsArenaEvent {
            event_type: "allReady".to_string(),
            pubkey: None,
            player: None,
            state: None,
            reason: None,
            final_score: None,
            seed: None,
            seconds: None,
            remaining: None,
            message: None,
        },
        CoreEvent::CountdownStart(seconds) => JsArenaEvent {
            event_type: "countdownStart".to_string(),
            pubkey: None,
            player: None,
            state: None,
            reason: None,
            final_score: None,
            seed: None,
            seconds: Some(seconds),
            remaining: None,
            message: None,
        },
        CoreEvent::CountdownTick(remaining) => JsArenaEvent {
            event_type: "countdownTick".to_string(),
            pubkey: None,
            player: None,
            state: None,
            reason: None,
            final_score: None,
            seed: None,
            seconds: None,
            remaining: Some(remaining),
            message: None,
        },
        CoreEvent::GameStart => JsArenaEvent {
            event_type: "gameStart".to_string(),
            pubkey: None,
            player: None,
            state: None,
            reason: None,
            final_score: None,
            seed: None,
            seconds: None,
            remaining: None,
            message: None,
        },
        CoreEvent::Error(message) => JsArenaEvent {
            event_type: "error".to_string(),
            pubkey: None,
            player: None,
            state: None,
            reason: None,
            final_score: None,
            seed: None,
            seconds: None,
            remaining: None,
            message: Some(message),
        },
    }
}

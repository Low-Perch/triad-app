use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

use triad_core::models::GameState;

const STORE_PATH: &str = ".settings.dat";
const GAME_KEY: &str = "game";

pub fn load_game(app: &AppHandle) -> Option<GameState> {
    let store = app.store(STORE_PATH).ok()?;
    let value = store.get(GAME_KEY)?;
    serde_json::from_value(value).ok()
}

pub fn save_game(app: &AppHandle, state: &GameState) -> Result<(), String> {
    let store = app
        .store(STORE_PATH)
        .map_err(|e| format!("Failed to open store: {}", e))?;

    let value =
        serde_json::to_value(state).map_err(|e| format!("Failed to serialize state: {}", e))?;

    store
        .set(GAME_KEY, value);

    store
        .save()
        .map_err(|e| format!("Failed to save store: {}", e))?;

    Ok(())
}

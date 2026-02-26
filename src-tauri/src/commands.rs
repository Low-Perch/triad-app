use std::sync::Mutex;
use tauri::State;

use triad_core::engine;
use triad_core::models::*;

use crate::persistence;

pub type AppState = Mutex<GameState>;

#[tauri::command]
pub fn init_game(
    state: State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<GameState, String> {
    let mut game_state = state.lock().map_err(|e| e.to_string())?;

    let saved = persistence::load_game(&app);
    let now_secs = triad_core::generator::now_unix_secs();
    let result = engine::init_game(saved, now_secs);
    *game_state = result.clone();
    persistence::save_game(&app, &game_state)?;

    Ok(result)
}

#[tauri::command]
pub fn add_key(
    key: String,
    state: State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<Input, String> {
    let mut game_state = state.lock().map_err(|e| e.to_string())?;

    let input = engine::add_key(&mut game_state, &key);
    persistence::save_game(&app, &game_state)?;

    Ok(input)
}

#[tauri::command]
pub fn remove_key(
    state: State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<Input, String> {
    let mut game_state = state.lock().map_err(|e| e.to_string())?;

    let input = engine::remove_key(&mut game_state);
    persistence::save_game(&app, &game_state)?;

    Ok(input)
}

#[tauri::command]
pub fn submit_solution(
    state: State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<SubmitResult, String> {
    let mut game_state = state.lock().map_err(|e| e.to_string())?;

    let result = engine::submit_solution(&mut game_state);
    persistence::save_game(&app, &game_state)?;

    Ok(result)
}

#[tauri::command]
pub fn activate_clue(
    clue_id: String,
    state: State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<ClueResult, String> {
    let mut game_state = state.lock().map_err(|e| e.to_string())?;

    let result = engine::activate_clue(&mut game_state, &clue_id);
    persistence::save_game(&app, &game_state)?;

    Ok(result)
}

#[tauri::command]
pub fn save_game(
    state: State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    let game_state = state.lock().map_err(|e| e.to_string())?;
    persistence::save_game(&app, &game_state)
}

#[tauri::command]
pub fn new_game(
    state: State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<GameState, String> {
    let mut game_state = state.lock().map_err(|e| e.to_string())?;

    let result = engine::new_game(&mut game_state);
    persistence::save_game(&app, &game_state)?;

    Ok(result)
}

#[tauri::command]
pub fn clear_input(
    state: State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<Input, String> {
    let mut game_state = state.lock().map_err(|e| e.to_string())?;

    let input = engine::clear_input(&mut game_state);
    persistence::save_game(&app, &game_state)?;

    Ok(input)
}

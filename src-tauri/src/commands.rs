use std::sync::Mutex;
use tauri::State;

use crate::game;
use crate::models::*;
use crate::persistence;

pub type AppState = Mutex<GameState>;

#[tauri::command]
pub fn init_game(
    state: State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<GameState, String> {
    let mut game_state = state.lock().map_err(|e| e.to_string())?;

    if let Some(saved) = persistence::load_game(&app) {
        *game_state = saved;

        if game_state.puzzle.solved {
            return Ok(game_state.clone());
        }
    } else {
        game::initialize_with_generated_puzzle(&mut game_state);
    }

    game::record_puzzle_played(&mut game_state);
    game::start_puzzle_timer(&mut game_state);
    persistence::save_game(&app, &game_state)?;

    Ok(game_state.clone())
}

#[tauri::command]
pub fn add_key(
    key: String,
    state: State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<Input, String> {
    let mut game_state = state.lock().map_err(|e| e.to_string())?;

    game::update_input_state(&mut game_state, InputState::Edit);
    game::add_key(&mut game_state, &key);
    persistence::save_game(&app, &game_state)?;

    Ok(game_state.input.clone())
}

#[tauri::command]
pub fn remove_key(
    state: State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<Input, String> {
    let mut game_state = state.lock().map_err(|e| e.to_string())?;

    game::update_input_state(&mut game_state, InputState::Edit);
    game::remove_key(&mut game_state);
    persistence::save_game(&app, &game_state)?;

    Ok(game_state.input.clone())
}

#[tauri::command]
pub fn submit_solution(
    state: State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<SubmitResult, String> {
    let mut game_state = state.lock().map_err(|e| e.to_string())?;

    let solved = game::submit_solution(&mut game_state);
    persistence::save_game(&app, &game_state)?;

    Ok(SubmitResult {
        solved,
        input_state: game_state.input.state.clone(),
        puzzle_state: game_state.puzzle.state.clone(),
        stats: game_state.stats.clone(),
    })
}

#[tauri::command]
pub fn activate_clue(
    clue_id: String,
    state: State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<ClueResult, String> {
    let mut game_state = state.lock().map_err(|e| e.to_string())?;

    game::activate_clue(&mut game_state, &clue_id);
    game::apply_clue_effects(&mut game_state, &clue_id);
    persistence::save_game(&app, &game_state)?;

    Ok(ClueResult {
        clues: game_state.clues.clone(),
        input: game_state.input.clone(),
        puzzle: game_state.puzzle.clone(),
        keys: game_state.keys.clone(),
    })
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

    game::new_game(&mut game_state);
    game::record_puzzle_played(&mut game_state);
    game::start_puzzle_timer(&mut game_state);
    persistence::save_game(&app, &game_state)?;

    Ok(game_state.clone())
}

#[tauri::command]
pub fn clear_input(
    state: State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<Input, String> {
    let mut game_state = state.lock().map_err(|e| e.to_string())?;

    game::clear_input(&mut game_state);
    persistence::save_game(&app, &game_state)?;

    Ok(game_state.input.clone())
}

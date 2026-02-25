use std::sync::Mutex;
use tauri::State;

use crate::game;
use crate::generator;
use crate::models::*;
use crate::persistence;

pub type AppState = Mutex<GameState>;

#[tauri::command]
pub fn init_game(
    state: State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<GameState, String> {
    let mut game_state = state.lock().map_err(|e| e.to_string())?;

    let today = generator::today_as_date_string();
    let puzzle_number = generator::today_as_days_since_epoch();

    if let Some(saved) = persistence::load_game(&app) {
        *game_state = saved;

        let is_todays_daily = game_state.puzzle_date.as_deref() == Some(today.as_str());

        if is_todays_daily {
            if game_state.puzzle.solved {
                return Ok(game_state.clone());
            }
            // Resume mid-solve on today's daily
            game::record_puzzle_played(&mut game_state);
            persistence::save_game(&app, &game_state)?;
            return Ok(game_state.clone());
        }

        // Day rolled over or legacy data — reset streak if previous was unsolved
        if !game_state.puzzle.solved {
            game_state.stats.current_streak = 0;
        }
    }

    // Generate today's daily puzzle
    game::initialize_with_daily_puzzle(&mut game_state, puzzle_number);
    game_state.puzzle_date = Some(today);
    game::record_puzzle_played(&mut game_state);
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

    let (solved, exhausted) = game::submit_solution(&mut game_state);
    persistence::save_game(&app, &game_state)?;

    Ok(SubmitResult {
        solved,
        exhausted,
        guesses: game_state.guesses,
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

    let stats = if clue_id == "solve" {
        Some(game_state.stats.clone())
    } else {
        None
    };

    Ok(ClueResult {
        clues: game_state.clues.clone(),
        input: game_state.input.clone(),
        puzzle: game_state.puzzle.clone(),
        keys: game_state.keys.clone(),
        stats,
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

use crate::game;
use crate::generator;
use crate::models::*;

/// Initialize or resume a game. Returns the game state to render.
/// `saved` is the previously persisted state (if any).
/// `now_secs` is the current Unix timestamp in seconds.
pub fn init_game(saved: Option<GameState>, now_secs: u64) -> GameState {
    let today = generator::date_string_from_secs(now_secs);
    let puzzle_number = generator::days_since_epoch(now_secs);

    if let Some(mut state) = saved {
        let is_todays_daily = state.puzzle_date.as_deref() == Some(today.as_str());

        if is_todays_daily {
            if !state.puzzle.solved {
                game::record_puzzle_played(&mut state);
            }
            return state;
        }

        // Day rolled over — reset streak if previous was unsolved
        if !state.puzzle.solved {
            state.stats.current_streak = 0;
        }

        // Generate today's daily puzzle, preserving stats
        game::initialize_with_daily_puzzle(&mut state, puzzle_number);
        state.puzzle_date = Some(today);
        game::record_puzzle_played(&mut state);
        return state;
    }

    // No saved state — fresh start
    let mut state = GameState::default();
    game::initialize_with_daily_puzzle(&mut state, puzzle_number);
    state.puzzle_date = Some(today);
    game::record_puzzle_played(&mut state);
    state
}

/// Add a key, returning the updated input.
pub fn add_key(state: &mut GameState, key: &str) -> Input {
    game::update_input_state(state, InputState::Edit);
    game::add_key(state, key);
    state.input.clone()
}

/// Remove the last key, returning the updated input.
pub fn remove_key(state: &mut GameState) -> Input {
    game::update_input_state(state, InputState::Edit);
    game::remove_key(state);
    state.input.clone()
}

/// Submit the current solution attempt.
pub fn submit_solution(state: &mut GameState) -> SubmitResult {
    let (solved, exhausted) = game::submit_solution(state);
    SubmitResult {
        solved,
        exhausted,
        guesses: state.guesses,
        input_state: state.input.state.clone(),
        puzzle_state: state.puzzle.state.clone(),
        stats: state.stats.clone(),
    }
}

/// Activate a clue by id.
pub fn activate_clue(state: &mut GameState, clue_id: &str) -> ClueResult {
    game::activate_clue(state, clue_id);
    game::apply_clue_effects(state, clue_id);

    let stats = if clue_id == "solve" {
        Some(state.stats.clone())
    } else {
        None
    };

    ClueResult {
        clues: state.clues.clone(),
        input: state.input.clone(),
        puzzle: state.puzzle.clone(),
        keys: state.keys.clone(),
        stats,
    }
}

/// Start a new (random) game, preserving stats.
pub fn new_game(state: &mut GameState) -> GameState {
    game::new_game(state);
    game::record_puzzle_played(state);
    state.clone()
}

/// Clear input after incorrect guess.
pub fn clear_input(state: &mut GameState) -> Input {
    game::clear_input(state);
    state.input.clone()
}

use crate::game;
use crate::generator;
use crate::models::*;

/// Initialize or resume a game. Returns the game state to render.
/// `saved` is the previously persisted state (if any).
/// `now_secs` is the current Unix timestamp in seconds, pre-shifted by the
/// caller's local UTC offset so day boundaries fall at local midnight.
pub fn init_game(saved: Option<GameState>, now_secs: u64) -> GameState {
    let today = generator::date_string_from_secs(now_secs);
    let puzzle_number = generator::days_since_epoch(now_secs);

    if let Some(mut state) = saved {
        let is_todays_daily = state.puzzle_date.as_deref() == Some(today.as_str());

        if is_todays_daily {
            game::backfill_streak_history(&mut state, puzzle_number);
            return state;
        }

        // Day rolled over — the streak survives only if the last daily was
        // finished. That daily is either the live game or stashed behind an
        // archive/random game.
        let daily_unsolved = if game::is_daily_game(&state) {
            !state.puzzle.solved
        } else {
            state
                .daily_snapshot
                .as_ref()
                .is_some_and(|snap| !snap.puzzle.solved)
        };
        if daily_unsolved {
            state.stats.current_streak = 0;
        }
        state.daily_snapshot = None;

        // Generate today's daily puzzle, preserving stats
        game::initialize_with_daily_puzzle(&mut state, puzzle_number);
        state.puzzle_date = Some(today);
        game::record_puzzle_played(&mut state);
        game::backfill_streak_history(&mut state, puzzle_number);
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

/// Start an archive game for a past daily date ("YYYY-MM-DD"). Archive games
/// are stat-neutral, and the daily is stashed so `resume_daily` can restore
/// it. `now_secs` is local-shifted, as in `init_game`.
pub fn archive_game(state: &mut GameState, date: &str, now_secs: u64) -> Result<GameState, String> {
    let number = generator::days_since_epoch_from_date(date)
        .ok_or_else(|| format!("Invalid archive date: {date}"))?;
    if number >= generator::days_since_epoch(now_secs) {
        return Err("Archive puzzles must be from a past date".to_string());
    }

    // Already on this archive puzzle (e.g. a reload re-applying a URL
    // parameter) — resume it rather than restarting
    if state.mode == GameMode::Archive && state.puzzle.puzzle_number == Some(number) {
        return Ok(state.clone());
    }

    game::stash_daily(state);
    game::initialize_with_daily_puzzle(state, number);
    state.mode = GameMode::Archive;
    Ok(state.clone())
}

/// Restore the stashed daily after an archive or random game.
pub fn resume_daily(state: &mut GameState) -> GameState {
    game::resume_daily(state);
    state.clone()
}

/// Clear input after incorrect guess.
pub fn clear_input(state: &mut GameState) -> Input {
    game::clear_input(state);
    state.input.clone()
}

/// Per-date results for finished daily/archive puzzles ("YYYY-MM-DD" keys).
pub fn get_history(state: &GameState) -> History {
    state.history.clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY: u64 = 86_400;
    /// 500 days past the puzzle epoch (2025-01-01 UTC).
    const T0: u64 = 1_735_689_600 + 500 * DAY;

    #[test]
    fn fresh_start_generates_todays_daily() {
        let state = init_game(None, T0);
        assert_eq!(state.puzzle.puzzle_number, Some(500));
        assert_eq!(
            state.puzzle_date.as_deref(),
            Some(generator::date_string_from_secs(T0).as_str())
        );
        assert_eq!(state.stats.played, 1);
    }

    #[test]
    fn resume_same_day_keeps_puzzle_and_played_count() {
        let first = init_game(None, T0);
        let resumed = init_game(Some(first.clone()), T0 + 3600);
        assert_eq!(resumed.puzzle.key, first.puzzle.key);
        assert_eq!(resumed.stats.played, 1);
    }

    #[test]
    fn rollover_generates_new_daily() {
        let first = init_game(None, T0);
        let next = init_game(Some(first.clone()), T0 + DAY);
        assert_eq!(next.puzzle.puzzle_number, Some(501));
        assert_ne!(next.puzzle_date, first.puzzle_date);
        assert_eq!(next.stats.played, 2);
    }

    #[test]
    fn rollover_resets_streak_when_daily_unsolved() {
        let mut first = init_game(None, T0);
        first.stats.current_streak = 3;

        let next = init_game(Some(first), T0 + DAY);
        assert_eq!(next.stats.current_streak, 0);
    }

    #[test]
    fn rollover_preserves_streak_when_daily_solved() {
        let mut first = init_game(None, T0);
        first.puzzle.solved = true;
        first.stats.current_streak = 3;

        let next = init_game(Some(first), T0 + DAY);
        assert_eq!(next.stats.current_streak, 3);
    }

    #[test]
    fn rollover_preserves_streak_when_random_game_unsolved() {
        let mut state = init_game(None, T0);
        state.puzzle.solved = true;
        state.stats.current_streak = 3;

        // Daily solved, then an unfinished random game overnight
        new_game(&mut state);
        assert!(state.puzzle.puzzle_number.is_none());

        let next = init_game(Some(state), T0 + DAY);
        assert_eq!(next.stats.current_streak, 3);
        assert_eq!(next.puzzle.puzzle_number, Some(501));
    }

    // --- archive tests ---

    fn date_for(secs: u64) -> String {
        generator::date_string_from_secs(secs)
    }

    #[test]
    fn archive_game_loads_past_puzzle_and_stashes_daily() {
        let mut state = init_game(None, T0);

        let result = archive_game(&mut state, &date_for(T0 - 5 * DAY), T0).unwrap();

        assert_eq!(result.puzzle.puzzle_number, Some(495));
        assert_eq!(result.mode, GameMode::Archive);
        assert_eq!(result.stats.played, 1); // stat-neutral
        assert_eq!(
            result.daily_snapshot.as_ref().unwrap().puzzle.puzzle_number,
            Some(500)
        );
        // Deterministic: same date → same puzzle as that day's daily
        let expected = generator::generate_daily_puzzle(495);
        assert_eq!(result.puzzle.key, expected.key);
    }

    #[test]
    fn archive_game_rejects_today_future_and_invalid_dates() {
        let mut state = init_game(None, T0);

        assert!(archive_game(&mut state, &date_for(T0), T0).is_err());
        assert!(archive_game(&mut state, &date_for(T0 + DAY), T0).is_err());
        assert!(archive_game(&mut state, "2024-12-31", T0).is_err());
        assert!(archive_game(&mut state, "not-a-date", T0).is_err());

        // Failed attempts leave the daily untouched
        assert_eq!(state.mode, GameMode::Daily);
        assert_eq!(state.puzzle.puzzle_number, Some(500));
    }

    #[test]
    fn archive_game_same_date_resumes_in_progress() {
        let mut state = init_game(None, T0);
        let date = date_for(T0 - 5 * DAY);

        archive_game(&mut state, &date, T0).unwrap();
        add_key(&mut state, "A");

        let resumed = archive_game(&mut state, &date, T0).unwrap();
        assert_eq!(resumed.input.keys[0], "A");
    }

    #[test]
    fn resume_daily_restores_stashed_daily() {
        let mut state = init_game(None, T0);
        add_key(&mut state, "Q");

        archive_game(&mut state, &date_for(T0 - 5 * DAY), T0).unwrap();
        let restored = resume_daily(&mut state);

        assert_eq!(restored.mode, GameMode::Daily);
        assert_eq!(restored.puzzle.puzzle_number, Some(500));
        assert_eq!(restored.input.keys[0], "Q");
        assert!(restored.daily_snapshot.is_none());
    }

    #[test]
    fn rollover_resets_streak_when_stashed_daily_unsolved() {
        let mut state = init_game(None, T0);
        state.stats.current_streak = 3;

        // Daily left unsolved behind an archive game overnight
        archive_game(&mut state, &date_for(T0 - 5 * DAY), T0).unwrap();

        let next = init_game(Some(state), T0 + DAY);
        assert_eq!(next.stats.current_streak, 0);
        assert_eq!(next.puzzle.puzzle_number, Some(501));
        assert!(next.daily_snapshot.is_none());
    }

    #[test]
    fn rollover_preserves_streak_when_stashed_daily_solved() {
        let mut state = init_game(None, T0);
        state.puzzle.solved = true;
        state.stats.current_streak = 3;

        archive_game(&mut state, &date_for(T0 - 5 * DAY), T0).unwrap();

        let next = init_game(Some(state), T0 + DAY);
        assert_eq!(next.stats.current_streak, 3);
    }

    // --- history tests ---

    fn fill_correct_answer(state: &mut GameState) {
        state.input.keys = state.puzzle.key.chars().map(|c| c.to_string()).collect();
    }

    #[test]
    fn solving_daily_records_history_as_daily() {
        let mut state = init_game(None, T0);
        fill_correct_answer(&mut state);

        let result = submit_solution(&mut state);
        assert!(result.solved);

        let history = get_history(&state);
        let rec = history.get(&date_for(T0)).expect("daily result recorded");
        assert!(rec.solved);
        assert!(rec.daily);
        assert_eq!(rec.guesses, 1);
    }

    #[test]
    fn solving_archive_records_history_as_non_daily() {
        let mut state = init_game(None, T0);
        let date = date_for(T0 - 5 * DAY);
        archive_game(&mut state, &date, T0).unwrap();
        fill_correct_answer(&mut state);

        submit_solution(&mut state);

        let rec = get_history(&state).remove(&date).expect("archive result recorded");
        assert!(rec.solved);
        assert!(!rec.daily);
    }

    #[test]
    fn backfill_reconstructs_streak_for_pre_history_saves() {
        let mut state = init_game(None, T0);
        state.stats.current_streak = 3;

        let resumed = init_game(Some(state), T0 + 3600);

        // Live daily unsolved, so the streak ends yesterday
        assert_eq!(resumed.history.len(), 3);
        for i in 1..=3 {
            let rec = resumed.history.get(&date_for(T0 - i * DAY)).unwrap();
            assert!(rec.solved);
            assert!(rec.daily);
            assert_eq!(rec.guesses, 0); // unknown-detail marker
        }
    }

    #[test]
    fn backfill_includes_today_when_daily_solved() {
        let mut state = init_game(None, T0);
        state.puzzle.solved = true;
        state.stats.current_streak = 3;

        let resumed = init_game(Some(state), T0 + 3600);

        assert_eq!(resumed.history.len(), 3);
        assert!(resumed.history.contains_key(&date_for(T0)));
        assert!(resumed.history.contains_key(&date_for(T0 - 2 * DAY)));
    }

    #[test]
    fn backfill_skips_when_history_exists() {
        let mut state = init_game(None, T0);
        fill_correct_answer(&mut state);
        submit_solution(&mut state); // records today, streak = 1
        state.stats.current_streak = 5;

        let resumed = init_game(Some(state), T0 + 3600);

        assert_eq!(resumed.history.len(), 1);
    }

    #[test]
    fn history_survives_rollover() {
        let mut state = init_game(None, T0);
        fill_correct_answer(&mut state);
        submit_solution(&mut state);

        let next = init_game(Some(state), T0 + DAY);
        assert!(next.history.contains_key(&date_for(T0)));
    }
}

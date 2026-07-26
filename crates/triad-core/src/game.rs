use rand::seq::SliceRandom;

use crate::generator;
use crate::models::*;

// --- Input operations ---

pub fn add_key(state: &mut GameState, key: &str) {
    let keys = &mut state.input.keys;
    let len = keys.len();

    let empty_idx = keys.iter().position(|k| k.is_empty());
    if let Some(idx) = empty_idx {
        if state.input.last_position_locked && idx == len - 1 {
            return;
        }
        keys[idx] = key.to_string();
    }
}

pub fn remove_key(state: &mut GameState) {
    let keys = &mut state.input.keys;
    let search_range = if state.input.last_position_locked {
        &keys[..keys.len() - 1]
    } else {
        &keys[..]
    };

    let last_filled = search_range.iter().rposition(|k| !k.is_empty());
    if let Some(idx) = last_filled {
        keys[idx] = "".to_string();
    }
}

pub fn lock_clue_key(state: &mut GameState, key: &str) {
    let len = state.input.keys.len();
    state.input.keys[len - 1] = key.to_string();
    state.input.last_position_locked = true;
}

pub fn update_input_state(state: &mut GameState, input_state: InputState) {
    state.input.state = input_state;
}

// --- Validation ---

pub fn valid_solution(input_keys: &[String], puzzle_key: &str) -> bool {
    let user_input: String = input_keys.iter().map(|k| k.to_uppercase()).collect();
    user_input == puzzle_key
}

pub const MAX_GUESSES: u32 = 6;

pub fn submit_solution(state: &mut GameState) -> (bool, bool) {
    state.guesses += 1;
    let solved = valid_solution(&state.input.keys, &state.puzzle.key);
    // Archive games are stat-neutral: no solved count, streak, or distribution
    let counts_stats = state.mode != GameMode::Archive;

    if solved {
        state.input.state = InputState::Correct;
        mark_puzzle_solved(state);
        update_puzzle_state(state, PuzzleState::Solution);
        if counts_stats {
            record_puzzle_solved(state);
            record_guess_distribution(state);
        }
        record_day_result(state, true);
        return (true, false);
    }

    let exhausted = state.guesses >= MAX_GUESSES;
    if exhausted {
        // Show the answer and count as a loss
        let key_chars: Vec<String> = state.puzzle.key.chars().map(|c| c.to_string()).collect();
        state.input.keys = key_chars;
        state.input.state = InputState::Correct;
        state.input.disabled = true;
        mark_puzzle_solved(state);
        update_puzzle_state(state, PuzzleState::Solution);
        if counts_stats {
            state.stats.current_streak = 0;
        }
        record_day_result(state, false);
    } else {
        state.input.state = InputState::Incorrect;
    }

    (false, exhausted)
}

pub fn record_guess_distribution(state: &mut GameState) {
    let bucket = std::cmp::min(state.guesses as usize - 1, 5);
    while state.stats.guess_distribution.len() < 6 {
        state.stats.guess_distribution.push(0);
    }
    state.stats.guess_distribution[bucket] += 1;
}

// --- Clue operations ---

pub fn activate_clue(state: &mut GameState, clue_id: &str) {
    let clue_index = state.clues.clues.iter().position(|c| c.id == clue_id);

    if let Some(idx) = clue_index {
        if state.clues.clues[idx].active || !state.clues.available {
            return;
        }

        // Solve clue requires all other clues to be used first
        if clue_id == "solve" && state.clues.used < 3 {
            return;
        }

        state.clues.clues[idx].active = true;
        state.clues.used += 1;
        state.clues.available = state.clues.used < 4;
    }
}

pub fn apply_clue_effects(state: &mut GameState, clue_id: &str) {
    match clue_id {
        "letter" => {
            let clue_key = state
                .puzzle
                .key
                .chars()
                .last()
                .map(|c| c.to_string())
                .unwrap_or_default();
            lock_clue_key(state, &clue_key);
        }
        "position" => {
            update_puzzle_state(state, PuzzleState::Clue);
        }
        "50/50" => {
            disable_keys(state);
        }
        "solve" => {
            solve_puzzle(state);
        }
        _ => {}
    }
}

pub fn solve_puzzle(state: &mut GameState) {
    let key_chars: Vec<String> = state.puzzle.key.chars().map(|c| c.to_string()).collect();
    state.input.keys = key_chars;
    state.input.state = InputState::Correct;
    state.input.disabled = true;

    mark_puzzle_solved(state);
    update_puzzle_state(state, PuzzleState::Solution);

    // Counts as a loss — reset streak, no solved++ (archive games stay stat-neutral)
    if state.mode != GameMode::Archive {
        state.stats.current_streak = 0;
        state.stats.solve_clue_count += 1;
    }
    record_day_result(state, false);
}

// --- Keys operations ---

pub fn disable_keys(state: &mut GameState) {
    let alpha = "qwertyuiopasdfghjklzxcvbnm";
    let puzzle_key = state.puzzle.key.to_lowercase();

    let keys_to_disable: Vec<String> = alpha
        .chars()
        .filter(|c| !puzzle_key.contains(*c))
        .map(|c| c.to_string())
        .collect();

    let mut rng = rand::thread_rng();
    let mut shuffled = keys_to_disable;
    shuffled.shuffle(&mut rng);

    state.keys.disabled_keys = shuffled.into_iter().take(13).collect();
    state.keys.keys_disabled = true;
}

// --- History operations ---

/// Records the outcome of a dated (daily or archive) puzzle into `history`.
/// A solved record is never downgraded: replays only overwrite an unsolved
/// entry, so a solve as the live daily keeps its `daily` flag over later
/// archive replays.
pub fn record_day_result(state: &mut GameState, solved: bool) {
    if state.mode == GameMode::Random {
        return;
    }
    let Some(number) = state.puzzle.puzzle_number else {
        return;
    };
    let date = generator::date_string_from_number(number);

    if state.history.get(&date).is_some_and(|r| r.solved) {
        return;
    }
    state.history.insert(
        date,
        DayRecord {
            solved,
            guesses: state.guesses,
            daily: state.mode == GameMode::Daily,
            perfect: solved && state.guesses == 1 && state.clues.used == 0,
        },
    );
}

/// One-time migration for saves that predate per-date history: a current
/// streak of N means the last N consecutive dailies were solved, so those
/// days are reconstructable even though nothing else is. The streak ends at
/// today when the live daily is already solved, otherwise at yesterday.
/// Backfilled records use `guesses: 0` (impossible normally) as an
/// "unknown detail" marker. No-op once any history exists.
pub fn backfill_streak_history(state: &mut GameState, today_number: u32) {
    if !state.history.is_empty() || state.stats.current_streak == 0 {
        return;
    }

    // Today's daily may be the live game or stashed behind an archive/random game
    let daily_solved_today = if is_daily_game(state) {
        state.puzzle.solved
    } else {
        state
            .daily_snapshot
            .as_ref()
            .is_some_and(|snap| snap.puzzle.solved)
    };
    let end = if daily_solved_today {
        today_number
    } else {
        let Some(yesterday) = today_number.checked_sub(1) else {
            return;
        };
        yesterday
    };

    for i in 0..state.stats.current_streak {
        let Some(number) = end.checked_sub(i) else { break };
        state.history.insert(
            generator::date_string_from_number(number),
            DayRecord { solved: true, guesses: 0, daily: true, perfect: false },
        );
    }
}

// --- Stats operations ---

pub fn record_puzzle_played(state: &mut GameState) {
    state.stats.played += 1;
}

pub fn record_puzzle_solved(state: &mut GameState) {
    state.stats.solved += 1;
    state.stats.current_streak += 1;
    if state.stats.current_streak > state.stats.best_streak {
        state.stats.best_streak = state.stats.current_streak;
    }
}

// --- Puzzle operations ---

pub fn update_puzzle_state(state: &mut GameState, puzzle_state: PuzzleState) {
    state.puzzle.state = puzzle_state;
}

pub fn mark_puzzle_solved(state: &mut GameState) {
    state.puzzle.solved = true;
}

// --- Game lifecycle ---

/// True when the current game is the actual daily (not random or archive).
/// The `puzzle_number` check keeps pre-`mode` saves of random games from
/// being mistaken for dailies.
pub fn is_daily_game(state: &GameState) -> bool {
    state.mode == GameMode::Daily && state.puzzle.puzzle_number.is_some()
}

/// Stashes the daily so it survives an archive/random game and can be
/// restored with `resume_daily`. No-op unless the daily is the live game.
pub fn stash_daily(state: &mut GameState) {
    if !is_daily_game(state) {
        return;
    }
    state.daily_snapshot = Some(DailySnapshot {
        puzzle: state.puzzle.clone(),
        input: state.input.clone(),
        clues: state.clues.clone(),
        keys: state.keys.clone(),
        guesses: state.guesses,
    });
}

/// Restores the stashed daily. No-op if there is nothing stashed.
pub fn resume_daily(state: &mut GameState) {
    if let Some(snap) = state.daily_snapshot.take() {
        state.puzzle = snap.puzzle;
        state.input = snap.input;
        state.clues = snap.clues;
        state.keys = snap.keys;
        state.guesses = snap.guesses;
        state.mode = GameMode::Daily;
    }
}

/// Resets game state for a new random puzzle while preserving stats.
/// The daily is stashed, not abandoned — any streak penalty for an
/// unfinished daily is decided at day rollover.
pub fn new_game(state: &mut GameState) {
    stash_daily(state);

    let previous_key = state.puzzle.key.to_lowercase();

    let puzzle = generator::generate_puzzle(Some(&previous_key));
    let key_len = puzzle.key.len();

    state.puzzle = puzzle;
    state.input = Input {
        length: key_len,
        disabled: false,
        keys: vec!["".to_string(); key_len],
        state: InputState::Edit,
        last_position_locked: false,
    };
    state.clues = Clues::default();
    state.keys = Keys::default();
    state.guesses = 0;
    state.mode = GameMode::Random;
}

/// Sets up initial game state with the daily puzzle for the given puzzle number.
pub fn initialize_with_daily_puzzle(state: &mut GameState, puzzle_number: u32) {
    let puzzle = generator::generate_daily_puzzle(puzzle_number);
    let key_len = puzzle.key.len();

    state.puzzle = puzzle;
    state.input = Input {
        length: key_len,
        disabled: false,
        keys: vec!["".to_string(); key_len],
        state: InputState::Edit,
        last_position_locked: false,
    };
    state.clues = Clues::default();
    state.keys = Keys::default();
    state.guesses = 0;
    state.mode = GameMode::Daily;
}

/// Clears input keys after incorrect guess, preserving locked last position.
pub fn clear_input(state: &mut GameState) {
    let len = state.input.keys.len();
    let locked_key = if state.input.last_position_locked {
        state.input.keys.last().cloned()
    } else {
        None
    };

    state.input.keys = vec!["".to_string(); len];

    if let Some(key) = locked_key {
        state.input.keys[len - 1] = key;
    }

    state.input.state = InputState::Edit;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_state() -> GameState {
        GameState::default()
    }

    // --- valid_solution tests ---

    #[test]
    fn valid_solution_exact_match() {
        let keys: Vec<String> = vec!["F", "I", "R", "M"]
            .into_iter()
            .map(String::from)
            .collect();
        assert!(valid_solution(&keys, "FIRM"));
    }

    #[test]
    fn valid_solution_case_insensitive() {
        let keys: Vec<String> = vec!["f", "i", "r", "m"]
            .into_iter()
            .map(String::from)
            .collect();
        assert!(valid_solution(&keys, "FIRM"));
    }

    #[test]
    fn valid_solution_mixed_case() {
        let keys: Vec<String> = vec!["f", "I", "r", "M"]
            .into_iter()
            .map(String::from)
            .collect();
        assert!(valid_solution(&keys, "FIRM"));
    }

    #[test]
    fn valid_solution_incorrect() {
        let keys: Vec<String> = vec!["T", "E", "S", "T"]
            .into_iter()
            .map(String::from)
            .collect();
        assert!(!valid_solution(&keys, "FIRM"));
    }

    #[test]
    fn valid_solution_partial() {
        let keys: Vec<String> = vec!["F", "I", "", ""]
            .into_iter()
            .map(String::from)
            .collect();
        assert!(!valid_solution(&keys, "FIRM"));
    }

    #[test]
    fn valid_solution_empty() {
        let keys: Vec<String> = vec!["", "", "", ""]
            .into_iter()
            .map(String::from)
            .collect();
        assert!(!valid_solution(&keys, "FIRM"));
    }

    // --- submit_solution tests ---

    #[test]
    fn submit_solution_correct() {
        let mut state = default_state();
        state.input.keys = vec!["F", "I", "R", "M"]
            .into_iter()
            .map(String::from)
            .collect();

        let (solved, exhausted) = submit_solution(&mut state);
        assert!(solved);
        assert!(!exhausted);
        assert_eq!(state.guesses, 1);
        assert_eq!(state.input.state, InputState::Correct);
        assert!(state.puzzle.solved);
        assert_eq!(state.puzzle.state, PuzzleState::Solution);
        assert_eq!(state.stats.solved, 1);
        assert_eq!(state.stats.guess_distribution[0], 1); // bucket 0 = 1 guess
    }

    #[test]
    fn submit_solution_incorrect() {
        let mut state = default_state();
        state.input.keys = vec!["X", "Y", "Z", "W"]
            .into_iter()
            .map(String::from)
            .collect();

        let (solved, exhausted) = submit_solution(&mut state);
        assert!(!solved);
        assert!(!exhausted);
        assert_eq!(state.guesses, 1);
        assert_eq!(state.input.state, InputState::Incorrect);
        assert!(!state.puzzle.solved);
        assert_eq!(state.stats.solved, 0);
    }

    #[test]
    fn submit_solution_exhausted_after_six() {
        let mut state = default_state();
        let wrong: Vec<String> = vec!["X", "Y", "Z", "W"].into_iter().map(String::from).collect();

        for i in 0..5 {
            state.input.keys = wrong.clone();
            let (solved, exhausted) = submit_solution(&mut state);
            assert!(!solved);
            assert!(!exhausted, "should not be exhausted at guess {}", i + 1);
            state.input.state = InputState::Edit; // simulate clear
        }

        // 6th guess — exhausted
        state.input.keys = wrong;
        let (solved, exhausted) = submit_solution(&mut state);
        assert!(!solved);
        assert!(exhausted);
        assert_eq!(state.guesses, 6);
        assert!(state.puzzle.solved); // marked solved to show answer
        assert_eq!(state.stats.current_streak, 0);
        assert_eq!(state.stats.solved, 0); // not counted as solved
    }

    #[test]
    fn submit_solution_increments_guesses() {
        let mut state = default_state();
        let wrong: Vec<String> = vec!["X", "Y", "Z", "W"].into_iter().map(String::from).collect();

        state.input.keys = wrong.clone();
        submit_solution(&mut state);
        assert_eq!(state.guesses, 1);

        state.input.keys = wrong;
        state.input.state = InputState::Edit;
        submit_solution(&mut state);
        assert_eq!(state.guesses, 2);
    }

    #[test]
    fn guess_distribution_correct_bucket() {
        let mut state = default_state();
        let wrong: Vec<String> = vec!["X", "Y", "Z", "W"].into_iter().map(String::from).collect();
        let correct: Vec<String> = vec!["F", "I", "R", "M"].into_iter().map(String::from).collect();

        // 2 wrong then correct = 3 guesses → bucket index 2
        state.input.keys = wrong.clone();
        submit_solution(&mut state);
        state.input.keys = wrong;
        state.input.state = InputState::Edit;
        submit_solution(&mut state);
        state.input.keys = correct;
        state.input.state = InputState::Edit;
        submit_solution(&mut state);

        assert_eq!(state.stats.guess_distribution[2], 1); // bucket 2 = 3 guesses
    }

    // --- add_key / remove_key tests ---

    #[test]
    fn add_key_basic() {
        let mut state = default_state();
        add_key(&mut state, "F");
        assert_eq!(state.input.keys[0], "F");
        assert_eq!(state.input.keys[1], "");
    }

    #[test]
    fn add_key_fills_sequentially() {
        let mut state = default_state();
        add_key(&mut state, "F");
        add_key(&mut state, "I");
        add_key(&mut state, "R");
        add_key(&mut state, "M");
        assert_eq!(
            state.input.keys,
            vec!["F", "I", "R", "M"]
        );
    }

    #[test]
    fn add_key_full_input_noop() {
        let mut state = default_state();
        state.input.keys = vec!["F", "I", "R", "M"]
            .into_iter()
            .map(String::from)
            .collect();
        add_key(&mut state, "X");
        assert_eq!(
            state.input.keys,
            vec!["F", "I", "R", "M"]
        );
    }

    #[test]
    fn add_key_respects_last_position_locked() {
        let mut state = default_state();
        state.input.last_position_locked = true;
        state.input.keys = vec!["F", "I", "R", ""]
            .into_iter()
            .map(String::from)
            .collect();
        // Last slot is empty but locked — should not fill
        add_key(&mut state, "X");
        assert_eq!(state.input.keys[3], "");
    }

    #[test]
    fn remove_key_basic() {
        let mut state = default_state();
        state.input.keys = vec!["F", "I", "", ""]
            .into_iter()
            .map(String::from)
            .collect();
        remove_key(&mut state);
        assert_eq!(
            state.input.keys,
            vec!["F", "", "", ""]
        );
    }

    #[test]
    fn remove_key_empty_noop() {
        let mut state = default_state();
        remove_key(&mut state);
        assert_eq!(
            state.input.keys,
            vec!["", "", "", ""]
        );
    }

    #[test]
    fn remove_key_respects_last_position_locked() {
        let mut state = default_state();
        state.input.last_position_locked = true;
        state.input.keys = vec!["F", "", "", "M"]
            .into_iter()
            .map(String::from)
            .collect();
        remove_key(&mut state);
        // Should remove "F" (last filled in non-locked range), not "M"
        assert_eq!(state.input.keys[0], "");
        assert_eq!(state.input.keys[3], "M");
    }

    // --- activate_clue tests ---

    #[test]
    fn activate_clue_marks_active() {
        let mut state = default_state();
        activate_clue(&mut state, "letter");
        let clue = state.clues.clues.iter().find(|c| c.id == "letter").unwrap();
        assert!(clue.active);
        assert_eq!(state.clues.used, 1);
        assert!(state.clues.available);
    }

    #[test]
    fn activate_clue_idempotent() {
        let mut state = default_state();
        activate_clue(&mut state, "letter");
        activate_clue(&mut state, "letter");
        assert_eq!(state.clues.used, 1);
    }

    #[test]
    fn activate_clue_available_after_three() {
        let mut state = default_state();
        activate_clue(&mut state, "position");
        activate_clue(&mut state, "letter");
        activate_clue(&mut state, "50/50");
        assert!(state.clues.available); // solve clue still available
        assert_eq!(state.clues.used, 3);
    }

    #[test]
    fn activate_clue_unavailable_after_four() {
        let mut state = default_state();
        activate_clue(&mut state, "position");
        activate_clue(&mut state, "letter");
        activate_clue(&mut state, "50/50");
        activate_clue(&mut state, "solve");
        assert!(!state.clues.available);
        assert_eq!(state.clues.used, 4);
    }

    #[test]
    fn activate_solve_clue_rejected_before_three() {
        let mut state = default_state();
        activate_clue(&mut state, "position");
        activate_clue(&mut state, "solve");
        // Solve should be rejected — only 1 clue used
        let solve = state.clues.clues.iter().find(|c| c.id == "solve").unwrap();
        assert!(!solve.active);
        assert_eq!(state.clues.used, 1);
    }

    #[test]
    fn solve_puzzle_fills_answer_and_resets_streak() {
        let mut state = default_state();
        state.stats.current_streak = 5;

        solve_puzzle(&mut state);

        assert_eq!(state.input.keys, vec!["F", "I", "R", "M"]);
        assert_eq!(state.input.state, InputState::Correct);
        assert!(state.input.disabled);
        assert!(state.puzzle.solved);
        assert_eq!(state.stats.current_streak, 0);
        assert_eq!(state.stats.solve_clue_count, 1);
        assert_eq!(state.stats.solved, 0); // not counted as solved
    }

    // --- apply_clue_effects tests ---

    #[test]
    fn apply_clue_letter_locks_key() {
        let mut state = default_state();
        apply_clue_effects(&mut state, "letter");
        assert_eq!(state.input.keys[3], "M"); // last char of "FIRM"
        assert!(state.input.last_position_locked);
    }

    #[test]
    fn apply_clue_position_changes_state() {
        let mut state = default_state();
        apply_clue_effects(&mut state, "position");
        assert_eq!(state.puzzle.state, PuzzleState::Clue);
    }

    #[test]
    fn apply_clue_fifty_fifty_disables_keys() {
        let mut state = default_state();
        apply_clue_effects(&mut state, "50/50");
        assert!(state.keys.keys_disabled);
        assert_eq!(state.keys.disabled_keys.len(), 13);
        // None of the disabled keys should be in FIRM
        for k in &state.keys.disabled_keys {
            assert!(
                !"firm".contains(&k.to_lowercase()),
                "disabled key '{}' is in puzzle key",
                k
            );
        }
    }

    // --- disable_keys tests ---

    #[test]
    fn disable_keys_count() {
        let mut state = default_state();
        disable_keys(&mut state);
        assert_eq!(state.keys.disabled_keys.len(), 13);
        assert!(state.keys.keys_disabled);
    }

    #[test]
    fn disable_keys_excludes_puzzle_chars() {
        let mut state = default_state();
        disable_keys(&mut state);
        let puzzle_chars: Vec<char> = state.puzzle.key.to_lowercase().chars().collect();
        for key in &state.keys.disabled_keys {
            for c in key.chars() {
                assert!(
                    !puzzle_chars.contains(&c),
                    "disabled key '{}' contains puzzle char",
                    key
                );
            }
        }
    }

    // --- stats tests ---

    #[test]
    fn record_puzzle_played_increments() {
        let mut state = default_state();
        record_puzzle_played(&mut state);
        assert_eq!(state.stats.played, 1);
        record_puzzle_played(&mut state);
        record_puzzle_played(&mut state);
        assert_eq!(state.stats.played, 3);
    }

    #[test]
    fn record_puzzle_solved_increments_and_streaks() {
        let mut state = default_state();
        record_puzzle_solved(&mut state);
        assert_eq!(state.stats.solved, 1);
        assert_eq!(state.stats.current_streak, 1);
    }

    #[test]
    fn record_puzzle_solved_tracks_best_streak() {
        let mut state = default_state();

        // Solve 3 in a row
        record_puzzle_solved(&mut state);
        record_puzzle_solved(&mut state);
        record_puzzle_solved(&mut state);
        assert_eq!(state.stats.current_streak, 3);
        assert_eq!(state.stats.best_streak, 3);

        // Reset streak (simulate skipping a puzzle)
        state.stats.current_streak = 0;

        // Solve 2 more — best streak should remain 3
        record_puzzle_solved(&mut state);
        record_puzzle_solved(&mut state);
        assert_eq!(state.stats.current_streak, 2);
        assert_eq!(state.stats.best_streak, 3);
    }

    // --- puzzle operations tests ---

    #[test]
    fn mark_puzzle_solved_sets_flag() {
        let mut state = default_state();
        assert!(!state.puzzle.solved);
        mark_puzzle_solved(&mut state);
        assert!(state.puzzle.solved);
    }

    #[test]
    fn update_puzzle_state_changes_state() {
        let mut state = default_state();
        assert_eq!(state.puzzle.state, PuzzleState::Start);
        update_puzzle_state(&mut state, PuzzleState::Clue);
        assert_eq!(state.puzzle.state, PuzzleState::Clue);
        update_puzzle_state(&mut state, PuzzleState::Solution);
        assert_eq!(state.puzzle.state, PuzzleState::Solution);
    }

    // --- lock_clue_key tests ---

    #[test]
    fn lock_clue_key_sets_last_position() {
        let mut state = default_state();
        lock_clue_key(&mut state, "M");
        assert_eq!(state.input.keys[3], "M");
        assert!(state.input.last_position_locked);
    }

    // --- new_game tests ---

    #[test]
    fn new_game_resets_puzzle_and_input() {
        let mut state = default_state();
        state.puzzle.solved = true;
        state.stats.played = 5;
        state.stats.solved = 3;

        new_game(&mut state);

        assert!(!state.puzzle.solved);
        assert_eq!(state.puzzle.state, PuzzleState::Start);
        assert_eq!(state.input.state, InputState::Edit);
        assert!(!state.input.disabled);
        assert!(!state.input.last_position_locked);
        // Stats preserved
        assert_eq!(state.stats.played, 5);
        assert_eq!(state.stats.solved, 3);
    }

    #[test]
    fn new_game_input_length_matches_key() {
        let mut state = default_state();
        new_game(&mut state);

        let key_len = state.puzzle.key.len();
        assert_eq!(state.input.length, key_len);
        assert_eq!(state.input.keys.len(), key_len);
    }

    #[test]
    fn new_game_stashes_daily_and_preserves_streak() {
        let mut state = default_state();
        state.stats.current_streak = 5;
        state.puzzle.solved = false;
        state.puzzle.puzzle_number = Some(42);
        state.guesses = 2;

        new_game(&mut state);

        // Streak penalty is decided at rollover, not here
        assert_eq!(state.stats.current_streak, 5);
        assert_eq!(state.mode, GameMode::Random);
        let snap = state.daily_snapshot.as_ref().expect("daily should be stashed");
        assert_eq!(snap.puzzle.puzzle_number, Some(42));
        assert_eq!(snap.guesses, 2);
        assert!(!snap.puzzle.solved);
    }

    #[test]
    fn new_game_does_not_stash_non_daily() {
        let mut state = default_state();
        state.puzzle.puzzle_number = None; // random game

        new_game(&mut state);

        assert!(state.daily_snapshot.is_none());
    }

    #[test]
    fn new_game_keeps_existing_snapshot() {
        let mut state = default_state();
        state.puzzle.puzzle_number = Some(42);
        new_game(&mut state); // stashes daily #42, now a random game

        new_game(&mut state); // random → random

        let snap = state.daily_snapshot.as_ref().unwrap();
        assert_eq!(snap.puzzle.puzzle_number, Some(42));
    }

    #[test]
    fn resume_daily_restores_stashed_game() {
        let mut state = default_state();
        state.puzzle.puzzle_number = Some(42);
        state.guesses = 3;
        let daily_key = state.puzzle.key.clone();
        new_game(&mut state);

        resume_daily(&mut state);

        assert_eq!(state.mode, GameMode::Daily);
        assert_eq!(state.puzzle.puzzle_number, Some(42));
        assert_eq!(state.puzzle.key, daily_key);
        assert_eq!(state.guesses, 3);
        assert!(state.daily_snapshot.is_none());
    }

    #[test]
    fn resume_daily_noop_without_snapshot() {
        let mut state = default_state();
        state.mode = GameMode::Random;
        state.puzzle.puzzle_number = None;

        resume_daily(&mut state);

        assert_eq!(state.mode, GameMode::Random);
    }

    // --- archive mode stat-neutrality ---

    #[test]
    fn archive_solve_does_not_touch_stats() {
        let mut state = default_state();
        state.mode = GameMode::Archive;
        state.stats.current_streak = 3;
        state.input.keys = vec!["F", "I", "R", "M"].into_iter().map(String::from).collect();

        let (solved, _) = submit_solution(&mut state);

        assert!(solved);
        assert!(state.puzzle.solved);
        assert_eq!(state.stats.solved, 0);
        assert_eq!(state.stats.current_streak, 3);
        assert!(state.stats.guess_distribution.iter().all(|&c| c == 0));
    }

    #[test]
    fn archive_exhaustion_does_not_reset_streak() {
        let mut state = default_state();
        state.mode = GameMode::Archive;
        state.stats.current_streak = 3;
        let wrong: Vec<String> = vec!["X", "Y", "Z", "W"].into_iter().map(String::from).collect();

        for _ in 0..6 {
            state.input.keys = wrong.clone();
            state.input.state = InputState::Edit;
            submit_solution(&mut state);
        }

        assert!(state.puzzle.solved); // answer revealed
        assert_eq!(state.stats.current_streak, 3);
    }

    #[test]
    fn archive_solve_clue_does_not_touch_stats() {
        let mut state = default_state();
        state.mode = GameMode::Archive;
        state.stats.current_streak = 3;

        solve_puzzle(&mut state);

        assert_eq!(state.stats.current_streak, 3);
        assert_eq!(state.stats.solve_clue_count, 0);
    }

    #[test]
    fn new_game_preserves_streak_when_solved() {
        let mut state = default_state();
        state.stats.current_streak = 5;
        state.puzzle.solved = true;

        new_game(&mut state);

        assert_eq!(state.stats.current_streak, 5);
    }

    #[test]
    fn new_game_resets_guesses() {
        let mut state = default_state();
        state.guesses = 4;
        state.puzzle.solved = true;

        new_game(&mut state);

        assert_eq!(state.guesses, 0);
    }

    #[test]
    fn new_game_resets_clues() {
        let mut state = default_state();
        state.clues.used = 2;
        state.clues.available = false;

        new_game(&mut state);

        assert_eq!(state.clues.used, 0);
        assert!(state.clues.available);
    }

    // --- record_day_result tests ---

    #[test]
    fn exhaustion_records_failed_day() {
        let mut state = default_state();
        state.puzzle.puzzle_number = Some(42);
        let date = generator::date_string_from_number(42);
        let wrong: Vec<String> = vec!["X", "Y", "Z", "W"].into_iter().map(String::from).collect();

        for _ in 0..6 {
            state.input.keys = wrong.clone();
            state.input.state = InputState::Edit;
            submit_solution(&mut state);
        }

        let rec = state.history.get(&date).expect("loss recorded");
        assert!(!rec.solved);
        assert!(rec.daily);
        assert_eq!(rec.guesses, 6);
    }

    #[test]
    fn solve_clue_records_failed_day() {
        let mut state = default_state();
        state.puzzle.puzzle_number = Some(42);

        solve_puzzle(&mut state);

        let date = generator::date_string_from_number(42);
        assert!(!state.history.get(&date).unwrap().solved);
    }

    #[test]
    fn archive_solve_upgrades_failed_record() {
        let mut state = default_state();
        state.puzzle.puzzle_number = Some(42);
        let date = generator::date_string_from_number(42);
        state.history.insert(
            date.clone(),
            DayRecord { solved: false, guesses: 6, daily: true, perfect: false },
        );
        state.mode = GameMode::Archive;
        state.input.keys = vec!["F", "I", "R", "M"].into_iter().map(String::from).collect();

        submit_solution(&mut state);

        let rec = state.history.get(&date).unwrap();
        assert!(rec.solved);
        assert!(!rec.daily);
    }

    #[test]
    fn archive_replay_does_not_downgrade_solved_record() {
        let mut state = default_state();
        state.puzzle.puzzle_number = Some(42);
        let date = generator::date_string_from_number(42);
        state.history.insert(
            date.clone(),
            DayRecord { solved: true, guesses: 2, daily: true, perfect: false },
        );
        state.mode = GameMode::Archive;

        solve_puzzle(&mut state);

        let rec = state.history.get(&date).unwrap();
        assert!(rec.solved);
        assert!(rec.daily);
        assert_eq!(rec.guesses, 2);
    }

    #[test]
    fn first_guess_solve_without_clues_is_perfect() {
        let mut state = default_state();
        state.puzzle.puzzle_number = Some(42);
        state.input.keys = vec!["F", "I", "R", "M"].into_iter().map(String::from).collect();

        submit_solution(&mut state);

        let date = generator::date_string_from_number(42);
        assert!(state.history.get(&date).unwrap().perfect);
    }

    #[test]
    fn solve_with_clue_or_extra_guess_is_not_perfect() {
        // Clue used disqualifies even a first-guess solve
        let mut state = default_state();
        state.puzzle.puzzle_number = Some(42);
        activate_clue(&mut state, "position");
        state.input.keys = vec!["F", "I", "R", "M"].into_iter().map(String::from).collect();
        submit_solution(&mut state);
        let date = generator::date_string_from_number(42);
        assert!(!state.history.get(&date).unwrap().perfect);

        // Second-guess solve disqualifies
        let mut state = default_state();
        state.puzzle.puzzle_number = Some(42);
        state.guesses = 1;
        state.input.keys = vec!["F", "I", "R", "M"].into_iter().map(String::from).collect();
        submit_solution(&mut state);
        let rec = state.history.get(&date).unwrap();
        assert!(rec.solved);
        assert!(!rec.perfect);
    }

    #[test]
    fn random_game_records_no_history() {
        let mut state = default_state();
        state.mode = GameMode::Random;
        state.puzzle.puzzle_number = None;
        state.input.keys = vec!["F", "I", "R", "M"].into_iter().map(String::from).collect();

        submit_solution(&mut state);

        assert!(state.history.is_empty());
    }

    // --- clear_input tests ---

    #[test]
    fn clear_input_resets_keys() {
        let mut state = default_state();
        state.input.keys = vec!["A", "B", "C", "D"]
            .into_iter()
            .map(String::from)
            .collect();
        state.input.state = InputState::Incorrect;

        clear_input(&mut state);

        assert_eq!(state.input.keys, vec!["", "", "", ""]);
        assert_eq!(state.input.state, InputState::Edit);
    }

    #[test]
    fn clear_input_preserves_locked_position() {
        let mut state = default_state();
        state.input.keys = vec!["A", "B", "C", "M"]
            .into_iter()
            .map(String::from)
            .collect();
        state.input.last_position_locked = true;
        state.input.state = InputState::Incorrect;

        clear_input(&mut state);

        assert_eq!(state.input.keys, vec!["", "", "", "M"]);
        assert_eq!(state.input.state, InputState::Edit);
    }
}

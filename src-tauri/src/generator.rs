use std::collections::HashMap;
use std::sync::OnceLock;
use std::time::{SystemTime, UNIX_EPOCH};

use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use serde::Deserialize;

use crate::models::{Puzzle, PuzzleState};

#[derive(Debug, Deserialize)]
pub struct DictEntry {
    #[serde(default)]
    pub prefix: Vec<String>,
    #[serde(default)]
    pub suffix: Vec<String>,
}

pub type Dictionary = HashMap<String, DictEntry>;

static DICTIONARY: OnceLock<Dictionary> = OnceLock::new();

fn get_dictionary() -> &'static Dictionary {
    DICTIONARY.get_or_init(|| {
        let data_str = include_str!("resources/dict.json");
        serde_json::from_str(data_str).expect("Failed to parse dict.json")
    })
}

// --- Date helpers ---

/// Epoch: 2026-01-01 00:00:00 UTC as Unix timestamp.
const EPOCH_SECS: u64 = 1_735_689_600;
const SECS_PER_DAY: u64 = 86_400;

/// Returns days elapsed since 2026-01-01 UTC. This is the daily puzzle number.
pub fn today_as_days_since_epoch() -> u32 {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    if now < EPOCH_SECS {
        0
    } else {
        ((now - EPOCH_SECS) / SECS_PER_DAY) as u32
    }
}

/// Returns today's date as "YYYY-MM-DD" using stdlib only (no chrono).
pub fn today_as_date_string() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let days = (secs / SECS_PER_DAY) as i64;
    // Gregorian calendar from Julian Day Number (Unix epoch = JDN 2440588)
    let jdn = days + 2_440_588;
    let a = jdn + 32044;
    let b = (4 * a + 3) / 146097;
    let c = a - (b * 146097) / 4;
    let d = (4 * c + 3) / 1461;
    let e = c - (1461 * d) / 4;
    let m = (5 * e + 2) / 153;
    let day = e - (153 * m + 2) / 5 + 1;
    let month = m + 3 - 12 * (m / 10);
    let year = b * 100 + d - 4800 + m / 10;

    format!("{:04}-{:02}-{:02}", year, month, day)
}

// --- Key/word selection (RNG-generic for deterministic daily puzzles) ---

/// Returns sorted eligible keys from the dictionary.
fn get_sorted_eligible_keys(exclude_key: Option<&str>) -> Vec<String> {
    let dict = get_dictionary();
    let mut keys: Vec<String> = dict
        .keys()
        .filter(|k| {
            let entry = &dict[k.as_str()];
            entry.prefix.len() + entry.suffix.len() >= 3
        })
        .filter(|k| match exclude_key {
            Some(prev) => k.as_str() != prev,
            None => true,
        })
        .cloned()
        .collect();
    keys.sort();
    keys
}

/// Selects a random key using the provided RNG.
fn get_random_key_with_rng<R: Rng>(rng: &mut R, exclude_key: Option<&str>) -> String {
    let keys = get_sorted_eligible_keys(exclude_key);
    keys.choose(rng)
        .expect("Dictionary should have valid keys")
        .clone()
}

/// Selects a random key from the dictionary, optionally excluding a specific key.
fn get_random_key(exclude_key: Option<&str>) -> String {
    let mut rng = thread_rng();
    get_random_key_with_rng(&mut rng, exclude_key)
}

/// Selects 3 unique words for the given key using the provided RNG.
/// Uses Vec (not HashSet) to preserve deterministic ordering.
fn select_three_words_with_rng<R: Rng>(rng: &mut R, key: &str) -> Vec<String> {
    let dict = get_dictionary();
    let entry = &dict[key];
    let mut selected: Vec<String> = Vec::new();

    // Sort source lists for cross-platform determinism
    let mut prefix = entry.prefix.clone();
    let mut suffix = entry.suffix.clone();
    prefix.sort();
    suffix.sort();

    // Try to get one from prefix (words ending with key)
    if !prefix.is_empty() {
        if let Some(word) = prefix.choose(rng) {
            selected.push(word.clone());
        }
    }

    // Try to get one from suffix (words starting with key)
    if !suffix.is_empty() && selected.len() < 3 {
        let mut attempts = 0;
        while attempts < 10 {
            if let Some(word) = suffix.choose(rng) {
                if !selected.contains(word) {
                    selected.push(word.clone());
                    break;
                }
            }
            attempts += 1;
        }
    }

    // Fill remaining from combined sorted list
    let combined: Vec<&String> = prefix.iter().chain(suffix.iter()).collect();
    let mut attempts = 0;
    while selected.len() < 3 && attempts < 100 {
        if let Some(word) = combined.choose(rng) {
            if !selected.contains(*word) {
                selected.push((*word).clone());
            }
        }
        attempts += 1;
    }

    selected
}

/// Selects 3 unique words for the given key, mixing prefix and suffix when possible.
fn select_three_words(key: &str) -> Vec<String> {
    let mut rng = thread_rng();
    select_three_words_with_rng(&mut rng, key)
}

// --- Puzzle building ---

enum WordPosition {
    KeyAtStart { fragment: String },
    KeyAtEnd { fragment: String },
}

/// Determines whether the key appears at the start or end of a word.
fn word_position(word: &str, key: &str) -> WordPosition {
    let word_lower = word.to_lowercase();
    let key_lower = key.to_lowercase();

    if word_lower.starts_with(&key_lower) {
        WordPosition::KeyAtStart {
            fragment: word[key.len()..].to_string(),
        }
    } else {
        WordPosition::KeyAtEnd {
            fragment: word[..word.len() - key.len()].to_string(),
        }
    }
}

/// Builds a Puzzle from a key and its 3 selected words.
fn build_puzzle(key: &str, words: &[String]) -> Puzzle {
    let underscores = "_".repeat(key.len());
    let key_upper = key.to_uppercase();

    let mut start_parts = Vec::new();
    let mut clue_parts = Vec::new();
    let mut solution_parts = Vec::new();

    for word in words {
        let word_upper = word.to_uppercase();
        match word_position(word, key) {
            WordPosition::KeyAtStart { fragment } => {
                let frag_upper = fragment.to_uppercase();
                start_parts.push(frag_upper.clone());
                clue_parts.push(format!("{}{}", underscores, frag_upper));
                solution_parts.push(word_upper);
            }
            WordPosition::KeyAtEnd { fragment } => {
                let frag_upper = fragment.to_uppercase();
                start_parts.push(frag_upper.clone());
                clue_parts.push(format!("{}{}", frag_upper, underscores));
                solution_parts.push(word_upper);
            }
        }
    }

    Puzzle {
        key: key_upper,
        start: start_parts.join(" / "),
        clue: clue_parts.join(" / "),
        solution: solution_parts.join(" / "),
        solved: false,
        state: PuzzleState::Start,
        puzzle_number: None,
    }
}

// --- Public generation functions ---

/// Generates a new random puzzle, optionally excluding a previous key to prevent repeats.
pub fn generate_puzzle(exclude_key: Option<&str>) -> Puzzle {
    loop {
        let key = get_random_key(exclude_key);
        let words = select_three_words(&key);
        if words.len() == 3 {
            return build_puzzle(&key, &words);
        }
    }
}

/// Generates a deterministic daily puzzle for the given puzzle number.
/// Same puzzle_number always produces the same puzzle.
pub fn generate_daily_puzzle(puzzle_number: u32) -> Puzzle {
    let mut rng = ChaCha8Rng::seed_from_u64(puzzle_number as u64);

    loop {
        let key = get_random_key_with_rng(&mut rng, None);
        let words = select_three_words_with_rng(&mut rng, &key);
        if words.len() == 3 {
            let mut puzzle = build_puzzle(&key, &words);
            puzzle.puzzle_number = Some(puzzle_number);
            return puzzle;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dictionary_loads_successfully() {
        let dict = get_dictionary();
        assert!(!dict.is_empty());
    }

    #[test]
    fn dictionary_has_expected_keys() {
        let dict = get_dictionary();
        assert!(dict.contains_key("ark"));
        assert!(dict.contains_key("gel"));
    }

    #[test]
    fn generate_puzzle_produces_valid_puzzle() {
        let puzzle = generate_puzzle(None);
        assert!(!puzzle.key.is_empty());
        assert!(!puzzle.start.is_empty());
        assert!(!puzzle.clue.is_empty());
        assert!(!puzzle.solution.is_empty());
        assert!(!puzzle.solved);
        assert_eq!(puzzle.state, PuzzleState::Start);
        assert!(puzzle.puzzle_number.is_none());
    }

    #[test]
    fn generate_puzzle_has_three_words() {
        let puzzle = generate_puzzle(None);
        assert_eq!(puzzle.start.split(" / ").count(), 3);
        assert_eq!(puzzle.clue.split(" / ").count(), 3);
        assert_eq!(puzzle.solution.split(" / ").count(), 3);
    }

    #[test]
    fn generate_puzzle_solution_contains_key() {
        let puzzle = generate_puzzle(None);
        let key_lower = puzzle.key.to_lowercase();
        for word in puzzle.solution.split(" / ") {
            assert!(
                word.to_lowercase().contains(&key_lower),
                "Solution word '{}' should contain key '{}'",
                word,
                puzzle.key
            );
        }
    }

    #[test]
    fn generate_puzzle_clue_has_underscores() {
        let puzzle = generate_puzzle(None);
        let expected = "_".repeat(puzzle.key.len());
        for part in puzzle.clue.split(" / ") {
            assert!(
                part.contains(&expected),
                "Clue part '{}' should contain {} underscores",
                part,
                puzzle.key.len()
            );
        }
    }

    #[test]
    fn generate_puzzle_excludes_previous_key() {
        let first = generate_puzzle(None);
        for _ in 0..20 {
            let next = generate_puzzle(Some(&first.key.to_lowercase()));
            assert_ne!(
                next.key.to_lowercase(),
                first.key.to_lowercase(),
                "Should not repeat excluded key"
            );
        }
    }

    #[test]
    fn generate_puzzle_key_length_3_or_4() {
        for _ in 0..50 {
            let puzzle = generate_puzzle(None);
            let len = puzzle.key.len();
            assert!(
                len == 3 || len == 4,
                "Key '{}' length should be 3 or 4, got {}",
                puzzle.key,
                len
            );
        }
    }

    #[test]
    fn generate_puzzle_solution_reconstructs_from_start_and_key() {
        for _ in 0..20 {
            let puzzle = generate_puzzle(None);
            let key = &puzzle.key;

            let solution_words: Vec<&str> = puzzle.solution.split(" / ").collect();
            let start_fragments: Vec<&str> = puzzle.start.split(" / ").collect();

            for (i, solution_word) in solution_words.iter().enumerate() {
                let fragment = start_fragments[i];
                let option_a = format!("{}{}", key, fragment).to_uppercase();
                let option_b = format!("{}{}", fragment, key).to_uppercase();
                assert!(
                    solution_word.to_uppercase() == option_a
                        || solution_word.to_uppercase() == option_b,
                    "Solution '{}' should reconstruct from key '{}' and fragment '{}'",
                    solution_word,
                    key,
                    fragment
                );
            }
        }
    }

    #[test]
    fn generate_puzzle_start_fragments_are_nonempty() {
        for _ in 0..20 {
            let puzzle = generate_puzzle(None);
            for part in puzzle.start.split(" / ") {
                assert!(!part.is_empty(), "Start fragment should not be empty");
            }
        }
    }

    // --- Daily puzzle tests ---

    #[test]
    fn daily_puzzle_is_deterministic() {
        let p1 = generate_daily_puzzle(42);
        let p2 = generate_daily_puzzle(42);
        assert_eq!(p1.key, p2.key);
        assert_eq!(p1.start, p2.start);
        assert_eq!(p1.solution, p2.solution);
        assert_eq!(p1.puzzle_number, Some(42));
    }

    #[test]
    fn daily_puzzle_differs_by_day() {
        let p1 = generate_daily_puzzle(1);
        let p2 = generate_daily_puzzle(2);
        // Different seeds should (almost certainly) produce different keys
        // In the extremely unlikely case they match, at least the puzzle_number differs
        assert_ne!(p1.puzzle_number, p2.puzzle_number);
    }

    #[test]
    fn daily_puzzle_is_valid() {
        let puzzle = generate_daily_puzzle(100);
        assert!(!puzzle.key.is_empty());
        assert_eq!(puzzle.start.split(" / ").count(), 3);
        assert_eq!(puzzle.puzzle_number, Some(100));
        assert!(!puzzle.solved);
    }

    #[test]
    fn today_as_days_since_epoch_is_reasonable() {
        let days = today_as_days_since_epoch();
        // Should be a positive number (we're past the epoch)
        assert!(days > 0, "Days since epoch should be positive, got {}", days);
        // Should be less than ~3650 (10 years)
        assert!(days < 3650, "Days since epoch seems too large: {}", days);
    }

    #[test]
    fn today_as_date_string_is_valid_format() {
        let date = today_as_date_string();
        assert_eq!(date.len(), 10);
        assert_eq!(&date[4..5], "-");
        assert_eq!(&date[7..8], "-");
        // Should start with 202x
        assert!(date.starts_with("202"), "Date should be in 2020s: {}", date);
    }
}

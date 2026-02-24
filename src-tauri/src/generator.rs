use std::collections::{HashMap, HashSet};
use std::sync::OnceLock;

use rand::seq::SliceRandom;
use rand::thread_rng;
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

/// Selects a random key from the dictionary, optionally excluding a specific key.
fn get_random_key(exclude_key: Option<&str>) -> String {
    let dict = get_dictionary();
    let keys: Vec<&String> = dict
        .keys()
        .filter(|k| {
            let entry = &dict[k.as_str()];
            entry.prefix.len() + entry.suffix.len() >= 3
        })
        .filter(|k| match exclude_key {
            Some(prev) => k.as_str() != prev,
            None => true,
        })
        .collect();

    let mut rng = thread_rng();
    keys.choose(&mut rng)
        .expect("Dictionary should have valid keys")
        .to_string()
}

/// Selects 3 unique words for the given key, mixing prefix and suffix when possible.
fn select_three_words(key: &str) -> Vec<String> {
    let dict = get_dictionary();
    let entry = &dict[key];
    let mut rng = thread_rng();
    let mut selected: HashSet<String> = HashSet::new();

    // Try to get one from prefix (words ending with key)
    if !entry.prefix.is_empty() {
        if let Some(word) = entry.prefix.choose(&mut rng) {
            selected.insert(word.clone());
        }
    }

    // Try to get one from suffix (words starting with key)
    if !entry.suffix.is_empty() && selected.len() < 3 {
        let mut attempts = 0;
        while attempts < 10 {
            if let Some(word) = entry.suffix.choose(&mut rng) {
                if selected.insert(word.clone()) {
                    break;
                }
            }
            attempts += 1;
        }
    }

    // Fill remaining from either array
    let combined: Vec<&String> = entry.prefix.iter().chain(entry.suffix.iter()).collect();
    let mut attempts = 0;
    while selected.len() < 3 && attempts < 100 {
        if let Some(word) = combined.choose(&mut rng) {
            selected.insert((*word).clone());
        }
        attempts += 1;
    }

    selected.into_iter().collect()
}

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
    }
}

/// Generates a new puzzle, optionally excluding a previous key to prevent repeats.
pub fn generate_puzzle(exclude_key: Option<&str>) -> Puzzle {
    loop {
        let key = get_random_key(exclude_key);
        let words = select_three_words(&key);
        if words.len() == 3 {
            return build_puzzle(&key, &words);
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
}

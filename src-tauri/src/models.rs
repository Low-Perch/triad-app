use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PuzzleState {
    Start,
    Clue,
    Solution,
}

impl Default for PuzzleState {
    fn default() -> Self {
        PuzzleState::Start
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle {
    pub key: String,
    pub clue: String,
    pub start: String,
    pub solved: bool,
    pub solution: String,
    pub state: PuzzleState,
    #[serde(default)]
    pub puzzle_number: Option<u32>,
}

impl Default for Puzzle {
    fn default() -> Self {
        Puzzle {
            key: "FIRM".to_string(),
            solved: false,
            state: PuzzleState::Start,
            start: "WARE / REAF / CON".to_string(),
            clue: "WARE____ / REAF / CON".to_string(),
            solution: "FIRMWARE / REAFFIRM / CONFIRM".to_string(),
            puzzle_number: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum InputState {
    Edit,
    Correct,
    Incorrect,
}

impl Default for InputState {
    fn default() -> Self {
        InputState::Edit
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Input {
    pub length: usize,
    pub state: InputState,
    pub disabled: bool,
    pub keys: Vec<String>,
    pub last_position_locked: bool,
}

impl Default for Input {
    fn default() -> Self {
        Input {
            length: 4,
            disabled: false,
            keys: vec!["".to_string(), "".to_string(), "".to_string(), "".to_string()],
            state: InputState::Edit,
            last_position_locked: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Clue {
    pub id: String,
    pub note: String,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Clues {
    pub clues: Vec<Clue>,
    pub used: u32,
    pub available: bool,
}

impl Default for Clues {
    fn default() -> Self {
        Clues {
            clues: vec![
                Clue {
                    id: "position".to_string(),
                    active: false,
                    note: "Reveal 1 position".to_string(),
                },
                Clue {
                    id: "letter".to_string(),
                    active: false,
                    note: "Reveal last letter".to_string(),
                },
                Clue {
                    id: "50/50".to_string(),
                    active: false,
                    note: "50/50".to_string(),
                },
                Clue {
                    id: "solve".to_string(),
                    active: false,
                    note: "Reveal answer".to_string(),
                },
            ],
            used: 0,
            available: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Keys {
    pub disabled_keys: Vec<String>,
    pub keys_disabled: bool,
}

impl Default for Keys {
    fn default() -> Self {
        Keys {
            disabled_keys: vec![],
            keys_disabled: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    pub played: u32,
    pub solved: u32,
    pub current_streak: u32,
    pub best_streak: u32,
    pub best_time: Option<i64>,
    pub solve_times: Vec<i64>,
    pub started_at: Option<i64>,
    #[serde(default)]
    pub guess_distribution: Vec<u32>,
    #[serde(default)]
    pub solve_clue_count: u32,
}

impl Default for Stats {
    fn default() -> Self {
        Stats {
            played: 0,
            solved: 0,
            current_streak: 0,
            best_streak: 0,
            best_time: None,
            solve_times: vec![],
            started_at: None,
            guess_distribution: vec![0; 6],
            solve_clue_count: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GameState {
    pub puzzle: Puzzle,
    pub input: Input,
    pub clues: Clues,
    pub keys: Keys,
    pub stats: Stats,
    #[serde(default)]
    pub puzzle_date: Option<String>,
    #[serde(default)]
    pub guesses: u32,
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            puzzle: Puzzle::default(),
            input: Input::default(),
            clues: Clues::default(),
            keys: Keys::default(),
            stats: Stats::default(),
            puzzle_date: None,
            guesses: 0,
        }
    }
}

/// Response from submit_solution command
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubmitResult {
    pub solved: bool,
    pub exhausted: bool,
    pub guesses: u32,
    pub input_state: InputState,
    pub puzzle_state: PuzzleState,
    pub stats: Stats,
}

/// Response from activate_clue command — returns all affected state slices
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClueResult {
    pub clues: Clues,
    pub input: Input,
    pub puzzle: Puzzle,
    pub keys: Keys,
    pub stats: Option<Stats>,
}

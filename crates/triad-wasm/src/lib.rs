use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use triad_core::engine;
use triad_core::models::GameState;

#[wasm_bindgen(start)]
fn start() {
    console_error_panic_hook::set_once();
}

thread_local! {
    static STATE: RefCell<GameState> = RefCell::new(GameState::default());
}

fn with_state<F, R>(f: F) -> R
where
    F: FnOnce(&mut GameState) -> R,
{
    STATE.with(|s| f(&mut s.borrow_mut()))
}

#[wasm_bindgen]
pub fn init_game(saved_json: Option<String>) -> Result<JsValue, JsError> {
    let saved: Option<GameState> = saved_json
        .and_then(|s| serde_json::from_str(&s).ok());
    let now_secs = (js_sys::Date::now() / 1000.0) as u64;
    let result = engine::init_game(saved, now_secs);
    with_state(|s| *s = result.clone());
    serde_wasm_bindgen::to_value(&result).map_err(|e| JsError::new(&e.to_string()))
}

#[wasm_bindgen]
pub fn add_key(key: &str) -> Result<JsValue, JsError> {
    with_state(|s| {
        let input = engine::add_key(s, key);
        serde_wasm_bindgen::to_value(&input).map_err(|e| JsError::new(&e.to_string()))
    })
}

#[wasm_bindgen]
pub fn remove_key() -> Result<JsValue, JsError> {
    with_state(|s| {
        let input = engine::remove_key(s);
        serde_wasm_bindgen::to_value(&input).map_err(|e| JsError::new(&e.to_string()))
    })
}

#[wasm_bindgen]
pub fn submit_solution() -> Result<JsValue, JsError> {
    with_state(|s| {
        let result = engine::submit_solution(s);
        serde_wasm_bindgen::to_value(&result).map_err(|e| JsError::new(&e.to_string()))
    })
}

#[wasm_bindgen]
pub fn activate_clue(clue_id: &str) -> Result<JsValue, JsError> {
    with_state(|s| {
        let result = engine::activate_clue(s, clue_id);
        serde_wasm_bindgen::to_value(&result).map_err(|e| JsError::new(&e.to_string()))
    })
}

#[wasm_bindgen]
pub fn save_game() -> Result<String, JsError> {
    with_state(|s| {
        serde_json::to_string(s).map_err(|e| JsError::new(&e.to_string()))
    })
}

#[wasm_bindgen]
pub fn new_game() -> Result<JsValue, JsError> {
    with_state(|s| {
        let result = engine::new_game(s);
        serde_wasm_bindgen::to_value(&result).map_err(|e| JsError::new(&e.to_string()))
    })
}

#[wasm_bindgen]
pub fn clear_input() -> Result<JsValue, JsError> {
    with_state(|s| {
        let input = engine::clear_input(s);
        serde_wasm_bindgen::to_value(&input).map_err(|e| JsError::new(&e.to_string()))
    })
}

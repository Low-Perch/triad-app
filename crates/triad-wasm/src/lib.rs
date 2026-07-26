use std::cell::RefCell;
use serde::Serialize;
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

/// Serialize with JSON-compatible semantics (maps as plain objects, None as
/// null) so web results match the Tauri bridge's serde_json output.
fn to_js<T: Serialize>(value: &T) -> Result<JsValue, JsError> {
    value
        .serialize(&serde_wasm_bindgen::Serializer::json_compatible())
        .map_err(|e| JsError::new(&e.to_string()))
}

/// Unix timestamp shifted by the local UTC offset so the engine's day
/// boundaries fall at local midnight (getTimezoneOffset returns
/// UTC-minus-local, in minutes).
fn local_now_secs() -> u64 {
    let offset_secs = js_sys::Date::new_0().get_timezone_offset() * 60.0;
    (js_sys::Date::now() / 1000.0 - offset_secs).max(0.0) as u64
}

#[wasm_bindgen]
pub fn init_game(saved_json: Option<String>) -> Result<JsValue, JsError> {
    let saved: Option<GameState> = saved_json
        .and_then(|s| serde_json::from_str(&s).ok());
    let result = engine::init_game(saved, local_now_secs());
    with_state(|s| *s = result.clone());
    to_js(&result)
}

#[wasm_bindgen]
pub fn add_key(key: &str) -> Result<JsValue, JsError> {
    with_state(|s| to_js(&engine::add_key(s, key)))
}

#[wasm_bindgen]
pub fn remove_key() -> Result<JsValue, JsError> {
    with_state(|s| to_js(&engine::remove_key(s)))
}

#[wasm_bindgen]
pub fn submit_solution() -> Result<JsValue, JsError> {
    with_state(|s| to_js(&engine::submit_solution(s)))
}

#[wasm_bindgen]
pub fn activate_clue(clue_id: &str) -> Result<JsValue, JsError> {
    with_state(|s| to_js(&engine::activate_clue(s, clue_id)))
}

#[wasm_bindgen]
pub fn save_game() -> Result<String, JsError> {
    with_state(|s| {
        serde_json::to_string(s).map_err(|e| JsError::new(&e.to_string()))
    })
}

#[wasm_bindgen]
pub fn new_game() -> Result<JsValue, JsError> {
    with_state(|s| to_js(&engine::new_game(s)))
}

#[wasm_bindgen]
pub fn archive_game(date: &str) -> Result<JsValue, JsError> {
    with_state(|s| {
        let result =
            engine::archive_game(s, date, local_now_secs()).map_err(|e| JsError::new(&e))?;
        to_js(&result)
    })
}

#[wasm_bindgen]
pub fn resume_daily() -> Result<JsValue, JsError> {
    with_state(|s| to_js(&engine::resume_daily(s)))
}

#[wasm_bindgen]
pub fn clear_input() -> Result<JsValue, JsError> {
    with_state(|s| to_js(&engine::clear_input(s)))
}

#[wasm_bindgen]
pub fn get_history() -> Result<JsValue, JsError> {
    with_state(|s| to_js(&engine::get_history(s)))
}

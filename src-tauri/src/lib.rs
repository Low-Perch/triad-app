use std::sync::Mutex;

mod commands;
mod game;
mod generator;
mod models;
mod persistence;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .manage(Mutex::new(models::GameState::default()))
        .invoke_handler(tauri::generate_handler![
            commands::init_game,
            commands::add_key,
            commands::remove_key,
            commands::submit_solution,
            commands::activate_clue,
            commands::save_game,
            commands::new_game,
            commands::clear_input,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

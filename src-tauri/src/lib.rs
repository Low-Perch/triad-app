use std::sync::Mutex;

mod commands;
mod persistence;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_process::init())
        .manage(Mutex::new(triad_core::models::GameState::default()))
        .invoke_handler(tauri::generate_handler![
            commands::init_game,
            commands::add_key,
            commands::remove_key,
            commands::submit_solution,
            commands::activate_clue,
            commands::save_game,
            commands::new_game,
            commands::archive_game,
            commands::resume_daily,
            commands::clear_input,
            commands::get_history,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

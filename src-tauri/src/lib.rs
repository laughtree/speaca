// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod constants;
mod commands;
mod services;
mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::document::save_content,
            commands::document::load_content,
            commands::document::save_meta,
            commands::document::load_meta,
            commands::document::create_new_work,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

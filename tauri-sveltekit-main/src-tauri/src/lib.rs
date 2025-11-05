// Declare modules
mod models;
mod database;
mod parser;
mod scanner;
mod thumbnails;
mod commands;

use std::sync::{Arc, Mutex};
use commands::AppState;

// Test command to verify IPC bridge
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// Simple test command that returns a FileEntry struct
#[tauri::command]
fn get_test_file() -> models::FileEntry {
    models::FileEntry {
        id: "test123".to_string(),
        path: "/path/to/test.png".to_string(),
        name: "test.png".to_string(),
        file_type: "image".to_string(),
        mtime: 1234567890.0,
        has_workflow: true,
        is_favorite: false,
        prompt_preview: Some("A beautiful landscape".to_string()),
        sampler_names: Some("euler, dpm++".to_string()),
        dimensions: Some("1024x1024".to_string()),
        duration: None,
        sampler_count: 2,
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_state = Arc::new(Mutex::new(AppState::new()));
    
    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            // Test commands
            greet,
            get_test_file,
            // Gallery commands
            commands::initialize_gallery,
            commands::get_files,
            commands::get_file_by_id,
            commands::get_workflow_metadata,
            commands::toggle_favorite,
            commands::batch_favorite,
            commands::delete_file,
            commands::batch_delete,
            commands::sync_files,
            commands::get_stats,
            commands::get_thumbnail_path,
            commands::health_check,
            commands::get_filter_options,
            // New commands
            commands::rename_file,
            commands::move_files,
            commands::search_files,
            commands::get_files_filtered,
            commands::create_folder,
            commands::get_config,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Declare modules
mod models;
mod database;
mod parser;

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
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, get_test_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

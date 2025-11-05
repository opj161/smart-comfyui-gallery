// SmartGallery - Tauri Application
// Core modules
mod models;
mod database;

// Re-export models for use in commands
pub use models::*;

use sqlx::SqlitePool;
use std::sync::Mutex;
use tauri::State;

// Global database connection pool
pub struct AppState {
    pub db_pool: Mutex<Option<SqlitePool>>,
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// Initialize database command
#[tauri::command]
async fn init_database(db_path: String, state: State<'_, AppState>) -> Result<String, String> {
    let path = std::path::PathBuf::from(db_path);
    
    match database::init_db(&path).await {
        Ok(pool) => {
            // Store the pool in application state
            let mut db_pool = state.db_pool.lock().unwrap();
            *db_pool = Some(pool);
            Ok("Database initialized successfully".to_string())
        }
        Err(e) => Err(format!("Failed to initialize database: {}", e)),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            db_pool: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![greet, init_database])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

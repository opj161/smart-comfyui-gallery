// Tauri commands for SmartGallery
// Exposes all backend functionality to the frontend

use std::sync::{Arc, Mutex};
use std::path::PathBuf;
use sqlx::SqlitePool;
use tauri::State;

use crate::models::*;
use crate::database;
use crate::scanner::{ScannerConfig, full_sync};
use crate::thumbnails::ThumbnailConfig;

/// Global application state
pub struct AppState {
    pub db_pool: Option<SqlitePool>,
    pub output_path: Option<PathBuf>,
    pub input_path: Option<PathBuf>,
    pub scanner_config: Option<ScannerConfig>,
    pub thumbnail_config: Option<ThumbnailConfig>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            db_pool: None,
            output_path: None,
            input_path: None,
            scanner_config: None,
            thumbnail_config: None,
        }
    }
}

/// Initialize the gallery with paths and database
#[tauri::command]
pub async fn initialize_gallery(
    output_path: String,
    input_path: Option<String>,
    state: State<'_, Arc<Mutex<AppState>>>,
) -> Result<String, String> {
    let output_path_buf = PathBuf::from(&output_path);
    
    // Create database directory
    let db_dir = output_path_buf.join("smartgallery_cache");
    std::fs::create_dir_all(&db_dir)
        .map_err(|e| format!("Failed to create database directory: {}", e))?;
    
    let db_path = db_dir.join("gallery_cache.sqlite");
    
    // Initialize database
    let pool = database::init_db(&db_path).await?;
    
    // Set up scanner config
    let scanner_config = ScannerConfig::new(output_path_buf.clone());
    
    // Set up thumbnail config
    let thumbnail_cache_dir = output_path_buf.join("thumbnails_cache");
    let thumbnail_config = ThumbnailConfig::new(thumbnail_cache_dir);
    
    // Update state
    let mut app_state = state.lock().unwrap();
    app_state.db_pool = Some(pool);
    app_state.output_path = Some(output_path_buf);
    app_state.input_path = input_path.map(PathBuf::from);
    app_state.scanner_config = Some(scanner_config);
    app_state.thumbnail_config = Some(thumbnail_config);
    
    Ok("Gallery initialized successfully".to_string())
}

/// Get paginated list of files with filters
#[tauri::command]
pub async fn get_files(
    folder_key: Option<String>,
    page: usize,
    per_page: usize,
    state: State<'_, Arc<Mutex<AppState>>>,
) -> Result<PaginatedFiles, String> {
    let app_state = state.lock().unwrap();
    let pool = app_state.db_pool.as_ref()
        .ok_or("Database not initialized")?;
    
    // For now, return a simple implementation
    // Full implementation would include folder filtering, search, etc.
    let offset = page * per_page;
    
    let files = sqlx::query(
        "SELECT id, path, name, type, mtime, has_workflow, is_favorite, 
                prompt_preview, sampler_names, dimensions, duration,
                (SELECT COUNT(*) FROM workflow_metadata WHERE file_id = files.id) as sampler_count
         FROM files
         ORDER BY mtime DESC
         LIMIT ? OFFSET ?"
    )
    .bind(per_page as i64)
    .bind(offset as i64)
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Failed to fetch files: {}", e))?;
    
    let file_entries: Vec<FileEntry> = files.into_iter().map(|row| FileEntry {
        id: row.get("id"),
        path: row.get("path"),
        name: row.get("name"),
        file_type: row.get("type"),
        mtime: row.get("mtime"),
        has_workflow: row.get::<i32, _>("has_workflow") != 0,
        is_favorite: row.get::<i32, _>("is_favorite") != 0,
        prompt_preview: row.get("prompt_preview"),
        sampler_names: row.get("sampler_names"),
        dimensions: row.get("dimensions"),
        duration: row.get("duration"),
        sampler_count: row.get::<i32, _>("sampler_count"),
    }).collect();
    
    let total_count = database::get_file_count(pool).await? as usize;
    let has_more = (offset + file_entries.len()) < total_count;
    
    Ok(PaginatedFiles {
        files: file_entries,
        total_count,
        has_more,
    })
}

/// Get a single file by ID
#[tauri::command]
pub async fn get_file_by_id(
    file_id: String,
    state: State<'_, Arc<Mutex<AppState>>>,
) -> Result<Option<FileEntry>, String> {
    let app_state = state.lock().unwrap();
    let pool = app_state.db_pool.as_ref()
        .ok_or("Database not initialized")?;
    
    database::get_file_by_id(pool, &file_id).await
}

/// Get workflow metadata for a file
#[tauri::command]
pub async fn get_workflow_metadata(
    file_id: String,
    state: State<'_, Arc<Mutex<AppState>>>,
) -> Result<Vec<WorkflowMetadata>, String> {
    let app_state = state.lock().unwrap();
    let pool = app_state.db_pool.as_ref()
        .ok_or("Database not initialized")?;
    
    database::get_workflow_metadata(pool, &file_id).await
}

/// Toggle favorite status for a file
#[tauri::command]
pub async fn toggle_favorite(
    file_id: String,
    state: State<'_, Arc<Mutex<AppState>>>,
) -> Result<bool, String> {
    let app_state = state.lock().unwrap();
    let pool = app_state.db_pool.as_ref()
        .ok_or("Database not initialized")?;
    
    database::toggle_favorite(pool, &file_id).await
}

/// Set favorite status for multiple files
#[tauri::command]
pub async fn batch_favorite(
    file_ids: Vec<String>,
    favorite: bool,
    state: State<'_, Arc<Mutex<AppState>>>,
) -> Result<(), String> {
    let app_state = state.lock().unwrap();
    let pool = app_state.db_pool.as_ref()
        .ok_or("Database not initialized")?;
    
    database::batch_set_favorite(pool, &file_ids, favorite).await
}

/// Delete a single file
#[tauri::command]
pub async fn delete_file(
    file_id: String,
    state: State<'_, Arc<Mutex<AppState>>>,
) -> Result<(), String> {
    let app_state = state.lock().unwrap();
    let pool = app_state.db_pool.as_ref()
        .ok_or("Database not initialized")?;
    
    // Get file path before deleting from DB
    let file = database::get_file_by_id(pool, &file_id).await?;
    
    if let Some(file_entry) = file {
        // Delete from filesystem
        let path = PathBuf::from(&file_entry.path);
        if path.exists() {
            std::fs::remove_file(&path)
                .map_err(|e| format!("Failed to delete file: {}", e))?;
        }
        
        // Delete from database
        database::delete_file(pool, &file_id).await?;
    }
    
    Ok(())
}

/// Delete multiple files
#[tauri::command]
pub async fn batch_delete(
    file_ids: Vec<String>,
    state: State<'_, Arc<Mutex<AppState>>>,
) -> Result<(), String> {
    let app_state = state.lock().unwrap();
    let pool = app_state.db_pool.as_ref()
        .ok_or("Database not initialized")?;
    
    // Get file paths before deleting from DB
    for file_id in &file_ids {
        let file = database::get_file_by_id(pool, file_id).await?;
        if let Some(file_entry) = file {
            let path = PathBuf::from(&file_entry.path);
            if path.exists() {
                let _ = std::fs::remove_file(&path); // Ignore errors, continue with DB deletion
            }
        }
    }
    
    // Delete from database
    database::delete_files(pool, &file_ids).await
}

/// Sync files from disk to database
#[tauri::command]
pub async fn sync_files(
    app_handle: tauri::AppHandle,
    state: State<'_, Arc<Mutex<AppState>>>,
) -> Result<String, String> {
    let (pool, config) = {
        let app_state = state.lock().unwrap();
        let pool = app_state.db_pool.as_ref()
            .ok_or("Database not initialized")?
            .clone();
        let config = app_state.scanner_config.as_ref()
            .ok_or("Scanner not initialized")?
            .clone();
        (pool, config)
    };
    
    // Perform sync with progress events
    let stats = full_sync(&pool, &config, Some(Box::new(move |progress| {
        let _ = app_handle.emit("sync-progress", &progress);
    }))).await?;
    
    // Emit completion event
    let _ = app_handle.emit("sync-complete", &stats);
    
    Ok(format!("Sync complete: {} files processed", stats.total_processed))
}

/// Get database statistics
#[tauri::command]
pub async fn get_stats(
    state: State<'_, Arc<Mutex<AppState>>>,
) -> Result<serde_json::Value, String> {
    let app_state = state.lock().unwrap();
    let pool = app_state.db_pool.as_ref()
        .ok_or("Database not initialized")?;
    
    let total_files = database::get_file_count(pool).await?;
    
    let favorites_count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM files WHERE is_favorite = 1"
    )
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Failed to count favorites: {}", e))?;
    
    let with_workflow_count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM files WHERE has_workflow = 1"
    )
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Failed to count workflows: {}", e))?;
    
    Ok(serde_json::json!({
        "total_files": total_files,
        "favorites": favorites_count,
        "with_workflow": with_workflow_count,
    }))
}

/// Get thumbnail path for a file
#[tauri::command]
pub async fn get_thumbnail_path(
    file_id: String,
    state: State<'_, Arc<Mutex<AppState>>>,
) -> Result<Option<String>, String> {
    let app_state = state.lock().unwrap();
    let pool = app_state.db_pool.as_ref()
        .ok_or("Database not initialized")?;
    let thumbnail_config = app_state.thumbnail_config.as_ref()
        .ok_or("Thumbnail config not initialized")?;
    
    // Get file from database
    let file = database::get_file_by_id(pool, &file_id).await?;
    
    if let Some(file_entry) = file {
        let file_path = PathBuf::from(&file_entry.path);
        
        // Check if thumbnail exists
        let thumb_path = crate::thumbnails::get_thumbnail_path(&file_path, thumbnail_config);
        
        if let Some(path) = thumb_path {
            return Ok(Some(path.to_string_lossy().to_string()));
        }
        
        // Try to create thumbnail
        match crate::thumbnails::get_or_create_thumbnail(&file_path, &file_entry.file_type, thumbnail_config) {
            Ok(path) => Ok(Some(path.to_string_lossy().to_string())),
            Err(_) => Ok(None),
        }
    } else {
        Ok(None)
    }
}

/// Health check
#[tauri::command]
pub async fn health_check(
    state: State<'_, Arc<Mutex<AppState>>>,
) -> Result<String, String> {
    let app_state = state.lock().unwrap();
    
    if app_state.db_pool.is_none() {
        return Ok("not_initialized".to_string());
    }
    
    Ok("healthy".to_string())
}

/// Get filter options (models, samplers, etc.)
#[tauri::command]
pub async fn get_filter_options(
    state: State<'_, Arc<Mutex<AppState>>>,
) -> Result<FilterOptions, String> {
    let app_state = state.lock().unwrap();
    let pool = app_state.db_pool.as_ref()
        .ok_or("Database not initialized")?;
    
    // Get distinct models
    let models = sqlx::query_scalar::<_, String>(
        "SELECT DISTINCT model_name FROM workflow_metadata 
         WHERE model_name IS NOT NULL 
         ORDER BY model_name"
    )
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Failed to fetch models: {}", e))?;
    
    // Get distinct samplers
    let samplers = sqlx::query_scalar::<_, String>(
        "SELECT DISTINCT sampler_name FROM workflow_metadata 
         WHERE sampler_name IS NOT NULL 
         ORDER BY sampler_name"
    )
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Failed to fetch samplers: {}", e))?;
    
    // Get distinct schedulers
    let schedulers = sqlx::query_scalar::<_, String>(
        "SELECT DISTINCT scheduler FROM workflow_metadata 
         WHERE scheduler IS NOT NULL 
         ORDER BY scheduler"
    )
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Failed to fetch schedulers: {}", e))?;
    
    // Get file extensions
    let extensions = vec![
        ".png".to_string(),
        ".jpg".to_string(),
        ".jpeg".to_string(),
        ".webp".to_string(),
        ".gif".to_string(),
        ".mp4".to_string(),
        ".avi".to_string(),
        ".mov".to_string(),
    ];
    
    Ok(FilterOptions {
        models,
        samplers,
        schedulers,
        extensions,
        prefixes: vec![],
    })
}

// Database layer for SmartGallery
// Handles SQLite connection pooling, schema initialization, and CRUD operations

use sqlx::{sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions}, Row};
use std::path::Path;
use crate::models::{FileEntry, WorkflowMetadata};

/// Initialize database with schema and indices
pub async fn init_db(db_path: &Path) -> Result<SqlitePool, String> {
    // Ensure parent directory exists
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create database directory: {}", e))?;
    }

    // Create connection pool
    let options = SqliteConnectOptions::new()
        .filename(db_path)
        .create_if_missing(true);
    
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await
        .map_err(|e| format!("Failed to connect to database: {}", e))?;

    // Enable WAL mode for better concurrency
    sqlx::query("PRAGMA journal_mode=WAL")
        .execute(&pool)
        .await
        .map_err(|e| format!("Failed to enable WAL mode: {}", e))?;
    
    sqlx::query("PRAGMA synchronous=NORMAL")
        .execute(&pool)
        .await
        .map_err(|e| format!("Failed to set synchronous mode: {}", e))?;
    
    sqlx::query("PRAGMA cache_size=-64000")
        .execute(&pool)
        .await
        .map_err(|e| format!("Failed to set cache size: {}", e))?;
    
    sqlx::query("PRAGMA temp_store=MEMORY")
        .execute(&pool)
        .await
        .map_err(|e| format!("Failed to set temp store: {}", e))?;

    // Create tables
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS files (
            id TEXT PRIMARY KEY,
            path TEXT NOT NULL UNIQUE,
            mtime REAL NOT NULL,
            name TEXT NOT NULL,
            type TEXT,
            duration TEXT,
            dimensions TEXT,
            has_workflow INTEGER,
            is_favorite INTEGER DEFAULT 0,
            prompt_preview TEXT,
            sampler_names TEXT
        )"
    )
    .execute(&pool)
    .await
    .map_err(|e| format!("Failed to create files table: {}", e))?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS workflow_metadata (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            file_id TEXT NOT NULL,
            sampler_index INTEGER DEFAULT 0,
            model_name TEXT,
            sampler_name TEXT,
            scheduler TEXT,
            cfg REAL,
            steps INTEGER,
            positive_prompt TEXT,
            negative_prompt TEXT,
            width INTEGER,
            height INTEGER,
            FOREIGN KEY (file_id) REFERENCES files(id) ON DELETE CASCADE
        )"
    )
    .execute(&pool)
    .await
    .map_err(|e| format!("Failed to create workflow_metadata table: {}", e))?;

    // Create indices for workflow metadata
    let workflow_indices = [
        "CREATE UNIQUE INDEX IF NOT EXISTS idx_file_sampler ON workflow_metadata(file_id, sampler_index)",
        "CREATE INDEX IF NOT EXISTS idx_model_name ON workflow_metadata(model_name)",
        "CREATE INDEX IF NOT EXISTS idx_sampler_name ON workflow_metadata(sampler_name)",
        "CREATE INDEX IF NOT EXISTS idx_scheduler ON workflow_metadata(scheduler)",
        "CREATE INDEX IF NOT EXISTS idx_cfg ON workflow_metadata(cfg)",
        "CREATE INDEX IF NOT EXISTS idx_steps ON workflow_metadata(steps)",
        "CREATE INDEX IF NOT EXISTS idx_width ON workflow_metadata(width)",
        "CREATE INDEX IF NOT EXISTS idx_height ON workflow_metadata(height)",
        "CREATE INDEX IF NOT EXISTS idx_file_id ON workflow_metadata(file_id)",
    ];

    for index_sql in &workflow_indices {
        sqlx::query(index_sql)
            .execute(&pool)
            .await
            .map_err(|e| format!("Failed to create index: {}", e))?;
    }

    // Create indices for files table (critical for performance)
    let file_indices = [
        "CREATE INDEX IF NOT EXISTS idx_files_name ON files(name)",
        "CREATE INDEX IF NOT EXISTS idx_files_mtime ON files(mtime DESC)",
        "CREATE INDEX IF NOT EXISTS idx_files_type ON files(type)",
        "CREATE INDEX IF NOT EXISTS idx_files_favorite ON files(is_favorite)",
        "CREATE INDEX IF NOT EXISTS idx_files_path ON files(path)",
    ];

    for index_sql in &file_indices {
        sqlx::query(index_sql)
            .execute(&pool)
            .await
            .map_err(|e| format!("Failed to create index: {}", e))?;
    }

    Ok(pool)
}

/// Insert or update a file entry
pub async fn upsert_file(pool: &SqlitePool, file: &FileEntry) -> Result<(), String> {
    sqlx::query(
        "INSERT OR REPLACE INTO files 
         (id, path, name, type, mtime, has_workflow, is_favorite, prompt_preview, sampler_names, dimensions, duration)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&file.id)
    .bind(&file.path)
    .bind(&file.name)
    .bind(&file.file_type)
    .bind(file.mtime)
    .bind(file.has_workflow as i32)
    .bind(file.is_favorite as i32)
    .bind(&file.prompt_preview)
    .bind(&file.sampler_names)
    .bind(&file.dimensions)
    .bind(&file.duration)
    .execute(pool)
    .await
    .map_err(|e| format!("Failed to insert file: {}", e))?;
    
    Ok(())
}

/// Get file by ID
pub async fn get_file_by_id(pool: &SqlitePool, file_id: &str) -> Result<Option<FileEntry>, String> {
    let row = sqlx::query(
        "SELECT id, path, name, type, mtime, has_workflow, is_favorite, 
                prompt_preview, sampler_names, dimensions, duration,
                (SELECT COUNT(*) FROM workflow_metadata WHERE file_id = files.id) as sampler_count
         FROM files WHERE id = ?"
    )
    .bind(file_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("Failed to fetch file: {}", e))?;

    match row {
        Some(row) => Ok(Some(FileEntry {
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
        })),
        None => Ok(None),
    }
}

/// Delete file by ID
pub async fn delete_file(pool: &SqlitePool, file_id: &str) -> Result<(), String> {
    sqlx::query("DELETE FROM files WHERE id = ?")
        .bind(file_id)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to delete file: {}", e))?;
    
    Ok(())
}

/// Delete multiple files
pub async fn delete_files(pool: &SqlitePool, file_ids: &[String]) -> Result<(), String> {
    if file_ids.is_empty() {
        return Ok(());
    }

    let placeholders = file_ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query_str = format!("DELETE FROM files WHERE id IN ({})", placeholders);
    
    let mut query = sqlx::query(&query_str);
    for id in file_ids {
        query = query.bind(id);
    }
    
    query.execute(pool)
        .await
        .map_err(|e| format!("Failed to delete files: {}", e))?;
    
    Ok(())
}

/// Toggle favorite status
pub async fn toggle_favorite(pool: &SqlitePool, file_id: &str) -> Result<bool, String> {
    let row = sqlx::query("SELECT is_favorite FROM files WHERE id = ?")
        .bind(file_id)
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Failed to fetch file: {}", e))?;
    
    let current_favorite = row.get::<i32, _>("is_favorite") != 0;
    let new_favorite = !current_favorite;
    
    sqlx::query("UPDATE files SET is_favorite = ? WHERE id = ?")
        .bind(new_favorite as i32)
        .bind(file_id)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to update favorite: {}", e))?;
    
    Ok(new_favorite)
}

/// Set favorite status for multiple files
pub async fn batch_set_favorite(pool: &SqlitePool, file_ids: &[String], favorite: bool) -> Result<(), String> {
    if file_ids.is_empty() {
        return Ok(());
    }

    let placeholders = file_ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query_str = format!("UPDATE files SET is_favorite = ? WHERE id IN ({})", placeholders);
    
    let mut query = sqlx::query(&query_str).bind(favorite as i32);
    for id in file_ids {
        query = query.bind(id);
    }
    
    query.execute(pool)
        .await
        .map_err(|e| format!("Failed to batch update favorites: {}", e))?;
    
    Ok(())
}

/// Insert workflow metadata
#[allow(dead_code)]
pub async fn insert_workflow_metadata(pool: &SqlitePool, metadata: &WorkflowMetadata) -> Result<(), String> {
    sqlx::query(
        "INSERT OR REPLACE INTO workflow_metadata 
         (file_id, sampler_index, model_name, sampler_name, scheduler, cfg, steps, 
          positive_prompt, negative_prompt, width, height)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&metadata.file_id)
    .bind(metadata.sampler_index)
    .bind(&metadata.model_name)
    .bind(&metadata.sampler_name)
    .bind(&metadata.scheduler)
    .bind(metadata.cfg)
    .bind(metadata.steps)
    .bind(&metadata.positive_prompt)
    .bind(&metadata.negative_prompt)
    .bind(metadata.width)
    .bind(metadata.height)
    .execute(pool)
    .await
    .map_err(|e| format!("Failed to insert workflow metadata: {}", e))?;
    
    Ok(())
}

/// Get workflow metadata for a file
pub async fn get_workflow_metadata(pool: &SqlitePool, file_id: &str) -> Result<Vec<WorkflowMetadata>, String> {
    let rows = sqlx::query(
        "SELECT id, file_id, sampler_index, model_name, sampler_name, scheduler, 
                cfg, steps, positive_prompt, negative_prompt, width, height
         FROM workflow_metadata WHERE file_id = ? ORDER BY sampler_index"
    )
    .bind(file_id)
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Failed to fetch workflow metadata: {}", e))?;

    let metadata = rows.into_iter().map(|row| WorkflowMetadata {
        id: Some(row.get("id")),
        file_id: row.get("file_id"),
        sampler_index: row.get("sampler_index"),
        model_name: row.get("model_name"),
        sampler_name: row.get("sampler_name"),
        scheduler: row.get("scheduler"),
        cfg: row.get("cfg"),
        steps: row.get("steps"),
        positive_prompt: row.get("positive_prompt"),
        negative_prompt: row.get("negative_prompt"),
        width: row.get("width"),
        height: row.get("height"),
    }).collect();

    Ok(metadata)
}

/// Get all file paths from database (for sync)
pub async fn get_all_file_paths(pool: &SqlitePool) -> Result<Vec<(String, f64)>, String> {
    let rows = sqlx::query("SELECT path, mtime FROM files")
        .fetch_all(pool)
        .await
        .map_err(|e| format!("Failed to fetch file paths: {}", e))?;

    let paths = rows.into_iter()
        .map(|row| (row.get("path"), row.get("mtime")))
        .collect();

    Ok(paths)
}

/// Get total file count
pub async fn get_file_count(pool: &SqlitePool) -> Result<i64, String> {
    let row = sqlx::query("SELECT COUNT(*) as count FROM files")
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Failed to count files: {}", e))?;
    
    Ok(row.get("count"))
}

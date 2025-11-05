// src-tauri/src/database.rs
//
// Database layer for SmartGallery using SQLx with SQLite
// Handles schema creation, migrations, and all database operations

use crate::models::*;
use anyhow::{Context, Result};
use sqlx::{sqlite::SqlitePool, Row, SqliteConnection};
use std::path::Path;

/// Initialize the database and create tables if they don't exist
pub async fn init_db(db_path: &Path) -> Result<SqlitePool> {
    // Create parent directory if it doesn't exist
    if let Some(parent) = db_path.parent() {
        tokio::fs::create_dir_all(parent).await
            .context("Failed to create database directory")?;
    }

    // Create connection pool
    let db_url = format!("sqlite://{}?mode=rwc", db_path.display());
    let pool = SqlitePool::connect(&db_url).await
        .context("Failed to connect to database")?;

    // Enable WAL mode and set PRAGMAs for performance
    configure_database(&pool).await?;

    // Create tables
    create_tables(&pool).await?;

    // Create indices
    create_indices(&pool).await?;

    // Run migrations
    run_migrations(&pool).await?;

    Ok(pool)
}

/// Configure database with performance PRAGMAs
async fn configure_database(pool: &SqlitePool) -> Result<()> {
    sqlx::query("PRAGMA journal_mode = WAL")
        .execute(pool)
        .await?;
    
    sqlx::query("PRAGMA synchronous = NORMAL")
        .execute(pool)
        .await?;
    
    sqlx::query("PRAGMA cache_size = -64000") // 64MB cache
        .execute(pool)
        .await?;
    
    sqlx::query("PRAGMA temp_store = MEMORY")
        .execute(pool)
        .await?;
    
    sqlx::query("PRAGMA mmap_size = 268435456") // 256MB mmap
        .execute(pool)
        .await?;

    Ok(())
}

/// Create main tables
async fn create_tables(pool: &SqlitePool) -> Result<()> {
    // Files table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS files (
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
        )
        "#,
    )
    .execute(pool)
    .await
    .context("Failed to create files table")?;

    // Workflow metadata table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS workflow_metadata (
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
        )
        "#,
    )
    .execute(pool)
    .await
    .context("Failed to create workflow_metadata table")?;

    Ok(())
}

/// Create indices for performance
async fn create_indices(pool: &SqlitePool) -> Result<()> {
    let indices = vec![
        // Workflow metadata indices
        "CREATE UNIQUE INDEX IF NOT EXISTS idx_file_sampler ON workflow_metadata(file_id, sampler_index)",
        "CREATE INDEX IF NOT EXISTS idx_model_name ON workflow_metadata(model_name)",
        "CREATE INDEX IF NOT EXISTS idx_sampler_name ON workflow_metadata(sampler_name)",
        "CREATE INDEX IF NOT EXISTS idx_scheduler ON workflow_metadata(scheduler)",
        "CREATE INDEX IF NOT EXISTS idx_cfg ON workflow_metadata(cfg)",
        "CREATE INDEX IF NOT EXISTS idx_steps ON workflow_metadata(steps)",
        "CREATE INDEX IF NOT EXISTS idx_width ON workflow_metadata(width)",
        "CREATE INDEX IF NOT EXISTS idx_height ON workflow_metadata(height)",
        "CREATE INDEX IF NOT EXISTS idx_file_id ON workflow_metadata(file_id)",
        
        // Files table indices (CRITICAL PERFORMANCE)
        "CREATE INDEX IF NOT EXISTS idx_files_name ON files(name)",
        "CREATE INDEX IF NOT EXISTS idx_files_mtime ON files(mtime DESC)",
        "CREATE INDEX IF NOT EXISTS idx_files_type ON files(type)",
        "CREATE INDEX IF NOT EXISTS idx_files_favorite ON files(is_favorite)",
        "CREATE INDEX IF NOT EXISTS idx_files_path ON files(path)",
    ];

    for index_sql in indices {
        sqlx::query(index_sql)
            .execute(pool)
            .await
            .context(format!("Failed to create index: {}", index_sql))?;
    }

    Ok(())
}

/// Run database migrations
async fn run_migrations(pool: &SqlitePool) -> Result<()> {
    // Check if prompt_preview column exists (migration from older versions)
    let has_prompt_preview = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM pragma_table_info('files') WHERE name='prompt_preview'"
    )
    .fetch_one(pool)
    .await?;

    if has_prompt_preview == 0 {
        sqlx::query("ALTER TABLE files ADD COLUMN prompt_preview TEXT")
            .execute(pool)
            .await?;
    }

    // Check if sampler_names column exists
    let has_sampler_names = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM pragma_table_info('files') WHERE name='sampler_names'"
    )
    .fetch_one(pool)
    .await?;

    if has_sampler_names == 0 {
        sqlx::query("ALTER TABLE files ADD COLUMN sampler_names TEXT")
            .execute(pool)
            .await?;
    }

    Ok(())
}

/// Insert or replace a file entry
pub async fn upsert_file(pool: &SqlitePool, file: &FileEntry) -> Result<()> {
    sqlx::query(
        r#"
        INSERT OR REPLACE INTO files 
        (id, path, mtime, name, type, duration, dimensions, has_workflow, is_favorite, prompt_preview, sampler_names)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&file.id)
    .bind(&file.path)
    .bind(file.mtime)
    .bind(&file.name)
    .bind(&file.file_type)
    .bind(&file.duration)
    .bind(&file.dimensions)
    .bind(file.has_workflow as i64)
    .bind(file.is_favorite as i64)
    .bind(&file.prompt_preview)
    .bind(&file.sampler_names)
    .execute(pool)
    .await?;

    Ok(())
}

/// Insert workflow metadata (replaces all metadata for a file_id)
pub async fn upsert_workflow_metadata(
    pool: &SqlitePool,
    file_id: &str,
    metadata_list: &[WorkflowMetadata],
) -> Result<()> {
    // Delete existing metadata for this file
    sqlx::query("DELETE FROM workflow_metadata WHERE file_id = ?")
        .bind(file_id)
        .execute(pool)
        .await?;

    // Insert new metadata
    for metadata in metadata_list {
        sqlx::query(
            r#"
            INSERT INTO workflow_metadata 
            (file_id, sampler_index, model_name, sampler_name, scheduler, cfg, steps, 
             positive_prompt, negative_prompt, width, height)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
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
        .await?;
    }

    Ok(())
}

/// Get a file by ID
pub async fn get_file_by_id(pool: &SqlitePool, id: &str) -> Result<Option<FileEntry>> {
    let row = sqlx::query(
        r#"
        SELECT id, path, name, type, mtime, has_workflow, is_favorite, 
               prompt_preview, sampler_names, dimensions, duration,
               (SELECT COUNT(*) FROM workflow_metadata WHERE file_id = files.id) as sampler_count
        FROM files WHERE id = ?
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    if let Some(row) = row {
        Ok(Some(FileEntry {
            id: row.try_get("id")?,
            path: row.try_get("path")?,
            name: row.try_get("name")?,
            file_type: row.try_get("type")?,
            mtime: row.try_get("mtime")?,
            has_workflow: row.try_get::<i64, _>("has_workflow")? != 0,
            is_favorite: row.try_get::<i64, _>("is_favorite")? != 0,
            prompt_preview: row.try_get("prompt_preview")?,
            sampler_names: row.try_get("sampler_names")?,
            dimensions: row.try_get("dimensions")?,
            duration: row.try_get("duration")?,
            sampler_count: row.try_get::<i64, _>("sampler_count")? as i32,
        }))
    } else {
        Ok(None)
    }
}

/// Delete files by paths
pub async fn delete_files_by_paths(pool: &SqlitePool, paths: &[String]) -> Result<()> {
    if paths.is_empty() {
        return Ok(());
    }

    // Build a query with placeholders
    let placeholders = vec!["?"; paths.len()].join(", ");
    let query_str = format!("DELETE FROM files WHERE path IN ({})", placeholders);

    let mut query = sqlx::query(&query_str);
    for path in paths {
        query = query.bind(path);
    }

    query.execute(pool).await?;
    Ok(())
}

/// Update favorite status
pub async fn update_favorite_status(
    pool: &SqlitePool,
    file_ids: &[String],
    is_favorite: bool,
) -> Result<()> {
    if file_ids.is_empty() {
        return Ok(());
    }

    let placeholders = vec!["?"; file_ids.len()].join(", ");
    let query_str = format!(
        "UPDATE files SET is_favorite = ? WHERE id IN ({})",
        placeholders
    );

    let mut query = sqlx::query(&query_str).bind(is_favorite as i64);
    for id in file_ids {
        query = query.bind(id);
    }

    query.execute(pool).await?;
    Ok(())
}

/// Get all file paths in the database
pub async fn get_all_file_paths(pool: &SqlitePool) -> Result<Vec<(String, f64)>> {
    let rows = sqlx::query("SELECT path, mtime FROM files")
        .fetch_all(pool)
        .await?;

    let mut result = Vec::new();
    for row in rows {
        result.push((row.try_get("path")?, row.try_get("mtime")?));
    }

    Ok(result)
}

/// Query files with filtering and pagination
pub async fn query_files(
    pool: &SqlitePool,
    filters: &FilterOptions,
    pagination: &PaginationParams,
    folder_path: Option<&str>,
) -> Result<PaginatedResult<FileEntry>> {
    // Build WHERE clause
    let mut where_clauses = Vec::new();
    let mut params: Vec<Box<dyn sqlx::Encode<'_, sqlx::Sqlite> + Send>> = Vec::new();

    // Folder filter
    if let Some(folder) = folder_path {
        where_clauses.push("path LIKE ?");
        params.push(Box::new(format!("{}%", folder)));
    }

    // Search filter
    if let Some(search) = &filters.search {
        if !search.is_empty() {
            where_clauses.push("name LIKE ?");
            params.push(Box::new(format!("%{}%", search)));
        }
    }

    // Type filter
    if !filters.file_types.is_empty() {
        let placeholders = vec!["?"; filters.file_types.len()].join(", ");
        where_clauses.push(&format!("type IN ({})", placeholders));
        for t in &filters.file_types {
            params.push(Box::new(t.clone()));
        }
    }

    // Favorites filter
    if filters.favorites_only {
        where_clauses.push("is_favorite = 1");
    }

    // Has workflow filter
    if let Some(has_workflow) = filters.has_workflow {
        where_clauses.push("has_workflow = ?");
        params.push(Box::new(has_workflow as i64));
    }

    // Build WHERE clause string
    let where_str = if where_clauses.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", where_clauses.join(" AND "))
    };

    // Build ORDER BY clause
    let order_by = format!(
        "ORDER BY {} {}",
        pagination.sort_by, pagination.sort_order
    );

    // Count total matching files
    let count_query = format!("SELECT COUNT(*) as count FROM files {}", where_str);
    let total: i64 = sqlx::query_scalar(&count_query)
        .fetch_one(pool)
        .await?;

    // Calculate pagination
    let offset = (pagination.page - 1) * pagination.per_page;
    let total_pages = ((total as f64) / (pagination.per_page as f64)).ceil() as i32;

    // Query files with pagination
    let query_str = format!(
        r#"
        SELECT id, path, name, type, mtime, has_workflow, is_favorite,
               prompt_preview, sampler_names, dimensions, duration,
               (SELECT COUNT(*) FROM workflow_metadata WHERE file_id = files.id) as sampler_count
        FROM files
        {}
        {}
        LIMIT ? OFFSET ?
        "#,
        where_str, order_by
    );

    let rows = sqlx::query(&query_str)
        .bind(pagination.per_page)
        .bind(offset)
        .fetch_all(pool)
        .await?;

    let mut items = Vec::new();
    for row in rows {
        items.push(FileEntry {
            id: row.try_get("id")?,
            path: row.try_get("path")?,
            name: row.try_get("name")?,
            file_type: row.try_get("type")?,
            mtime: row.try_get("mtime")?,
            has_workflow: row.try_get::<i64, _>("has_workflow")? != 0,
            is_favorite: row.try_get::<i64, _>("is_favorite")? != 0,
            prompt_preview: row.try_get("prompt_preview")?,
            sampler_names: row.try_get("sampler_names")?,
            dimensions: row.try_get("dimensions")?,
            duration: row.try_get("duration")?,
            sampler_count: row.try_get::<i64, _>("sampler_count")? as i32,
        });
    }

    Ok(PaginatedResult {
        items,
        total,
        page: pagination.page,
        per_page: pagination.per_page,
        total_pages,
    })
}

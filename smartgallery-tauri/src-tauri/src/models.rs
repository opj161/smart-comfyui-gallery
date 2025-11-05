// src-tauri/src/models.rs
//
// Core data structures for SmartGallery
// These types are serializable for IPC communication between Rust and frontend

use serde::{Deserialize, Serialize};

/// Represents a file entry in the gallery database
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileEntry {
    pub id: String,
    pub path: String,
    pub name: String,
    #[serde(rename = "type")]
    pub file_type: String, // 'type' is a keyword, so we use file_type with rename
    pub mtime: f64,
    pub has_workflow: bool,
    pub is_favorite: bool,
    pub prompt_preview: Option<String>,
    pub sampler_names: Option<String>,
    pub dimensions: Option<String>,
    pub duration: Option<String>,
    pub sampler_count: i32,
}

/// Represents workflow metadata extracted from ComfyUI files
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WorkflowMetadata {
    pub id: Option<i64>,
    pub file_id: String,
    pub sampler_index: i32,
    pub model_name: Option<String>,
    pub sampler_name: Option<String>,
    pub scheduler: Option<String>,
    pub cfg: Option<f64>,
    pub steps: Option<i64>,
    pub positive_prompt: Option<String>,
    pub negative_prompt: Option<String>,
    pub width: Option<i64>,
    pub height: Option<i64>,
}

/// Represents a folder in the file system hierarchy
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FolderEntry {
    pub key: String,
    pub name: String,
    pub path: String,
    pub parent_key: Option<String>,
    pub file_count: i32,
    pub subfolders: Vec<FolderEntry>,
}

/// Filter options for gallery queries
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FilterOptions {
    pub search: Option<String>,
    pub file_types: Vec<String>,
    pub extensions: Vec<String>,
    pub favorites_only: bool,
    pub has_workflow: Option<bool>,
    pub date_min: Option<String>,
    pub date_max: Option<String>,
    pub model_name: Option<String>,
    pub sampler_name: Option<String>,
    pub scheduler: Option<String>,
    pub cfg_min: Option<f64>,
    pub cfg_max: Option<f64>,
    pub steps_min: Option<i64>,
    pub steps_max: Option<i64>,
    pub width_min: Option<i64>,
    pub width_max: Option<i64>,
    pub height_min: Option<i64>,
    pub height_max: Option<i64>,
}

/// Pagination parameters for queries
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaginationParams {
    pub page: i32,
    pub per_page: i32,
    pub sort_by: String,
    pub sort_order: String,
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            page: 1,
            per_page: 50,
            sort_by: "mtime".to_string(),
            sort_order: "desc".to_string(),
        }
    }
}

/// Result of a paginated query
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaginatedResult<T> {
    pub items: Vec<T>,
    pub total: i64,
    pub page: i32,
    pub per_page: i32,
    pub total_pages: i32,
}

/// Sync progress update
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SyncProgress {
    pub current: i32,
    pub total: i32,
    pub status: String,
    pub message: Option<String>,
}

/// Application configuration
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub base_output_path: String,
    pub base_input_path: Option<String>,
    pub server_port: u16,
    pub enable_upload: bool,
    pub max_upload_size_mb: u64,
    pub thumbnail_quality: u8,
    pub ffprobe_manual_path: Option<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            base_output_path: String::new(),
            base_input_path: None,
            server_port: 8008,
            enable_upload: true,
            max_upload_size_mb: 100,
            thumbnail_quality: 85,
            ffprobe_manual_path: None,
        }
    }
}

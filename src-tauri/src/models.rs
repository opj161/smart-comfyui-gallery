// src-tauri/src/models.rs
// Core data structures for SmartGallery
// These structs define the data model shared between Rust backend and TypeScript frontend

use serde::{Deserialize, Serialize};

/// Represents a file entry in the gallery
/// Corresponds to the 'files' table in the SQLite database
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileEntry {
    pub id: String,
    pub path: String,
    pub name: String,
    #[serde(rename = "type")]
    pub file_type: String, // 'type' is a keyword in Rust, so we use file_type
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
/// Corresponds to the 'workflow_metadata' table in the SQLite database
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
    pub seed: Option<i64>,
}

/// Request structure for filtering files
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FilterRequest {
    pub folder_key: Option<String>,
    pub search: Option<String>,
    pub file_types: Option<Vec<String>>,
    pub extensions: Option<Vec<String>>,
    pub favorites_only: bool,
    pub has_workflow_only: bool,
    
    // Date filters
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    
    // Dimension filters
    pub width_min: Option<i64>,
    pub width_max: Option<i64>,
    pub height_min: Option<i64>,
    pub height_max: Option<i64>,
    
    // Workflow filters
    pub model_names: Option<Vec<String>>,
    pub sampler_names: Option<Vec<String>>,
    pub schedulers: Option<Vec<String>>,
    pub cfg_min: Option<f64>,
    pub cfg_max: Option<f64>,
    pub steps_min: Option<i64>,
    pub steps_max: Option<i64>,
    pub seed: Option<String>,
    pub prompt_search: Option<String>,
    
    // Pagination
    pub page: i32,
    pub per_page: i32,
    
    // Sorting
    pub sort_by: String, // "date" or "name"
    pub sort_order: String, // "asc" or "desc"
}

/// Response structure for file listing
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileListResponse {
    pub files: Vec<FileEntry>,
    pub total: i32,
    pub page: i32,
    pub per_page: i32,
    pub has_more: bool,
}

/// Folder structure for sidebar navigation
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FolderNode {
    pub key: String,
    pub name: String,
    pub path: String,
    pub file_count: i32,
    pub children: Vec<FolderNode>,
}

/// Sync progress update
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SyncProgress {
    pub current: usize,
    pub total: usize,
    pub status: String,
    pub message: Option<String>,
}

/// Configuration for the application
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub base_output_path: String,
    pub base_input_path: Option<String>,
    pub server_port: u16,
    pub enable_upload: bool,
    pub max_upload_size_mb: i32,
    pub thumbnail_quality: u8,
    pub ffprobe_manual_path: Option<String>,
}

/// Filter options for dropdowns (models, samplers, etc.)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FilterOptions {
    pub file_types: Vec<String>,
    pub extensions: Vec<String>,
    pub model_names: Vec<String>,
    pub sampler_names: Vec<String>,
    pub schedulers: Vec<String>,
}

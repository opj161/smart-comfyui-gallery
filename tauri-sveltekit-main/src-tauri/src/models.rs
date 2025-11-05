// Core data models for SmartGallery
// These structs map to the Python application's database schema

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileEntry {
    pub id: String,
    pub path: String,
    pub name: String,
    #[serde(rename = "type")]
    pub file_type: String, // 'type' is a keyword in Rust
    pub mtime: f64,
    pub has_workflow: bool,
    pub is_favorite: bool,
    pub prompt_preview: Option<String>,
    pub sampler_names: Option<String>,
    pub dimensions: Option<String>,
    pub duration: Option<String>,
    pub sampler_count: i32,
}

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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FolderConfig {
    pub display_name: String,
    pub path: String,
    pub relative_path: String,
    pub parent: Option<String>,
    pub children: Vec<String>,
    pub mtime: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SyncProgress {
    pub status: String,
    pub current: usize,
    pub total: usize,
    pub message: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FilterOptions {
    pub models: Vec<String>,
    pub samplers: Vec<String>,
    pub schedulers: Vec<String>,
    pub extensions: Vec<String>,
    pub prefixes: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GalleryFilters {
    pub search: Option<String>,
    pub folder_key: Option<String>,
    pub favorites_only: bool,
    pub file_types: Vec<String>,
    pub extensions: Vec<String>,
    pub prefixes: Vec<String>,
    pub model: Option<String>,
    pub sampler: Option<String>,
    pub scheduler: Option<String>,
    pub cfg_min: Option<f64>,
    pub cfg_max: Option<f64>,
    pub steps_min: Option<i64>,
    pub steps_max: Option<i64>,
    pub width_min: Option<i64>,
    pub width_max: Option<i64>,
    pub height_min: Option<i64>,
    pub height_max: Option<i64>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaginatedFiles {
    pub files: Vec<FileEntry>,
    pub total_count: usize,
    pub has_more: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub output_path: String,
    pub input_path: Option<String>,
    pub port: Option<u16>,
}

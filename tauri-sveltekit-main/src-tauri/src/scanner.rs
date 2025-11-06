// File system scanner for SmartGallery
// Scans directories for media files and syncs with database

use std::path::{Path, PathBuf};
use std::collections::{HashMap, HashSet};
use walkdir::WalkDir;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;
use serde::Serialize;

use crate::models::{FileEntry, SyncProgress};
use crate::database;
use crate::parser;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ScannerConfig {
    pub output_path: PathBuf,
    pub input_path: Option<PathBuf>,
    pub thumbnail_cache_dir: PathBuf,
    pub thumbnail_width: u32,
    pub image_extensions: Vec<String>,
    pub video_extensions: Vec<String>,
    pub audio_extensions: Vec<String>,
    pub animated_extensions: Vec<String>,
}

impl ScannerConfig {
    pub fn new(output_path: PathBuf) -> Self {
        let thumbnail_cache_dir = output_path.join("thumbnails_cache");
        
        Self {
            output_path: output_path.clone(),
            input_path: None,
            thumbnail_cache_dir,
            thumbnail_width: 200,
            image_extensions: vec![
                ".png".to_string(), ".jpg".to_string(), ".jpeg".to_string(),
                ".webp".to_string(), ".bmp".to_string(), ".gif".to_string(),
            ],
            video_extensions: vec![
                ".mp4".to_string(), ".avi".to_string(), ".mov".to_string(),
                ".mkv".to_string(), ".webm".to_string(), ".flv".to_string(),
            ],
            audio_extensions: vec![
                ".mp3".to_string(), ".wav".to_string(), ".ogg".to_string(),
                ".flac".to_string(), ".m4a".to_string(),
            ],
            animated_extensions: vec![
                ".gif".to_string(), ".webp".to_string(),
            ],
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ScanStats {
    pub total_processed: usize,
    pub files_with_workflows: usize,
    pub workflows_extracted: usize,
    pub metadata_extracted: usize,
    pub failed_files: usize,
}

impl ScanStats {
    pub fn new() -> Self {
        Self {
            total_processed: 0,
            files_with_workflows: 0,
            workflows_extracted: 0,
            metadata_extracted: 0,
            failed_files: 0,
        }
    }
}

/// Scan all files in output directory recursively
pub fn scan_directory(config: &ScannerConfig) -> Result<Vec<PathBuf>, String> {
    let mut files = Vec::new();
    
    let walker = WalkDir::new(&config.output_path)
        .follow_links(false)
        .into_iter()
        .filter_entry(|e| {
            // Skip thumbnail and database cache directories
            let path = e.path();
            let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            !file_name.starts_with("thumbnails_cache") 
                && !file_name.starts_with("smartgallery_cache")
                && !file_name.starts_with(".")
        });
    
    for entry in walker {
        match entry {
            Ok(entry) => {
                let path = entry.path();
                if path.is_file() {
                    // Check if it's a media file
                    if let Some(ext) = path.extension() {
                        let ext_str = format!(".{}", ext.to_string_lossy().to_lowercase());
                        
                        // Skip JSON and database files
                        if ext_str == ".json" || ext_str == ".sqlite" || ext_str == ".db" {
                            continue;
                        }
                        
                        // Check if it's a supported media file
                        if config.image_extensions.contains(&ext_str)
                            || config.video_extensions.contains(&ext_str)
                            || config.audio_extensions.contains(&ext_str)
                        {
                            files.push(path.to_path_buf());
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Error walking directory: {}", e);
            }
        }
    }
    
    Ok(files)
}

/// Get file modification time as f64 (seconds since epoch)
fn get_mtime(path: &Path) -> Result<f64, String> {
    let metadata = std::fs::metadata(path)
        .map_err(|e| format!("Failed to get metadata: {}", e))?;
    
    let mtime = metadata.modified()
        .map_err(|e| format!("Failed to get mtime: {}", e))?;
    
    let duration = mtime.duration_since(SystemTime::UNIX_EPOCH)
        .map_err(|e| format!("Failed to convert time: {}", e))?;
    
    Ok(duration.as_secs_f64())
}

/// Generate file ID from path (simple hash)
fn generate_file_id(path: &Path) -> String {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(path.to_string_lossy().as_bytes());
    let result = hasher.finalize();
    hex::encode(&result[..16]) // Use first 16 bytes
}

/// Process a single file: extract metadata, workflow, create thumbnail
/// Returns both the FileEntry and the parsed workflow data
pub fn process_file(
    filepath: &Path,
    config: &ScannerConfig,
) -> Result<(FileEntry, Vec<parser::ParsedWorkflow>), String> {
    // Generate file ID
    let file_id = generate_file_id(filepath);
    
    // Get file name
    let file_name = filepath
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or("Invalid filename")?
        .to_string();
    
    // Get modification time
    let mtime = get_mtime(filepath)?;
    
    // Determine file type
    let extension = filepath
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");
    let ext_lower = format!(".{}", extension.to_lowercase());
    
    let file_type = if config.image_extensions.contains(&ext_lower) {
        "image"
    } else if config.video_extensions.contains(&ext_lower) {
        "video"
    } else if config.audio_extensions.contains(&ext_lower) {
        "audio"
    } else {
        "unknown"
    };
    
    // Extract workflow if present
    let (has_workflow, workflow_metadata) = extract_workflow_from_file(filepath)?;
    
    // Get dimensions and duration based on file type
    let (dimensions, duration) = extract_media_metadata(filepath, file_type)?;
    
    // Calculate prompt preview and sampler names from workflow metadata
    let (prompt_preview, sampler_names, sampler_count) = if !workflow_metadata.is_empty() {
        let first_prompt = workflow_metadata[0].positive_prompt.clone();
        let preview = if first_prompt.len() > 150 {
            format!("{}...", &first_prompt[..150])
        } else {
            first_prompt
        };
        
        let unique_samplers: HashSet<String> = workflow_metadata
            .iter()
            .filter_map(|m| m.sampler_name.clone())
            .collect();
        let samplers_str = unique_samplers.into_iter().collect::<Vec<_>>().join(", ");
        
        (
            Some(preview),
            Some(samplers_str),
            workflow_metadata.len() as i32,
        )
    } else {
        (None, None, 0)
    };
    
    let file_entry = FileEntry {
        id: file_id,
        path: filepath.to_string_lossy().to_string(),
        name: file_name,
        file_type: file_type.to_string(),
        mtime,
        has_workflow,
        is_favorite: false,
        prompt_preview,
        sampler_names,
        dimensions,
        duration,
        sampler_count,
    };
    
    // Return both the file entry and the workflow metadata
    Ok((file_entry, workflow_metadata))
}

/// Extract workflow from PNG tEXt chunk or video metadata
fn extract_workflow_from_file(filepath: &Path) -> Result<(bool, Vec<parser::ParsedWorkflow>), String> {
    // Try to read file and extract workflow JSON
    let extension = filepath.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");
    
    if extension.to_lowercase() == "png" {
        // Extract from PNG tEXt chunk
        match extract_png_workflow(filepath) {
            Ok(Some(workflow_json)) => {
                match parser::extract_workflow_metadata(&workflow_json, filepath) {
                    Ok(metadata) if !metadata.is_empty() => Ok((true, metadata)),
                    Ok(_) => Ok((true, Vec::new())), // Has workflow but couldn't parse
                    Err(_) => Ok((true, Vec::new())), // Has workflow but parse failed
                }
            }
            Ok(None) => Ok((false, Vec::new())),
            Err(_) => Ok((false, Vec::new())),
        }
    } else {
        // For videos and other formats, we'd need to implement metadata extraction
        // For now, return no workflow
        Ok((false, Vec::new()))
    }
}

/// Extract workflow JSON from PNG tEXt chunk
fn extract_png_workflow(filepath: &Path) -> Result<Option<String>, String> {
    use std::fs::File;
    use std::io::{Read, BufReader};
    
    let file = File::open(filepath)
        .map_err(|e| format!("Failed to open file: {}", e))?;
    let mut reader = BufReader::new(file);
    
    // Read PNG signature
    let mut sig = [0u8; 8];
    reader.read_exact(&mut sig)
        .map_err(|e| format!("Failed to read PNG signature: {}", e))?;
    
    if sig != [137, 80, 78, 71, 13, 10, 26, 10] {
        return Ok(None); // Not a PNG
    }
    
    // Read chunks looking for tEXt with "workflow" or "prompt"
    loop {
        let mut length_buf = [0u8; 4];
        if reader.read_exact(&mut length_buf).is_err() {
            break;
        }
        let length = u32::from_be_bytes(length_buf);
        
        let mut type_buf = [0u8; 4];
        if reader.read_exact(&mut type_buf).is_err() {
            break;
        }
        
        // Check if this is a tEXt chunk
        if &type_buf == b"tEXt" {
            let mut data = vec![0u8; length as usize];
            if reader.read_exact(&mut data).is_err() {
                break;
            }
            
            // Find null separator
            if let Some(null_pos) = data.iter().position(|&b| b == 0) {
                let keyword = String::from_utf8_lossy(&data[..null_pos]);
                let text = String::from_utf8_lossy(&data[null_pos + 1..]);
                
                // Check for workflow keywords
                if keyword.to_lowercase().contains("workflow") 
                    || keyword.to_lowercase().contains("prompt") {
                    return Ok(Some(text.to_string()));
                }
            }
            
            // Skip CRC
            let mut crc = [0u8; 4];
            let _ = reader.read_exact(&mut crc);
        } else {
            // Skip chunk data and CRC
            let mut skip_buf = vec![0u8; (length + 4) as usize];
            if reader.read_exact(&mut skip_buf).is_err() {
                break;
            }
        }
    }
    
    Ok(None)
}

/// Extract media metadata (dimensions, duration)
fn extract_media_metadata(filepath: &Path, file_type: &str) -> Result<(Option<String>, Option<String>), String> {
    if file_type == "image" {
        // Try to get image dimensions using image crate
        if let Ok(img) = image::open(filepath) {
            let (width, height) = (img.width(), img.height());
            return Ok((Some(format!("{}x{}", width, height)), None));
        }
    }
    // For video/audio, we'd need ffprobe or similar - return None for now
    Ok((None, None))
}

/// Full database sync: compare disk files with database and process changes
pub async fn full_sync(
    pool: &sqlx::SqlitePool,
    config: &ScannerConfig,
    progress_callback: Option<Box<dyn Fn(SyncProgress) + Send + Sync>>,
) -> Result<ScanStats, String> {
    // Get all files from database
    let db_files_vec = database::get_all_file_paths(pool).await?;
    let db_files: HashMap<String, f64> = db_files_vec.into_iter().collect();
    
    // Scan disk for files
    let disk_files_paths = scan_directory(config)?;
    
    // Build disk files map with mtimes
    let mut disk_files: HashMap<String, f64> = HashMap::new();
    for path in &disk_files_paths {
        if let Ok(mtime) = get_mtime(path) {
            disk_files.insert(path.to_string_lossy().to_string(), mtime);
        }
    }
    
    // Determine what to add, update, delete
    let db_paths: HashSet<String> = db_files.keys().cloned().collect();
    let disk_paths: HashSet<String> = disk_files.keys().cloned().collect();
    
    let to_delete: Vec<String> = db_paths.difference(&disk_paths).cloned().collect();
    let to_add: Vec<String> = disk_paths.difference(&db_paths).cloned().collect();
    let to_check: Vec<String> = disk_paths.intersection(&db_paths).cloned().collect();
    
    // Find files that need updating (mtime changed)
    let to_update: Vec<String> = to_check
        .into_iter()
        .filter(|path| {
            let disk_mtime = disk_files.get(path).copied().unwrap_or(0.0) as i64;
            let db_mtime = db_files.get(path).copied().unwrap_or(0.0) as i64;
            disk_mtime > db_mtime
        })
        .collect();
    
    // Combine files to process
    let mut files_to_process: Vec<String> = to_add;
    files_to_process.extend(to_update);
    
    let total_files = files_to_process.len();
    
    // Delete removed files
    for _path in &to_delete {
        // We'd need to get the file_id from the path first
        // For now, just note that we'd delete them
    }
    
    // Process files in parallel using Rayon
    let stats = Arc::new(Mutex::new(ScanStats::new()));
    let processed = Arc::new(Mutex::new(0usize));
    let pool_arc = Arc::new(pool.clone());
    
    files_to_process
        .par_iter()
        .for_each(|path| {
            let path_buf = PathBuf::from(path);
            let pool_clone = pool_arc.clone();
            
            // Use block_on to execute async database operations in sync context
            let result = tauri::async_runtime::block_on(async {
                // Process file to get metadata AND workflow data in one pass
                let (mut file_entry, workflow_metadata) = match process_file(&path_buf, config) {
                    Ok(data) => data,
                    Err(e) => {
                        eprintln!("Failed to process file {}: {}", path, e);
                        return Err(e);
                    }
                };
                
                // Preserve favorite status if file already exists in database
                if let Ok(Some(existing_file)) = database::get_file_by_id(&pool_clone, &file_entry.id).await {
                    file_entry.is_favorite = existing_file.is_favorite;
                }

                // **CRITICAL FIX: Save file entry to database**
                if let Err(e) = database::upsert_file(&pool_clone, &file_entry).await {
                    eprintln!("Failed to upsert file record for {}: {}", path, e);
                    return Err(format!("Database error: {}", e));
                }

                // **Save workflow metadata if present**
                if !workflow_metadata.is_empty() {
                    for (i, parsed) in workflow_metadata.iter().enumerate() {
                        let meta = crate::models::WorkflowMetadata {
                            id: None,
                            file_id: file_entry.id.clone(),
                            sampler_index: i as i32,
                            model_name: parsed.model_name.clone(),
                            sampler_name: parsed.sampler_name.clone(),
                            scheduler: parsed.scheduler.clone(),
                            cfg: parsed.cfg,
                            steps: parsed.steps,
                            positive_prompt: Some(parsed.positive_prompt.clone()),
                            negative_prompt: Some(parsed.negative_prompt.clone()),
                            width: parsed.width,
                            height: parsed.height,
                        };
                        
                        if let Err(e) = database::insert_workflow_metadata(&pool_clone, &meta).await {
                            eprintln!("Failed to insert workflow metadata for {}: {}", path, e);
                        }
                    }
                }
                
                Ok((file_entry.has_workflow, workflow_metadata.len()))
            });
            
            // Update stats based on result
            match result {
                Ok((has_workflow, metadata_count)) => {
                    let mut stats_guard = stats.lock().unwrap();
                    stats_guard.total_processed += 1;
                    if has_workflow {
                        stats_guard.files_with_workflows += 1;
                        if metadata_count > 0 {
                            stats_guard.metadata_extracted += metadata_count;
                        }
                    }
                }
                Err(_) => {
                    let mut stats_guard = stats.lock().unwrap();
                    stats_guard.failed_files += 1;
                }
            }
            
            // Update progress
            let mut processed_guard = processed.lock().unwrap();
            *processed_guard += 1;
            
            if let Some(ref callback) = progress_callback {
                callback(SyncProgress {
                    status: "processing".to_string(),
                    current: *processed_guard,
                    total: total_files,
                    message: Some(format!("Processing {}/{}", *processed_guard, total_files)),
                });
            }
        });
    
    let final_stats = stats.lock().unwrap().clone();
    Ok(final_stats)
}

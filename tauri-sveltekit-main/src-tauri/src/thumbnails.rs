// Thumbnail generation for SmartGallery
// Creates thumbnails for images and videos

use std::path::{Path, PathBuf};
use image::{DynamicImage, imageops::FilterType, ImageFormat, GenericImageView};
use std::fs;

#[derive(Debug, Clone)]
pub struct ThumbnailConfig {
    pub cache_dir: PathBuf,
    pub width: u32,
    pub height: u32,
    pub quality: u8,
}

impl ThumbnailConfig {
    pub fn new(cache_dir: PathBuf) -> Self {
        Self {
            cache_dir,
            width: 200,
            height: 400, // 2x width for aspect ratio preservation
            quality: 85,
        }
    }
}

/// Generate file hash for thumbnail filename
fn generate_file_hash(path: &Path) -> String {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(path.to_string_lossy().as_bytes());
    let result = hasher.finalize();
    hex::encode(&result[..16])
}

/// Create thumbnail for an image file
pub fn create_image_thumbnail(
    filepath: &Path,
    config: &ThumbnailConfig,
) -> Result<PathBuf, String> {
    // Ensure cache directory exists
    fs::create_dir_all(&config.cache_dir)
        .map_err(|e| format!("Failed to create cache directory: {}", e))?;
    
    // Generate hash for cache filename
    let file_hash = generate_file_hash(filepath);
    
    // Load image
    let img = image::open(filepath)
        .map_err(|e| format!("Failed to open image: {}", e))?;
    
    // Determine output format based on input
    let format = match filepath.extension().and_then(|e| e.to_str()) {
        Some("gif") => ImageFormat::Gif,
        Some("webp") => ImageFormat::WebP,
        Some("png") => ImageFormat::Png,
        _ => ImageFormat::Jpeg,
    };
    
    let extension = match format {
        ImageFormat::Gif => "gif",
        ImageFormat::WebP => "webp",
        ImageFormat::Png => "png",
        _ => "jpeg",
    };
    
    let cache_path = config.cache_dir.join(format!("{}.{}", file_hash, extension));
    
    // Check if thumbnail already exists
    if cache_path.exists() {
        return Ok(cache_path);
    }
    
    // Create thumbnail
    let thumbnail = resize_image(&img, config.width, config.height);
    
    // Convert to RGB if needed
    let thumbnail_rgb = if thumbnail.color().has_alpha() && format == ImageFormat::Jpeg {
        DynamicImage::ImageRgb8(thumbnail.to_rgb8())
    } else {
        thumbnail
    };
    
    // Save thumbnail
    thumbnail_rgb.save_with_format(&cache_path, format)
        .map_err(|e| format!("Failed to save thumbnail: {}", e))?;
    
    Ok(cache_path)
}

/// Resize image maintaining aspect ratio
fn resize_image(img: &DynamicImage, max_width: u32, max_height: u32) -> DynamicImage {
    let (width, height) = img.dimensions();
    
    // Calculate new dimensions maintaining aspect ratio
    let ratio = (max_width as f32 / width as f32).min(max_height as f32 / height as f32);
    let new_width = (width as f32 * ratio) as u32;
    let new_height = (height as f32 * ratio) as u32;
    
    img.resize(new_width, new_height, FilterType::Lanczos3)
}

/// Create thumbnail for a video file using ffmpeg
pub fn create_video_thumbnail(
    filepath: &Path,
    config: &ThumbnailConfig,
) -> Result<PathBuf, String> {
    // Ensure cache directory exists
    fs::create_dir_all(&config.cache_dir)
        .map_err(|e| format!("Failed to create cache directory: {}", e))?;
    
    // Generate hash for cache filename
    let file_hash = generate_file_hash(filepath);
    let cache_path = config.cache_dir.join(format!("{}.jpeg", file_hash));
    
    // Check if thumbnail already exists
    if cache_path.exists() {
        return Ok(cache_path);
    }
    
    // Use ffmpeg to extract a frame
    let output = std::process::Command::new("ffmpeg")
        .arg("-i")
        .arg(filepath)
        .arg("-ss")
        .arg("00:00:01") // Seek to 1 second
        .arg("-vframes")
        .arg("1") // Extract 1 frame
        .arg("-vf")
        .arg(format!("scale={}:-1", config.width)) // Scale to thumbnail width
        .arg("-q:v")
        .arg("2") // Quality
        .arg(&cache_path)
        .arg("-y") // Overwrite
        .output()
        .map_err(|e| format!("Failed to execute ffmpeg: {}", e))?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("ffmpeg failed: {}", stderr));
    }
    
    Ok(cache_path)
}

/// Get thumbnail path for a file (create if doesn't exist)
pub fn get_or_create_thumbnail(
    filepath: &Path,
    file_type: &str,
    config: &ThumbnailConfig,
) -> Result<PathBuf, String> {
    match file_type {
        "image" | "animated_image" => create_image_thumbnail(filepath, config),
        "video" => create_video_thumbnail(filepath, config),
        _ => Err("Unsupported file type for thumbnail".to_string()),
    }
}

/// Check if thumbnail exists for a file
pub fn thumbnail_exists(filepath: &Path, config: &ThumbnailConfig) -> bool {
    let file_hash = generate_file_hash(filepath);
    
    // Check for common thumbnail formats
    for ext in &["jpeg", "jpg", "png", "gif", "webp"] {
        let cache_path = config.cache_dir.join(format!("{}.{}", file_hash, ext));
        if cache_path.exists() {
            return true;
        }
    }
    
    false
}

/// Get existing thumbnail path without creating
pub fn get_thumbnail_path(filepath: &Path, config: &ThumbnailConfig) -> Option<PathBuf> {
    let file_hash = generate_file_hash(filepath);
    
    // Check for common thumbnail formats
    for ext in &["jpeg", "jpg", "png", "gif", "webp"] {
        let cache_path = config.cache_dir.join(format!("{}.{}", file_hash, ext));
        if cache_path.exists() {
            return Some(cache_path);
        }
    }
    
    None
}

/// Clean up old/unused thumbnails
pub fn cleanup_thumbnails(
    valid_file_paths: &[PathBuf],
    config: &ThumbnailConfig,
) -> Result<usize, String> {
    let mut removed_count = 0;
    
    // Build set of valid hashes
    let valid_hashes: std::collections::HashSet<String> = valid_file_paths
        .iter()
        .map(|p| generate_file_hash(p))
        .collect();
    
    // Read cache directory
    let entries = fs::read_dir(&config.cache_dir)
        .map_err(|e| format!("Failed to read cache directory: {}", e))?;
    
    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_file() {
                // Extract hash from filename
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    if !valid_hashes.contains(stem) {
                        // Remove orphaned thumbnail
                        if fs::remove_file(&path).is_ok() {
                            removed_count += 1;
                        }
                    }
                }
            }
        }
    }
    
    Ok(removed_count)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_generate_file_hash() {
        let path = Path::new("/test/file.png");
        let hash = generate_file_hash(path);
        assert_eq!(hash.len(), 32); // 16 bytes hex encoded
    }
    
    #[test]
    fn test_resize_image() {
        // Create a simple test image
        let img = DynamicImage::new_rgb8(1000, 800);
        let resized = resize_image(&img, 200, 400);
        
        let (width, height) = resized.dimensions();
        assert!(width <= 200);
        assert!(height <= 400);
    }
}

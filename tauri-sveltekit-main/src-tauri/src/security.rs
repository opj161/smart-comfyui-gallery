// Security module for path validation and access control
use std::path::{Path, PathBuf};

/// Validate that a path is within allowed directories
pub fn validate_path(
    path: &Path,
    allowed_dirs: &[PathBuf]
) -> Result<PathBuf, String> {
    // Canonicalize path (resolve symlinks, .., etc.)
    let canonical = path.canonicalize()
        .map_err(|e| format!("Invalid path: {}", e))?;
    
    // Check if path is within any allowed directory
    for allowed_dir in allowed_dirs {
        let canonical_allowed = allowed_dir.canonicalize()
            .map_err(|e| format!("Invalid allowed directory: {}", e))?;
        
        if canonical.starts_with(&canonical_allowed) {
            return Ok(canonical);
        }
    }
    
    Err(format!(
        "Path {:?} is not within allowed directories",
        path
    ))
}

/// Get list of allowed directories from app state
pub fn get_allowed_directories(
    output_path: &Option<PathBuf>,
    input_path: &Option<PathBuf>
) -> Vec<PathBuf> {
    let mut dirs = Vec::new();
    
    if let Some(output) = output_path {
        dirs.push(output.clone());
    }
    
    if let Some(input) = input_path {
        dirs.push(input.clone());
    }
    
    dirs
}

/// Validate and sanitize a file path for safe operations
pub fn sanitize_filename(filename: &str) -> Result<String, String> {
    // Remove path traversal attempts
    if filename.contains("..") || filename.contains("/") || filename.contains("\\") {
        return Err("Invalid filename: contains path traversal characters".to_string());
    }
    
    // Check for valid UTF-8
    if !filename.is_ascii() && !filename.chars().all(|c| c.is_alphanumeric() || c == '.' || c == '-' || c == '_') {
        return Err("Invalid filename: contains invalid characters".to_string());
    }
    
    Ok(filename.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_validate_path_within_allowed() {
        let temp = tempdir().unwrap();
        let allowed = temp.path().to_path_buf();
        let test_file = allowed.join("test.txt");
        fs::write(&test_file, "test").unwrap();
        
        let result = validate_path(&test_file, &[allowed]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_path_outside_allowed() {
        let temp1 = tempdir().unwrap();
        let temp2 = tempdir().unwrap();
        let allowed = temp1.path().to_path_buf();
        let test_file = temp2.path().join("test.txt");
        fs::write(&test_file, "test").unwrap();
        
        let result = validate_path(&test_file, &[allowed]);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_path_traversal() {
        let temp = tempdir().unwrap();
        let allowed = temp.path().join("safe");
        fs::create_dir_all(&allowed).unwrap();
        
        // Create a file outside the allowed directory
        let outside_file = temp.path().join("outside.txt");
        fs::write(&outside_file, "test").unwrap();
        
        // Try to access it via path traversal
        let evil_path = allowed.join("..").join("outside.txt");
        
        let result = validate_path(&evil_path, &[allowed]);
        // Should fail because canonical path is outside allowed
        assert!(result.is_err());
    }

    #[test]
    fn test_sanitize_filename_valid() {
        assert!(sanitize_filename("test.png").is_ok());
        assert!(sanitize_filename("my-file_123.jpg").is_ok());
    }

    #[test]
    fn test_sanitize_filename_invalid() {
        assert!(sanitize_filename("../etc/passwd").is_err());
        assert!(sanitize_filename("test/file.png").is_err());
        assert!(sanitize_filename("..\\windows\\system32").is_err());
    }
}

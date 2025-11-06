// Custom error types for the application
// Provides better error handling and user-friendly messages

use std::fmt;

#[derive(Debug)]
pub enum AppError {
    DatabaseError(String),
    FileSystemError(String),
    ValidationError(String),
    NetworkError(String),
    NotFound(String),
    PermissionDenied(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            AppError::FileSystemError(msg) => write!(f, "File system error: {}", msg),
            AppError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            AppError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

// Conversion from sqlx::Error
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::DatabaseError(err.to_string())
    }
}

// Conversion from std::io::Error
impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::FileSystemError(err.to_string())
    }
}

// Convert AppError to String for Tauri commands
impl From<AppError> for String {
    fn from(err: AppError) -> Self {
        err.to_string()
    }
}

// Helper function to create user-friendly error messages
#[allow(dead_code)]
pub fn to_user_message(error: &AppError) -> String {
    match error {
        AppError::DatabaseError(_) => "A database error occurred. Please try again.".to_string(),
        AppError::FileSystemError(_) => "A file system error occurred. Please check file permissions.".to_string(),
        AppError::ValidationError(msg) => format!("Invalid input: {}", msg),
        AppError::NetworkError(_) => "A network error occurred. Please check your connection.".to_string(),
        AppError::NotFound(msg) => format!("Not found: {}", msg),
        AppError::PermissionDenied(msg) => format!("Permission denied: {}", msg),
    }
}

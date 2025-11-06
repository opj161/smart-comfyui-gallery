// Configuration management for SmartGallery
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::fs;
use tauri::Manager;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub output_path: String,
    pub input_path: Option<String>,
    pub thumbnail_size: u32,
    pub theme: String,
    pub max_cache_size_mb: u32,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            output_path: "".to_string(),
            input_path: None,
            thumbnail_size: 256,
            theme: "dark".to_string(),
            max_cache_size_mb: 1000,
        }
    }
}

impl AppConfig {
    /// Load config from file, or create default if doesn't exist
    pub fn load(config_path: &PathBuf) -> Result<Self, String> {
        if config_path.exists() {
            let contents = fs::read_to_string(config_path)
                .map_err(|e| format!("Failed to read config: {}", e))?;
            let config: AppConfig = serde_json::from_str(&contents)
                .map_err(|e| format!("Failed to parse config: {}", e))?;
            Ok(config)
        } else {
            // Create default config
            let config = Self::default();
            config.save(config_path)?;
            Ok(config)
        }
    }

    /// Save config to file
    pub fn save(&self, config_path: &PathBuf) -> Result<(), String> {
        // Ensure parent directory exists
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create config directory: {}", e))?;
        }
        
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize config: {}", e))?;
        fs::write(config_path, json)
            .map_err(|e| format!("Failed to write config: {}", e))?;
        Ok(())
    }

    /// Validate that configured paths exist
    pub fn validate(&self) -> Result<(), String> {
        if self.output_path.is_empty() {
            return Err("Output path not configured".to_string());
        }
        
        let output = PathBuf::from(&self.output_path);
        if !output.exists() {
            return Err(format!("Output path does not exist: {}", self.output_path));
        }
        
        if let Some(input) = &self.input_path {
            let input_path = PathBuf::from(input);
            if !input_path.exists() {
                return Err(format!("Input path does not exist: {}", input));
            }
        }
        
        Ok(())
    }
}

#[tauri::command]
pub async fn load_config(app_handle: tauri::AppHandle) -> Result<AppConfig, String> {
    let config_dir = app_handle.path()
        .app_config_dir()
        .map_err(|e| format!("Failed to get config directory: {}", e))?;
    
    fs::create_dir_all(&config_dir)
        .map_err(|e| format!("Failed to create config directory: {}", e))?;
    
    let config_path = config_dir.join("config.json");
    AppConfig::load(&config_path)
}

#[tauri::command]
pub async fn save_config(
    config: AppConfig,
    app_handle: tauri::AppHandle
) -> Result<(), String> {
    // Validate config before saving
    config.validate()?;
    
    let config_dir = app_handle.path()
        .app_config_dir()
        .map_err(|e| format!("Failed to get config directory: {}", e))?;
    
    let config_path = config_dir.join("config.json");
    config.save(&config_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_config_default() {
        let config = AppConfig::default();
        assert_eq!(config.thumbnail_size, 256);
        assert_eq!(config.theme, "dark");
        assert_eq!(config.max_cache_size_mb, 1000);
    }

    #[test]
    fn test_config_save_load() {
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("config.json");
        
        let mut config = AppConfig::default();
        config.output_path = "/test/path".to_string();
        config.thumbnail_size = 512;
        
        config.save(&config_path).unwrap();
        
        let loaded = AppConfig::load(&config_path).unwrap();
        assert_eq!(loaded.output_path, "/test/path");
        assert_eq!(loaded.thumbnail_size, 512);
    }
}

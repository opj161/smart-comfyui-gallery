# Quick Implementation Guide

This document provides code templates and examples for implementing the critical missing features.

---

## 1. Configuration System

### Backend: Config Storage (`src-tauri/src/config.rs`)

```rust
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::fs;

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
            output_path: "C:\\.ai\\ComfyUI\\output".to_string(),
            input_path: Some("C:\\.ai\\ComfyUI\\input".to_string()),
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
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize config: {}", e))?;
        fs::write(config_path, json)
            .map_err(|e| format!("Failed to write config: {}", e))?;
        Ok(())
    }
}

#[tauri::command]
pub async fn load_config(app_handle: tauri::AppHandle) -> Result<AppConfig, String> {
    let config_dir = app_handle.path_resolver()
        .app_config_dir()
        .ok_or("Failed to get config directory")?;
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
    let config_dir = app_handle.path_resolver()
        .app_config_dir()
        .ok_or("Failed to get config directory")?;
    let config_path = config_dir.join("config.json");
    config.save(&config_path)
}
```

### Frontend: Settings Panel (`src/lib/components/SettingsPanel.svelte`)

```svelte
<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { open } from '@tauri-apps/plugin-dialog';
	import type { AppConfig } from '$lib/types';

	let config: AppConfig | null = $state(null);
	let isLoading = $state(true);
	let error = $state<string | null>(null);
	let isSaving = $state(false);

	// Load config on mount
	async function loadConfig() {
		try {
			config = await invoke('load_config');
			isLoading = false;
		} catch (e) {
			error = `Failed to load config: ${e}`;
			isLoading = false;
		}
	}

	// Save config
	async function saveConfig() {
		if (!config) return;
		
		isSaving = true;
		try {
			await invoke('save_config', { config });
			// Trigger app reinitialization
			await invoke('initialize_gallery', {
				outputPath: config.output_path,
				inputPath: config.input_path
			});
			alert('Settings saved! Gallery will reload.');
		} catch (e) {
			error = `Failed to save config: ${e}`;
		} finally {
			isSaving = false;
		}
	}

	// Select output directory
	async function selectOutputPath() {
		const selected = await open({
			directory: true,
			multiple: false,
			title: 'Select Output Directory'
		});
		
		if (selected && typeof selected === 'string' && config) {
			config.output_path = selected;
		}
	}

	// Select input directory
	async function selectInputPath() {
		const selected = await open({
			directory: true,
			multiple: false,
			title: 'Select Input Directory (Optional)'
		});
		
		if (selected && typeof selected === 'string' && config) {
			config.input_path = selected;
		}
	}

	// Initialize
	loadConfig();
</script>

<div class="settings-panel">
	<h2>Settings</h2>

	{#if isLoading}
		<p>Loading settings...</p>
	{:else if error}
		<p class="error">{error}</p>
	{:else if config}
		<form onsubmit={(e) => { e.preventDefault(); saveConfig(); }}>
			<!-- Output Path -->
			<div class="form-group">
				<label for="output-path">Output Directory</label>
				<div class="path-input">
					<input
						id="output-path"
						type="text"
						bind:value={config.output_path}
						required
					/>
					<button type="button" onclick={selectOutputPath}>Browse</button>
				</div>
				<p class="hint">ComfyUI output folder (required)</p>
			</div>

			<!-- Input Path -->
			<div class="form-group">
				<label for="input-path">Input Directory</label>
				<div class="path-input">
					<input
						id="input-path"
						type="text"
						bind:value={config.input_path}
						placeholder="Optional"
					/>
					<button type="button" onclick={selectInputPath}>Browse</button>
				</div>
				<p class="hint">ComfyUI input folder (optional)</p>
			</div>

			<!-- Thumbnail Size -->
			<div class="form-group">
				<label for="thumbnail-size">Thumbnail Size</label>
				<input
					id="thumbnail-size"
					type="number"
					bind:value={config.thumbnail_size}
					min="128"
					max="512"
					step="64"
				/>
				<p class="hint">Width in pixels (128-512)</p>
			</div>

			<!-- Theme -->
			<div class="form-group">
				<label for="theme">Theme</label>
				<select id="theme" bind:value={config.theme}>
					<option value="dark">Dark</option>
					<option value="light">Light</option>
				</select>
			</div>

			<!-- Cache Size -->
			<div class="form-group">
				<label for="cache-size">Max Cache Size (MB)</label>
				<input
					id="cache-size"
					type="number"
					bind:value={config.max_cache_size_mb}
					min="100"
					max="10000"
					step="100"
				/>
				<p class="hint">Maximum memory for thumbnails</p>
			</div>

			<!-- Actions -->
			<div class="actions">
				<button type="submit" disabled={isSaving}>
					{isSaving ? 'Saving...' : 'Save Settings'}
				</button>
			</div>
		</form>
	{/if}
</div>

<style>
	.settings-panel {
		padding: 2rem;
		max-width: 600px;
		margin: 0 auto;
	}

	.form-group {
		margin-bottom: 1.5rem;
	}

	label {
		display: block;
		margin-bottom: 0.5rem;
		font-weight: 600;
	}

	input, select {
		width: 100%;
		padding: 0.5rem;
		border: 1px solid #444;
		border-radius: 4px;
		background: #2a2a2a;
		color: #e0e0e0;
	}

	.path-input {
		display: flex;
		gap: 0.5rem;
	}

	.path-input input {
		flex: 1;
	}

	.path-input button {
		padding: 0.5rem 1rem;
		background: #0066cc;
		color: white;
		border: none;
		border-radius: 4px;
		cursor: pointer;
	}

	.hint {
		margin-top: 0.25rem;
		font-size: 0.875rem;
		color: #999;
	}

	.error {
		color: #ff5555;
		padding: 1rem;
		background: rgba(255, 85, 85, 0.1);
		border-radius: 4px;
	}

	.actions {
		margin-top: 2rem;
	}

	button[type="submit"] {
		width: 100%;
		padding: 0.75rem;
		background: #00aa00;
		color: white;
		border: none;
		border-radius: 4px;
		font-size: 1rem;
		cursor: pointer;
	}

	button[type="submit"]:disabled {
		background: #666;
		cursor: not-allowed;
	}
</style>
```

---

## 2. Path Validation

### Backend: Security Module (`src-tauri/src/security.rs`)

```rust
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
        
        let evil_path = allowed.join("..").join("..").join("etc").join("passwd");
        
        let result = validate_path(&evil_path, &[allowed]);
        assert!(result.is_err());
    }
}
```

### Usage in Commands:

```rust
use crate::security::{validate_path, get_allowed_directories};

#[tauri::command]
pub async fn delete_file(
    file_id: String,
    state: State<'_, Arc<Mutex<AppState>>>,
) -> Result<(), String> {
    let (pool, allowed_dirs) = {
        let app_state = state.lock().unwrap();
        let pool = app_state.db_pool.as_ref()
            .ok_or("Database not initialized")?
            .clone();
        let allowed = get_allowed_directories(
            &app_state.output_path,
            &app_state.input_path
        );
        (pool, allowed)
    };

    // Get file path from database
    let row = sqlx::query("SELECT path FROM files WHERE id = ?")
        .bind(&file_id)
        .fetch_one(&pool)
        .await
        .map_err(|e| format!("File not found: {}", e))?;
    
    let path_str: String = row.get("path");
    let file_path = PathBuf::from(&path_str);
    
    // SECURITY: Validate path before deletion
    let validated_path = validate_path(&file_path, &allowed_dirs)?;
    
    // Delete file
    std::fs::remove_file(&validated_path)
        .map_err(|e| format!("Failed to delete file: {}", e))?;
    
    // Remove from database
    sqlx::query("DELETE FROM files WHERE id = ?")
        .bind(&file_id)
        .execute(&pool)
        .await
        .map_err(|e| format!("Failed to remove from database: {}", e))?;
    
    Ok(())
}
```

---

## 3. File Upload

### Backend: Upload Command (`src-tauri/src/commands.rs`)

```rust
use std::fs;
use uuid::Uuid;

#[tauri::command]
pub async fn upload_file(
    source_path: String,
    state: State<'_, Arc<Mutex<AppState>>>,
) -> Result<String, String> {
    let (pool, output_path, scanner_config, thumbnail_config, allowed_dirs) = {
        let app_state = state.lock().unwrap();
        let pool = app_state.db_pool.as_ref()
            .ok_or("Database not initialized")?
            .clone();
        let output = app_state.output_path.as_ref()
            .ok_or("Output path not set")?
            .clone();
        let scanner = app_state.scanner_config.as_ref()
            .ok_or("Scanner config not set")?
            .clone();
        let thumbnail = app_state.thumbnail_config.as_ref()
            .ok_or("Thumbnail config not set")?
            .clone();
        let allowed = get_allowed_directories(
            &app_state.output_path,
            &app_state.input_path
        );
        (pool, output, scanner, thumbnail, allowed)
    };

    // Validate source file exists
    let source = PathBuf::from(&source_path);
    if !source.exists() {
        return Err("Source file does not exist".to_string());
    }

    // Generate destination path
    let filename = source.file_name()
        .ok_or("Invalid filename")?
        .to_string_lossy()
        .to_string();
    let dest_path = output_path.join(&filename);

    // Check if file already exists
    if dest_path.exists() {
        // Generate unique filename
        let stem = source.file_stem()
            .ok_or("Invalid filename")?
            .to_string_lossy();
        let ext = source.extension()
            .map(|e| e.to_string_lossy())
            .unwrap_or_default();
        let unique_name = format!("{}_{}.{}", stem, Uuid::new_v4(), ext);
        let dest_path = output_path.join(&unique_name);
    }

    // Copy file to output directory
    fs::copy(&source, &dest_path)
        .map_err(|e| format!("Failed to copy file: {}", e))?;

    // Process the uploaded file
    let file_entry = crate::scanner::process_single_file(
        &dest_path,
        &scanner_config,
        &thumbnail_config
    ).await?;

    // Insert into database
    crate::database::insert_file(&pool, &file_entry).await?;

    Ok(file_entry.id)
}

#[tauri::command]
pub async fn upload_multiple_files(
    source_paths: Vec<String>,
    state: State<'_, Arc<Mutex<AppState>>>,
    app_handle: tauri::AppHandle,
) -> Result<Vec<String>, String> {
    let mut file_ids = Vec::new();
    let total = source_paths.len();

    for (index, source_path) in source_paths.iter().enumerate() {
        // Emit progress event
        app_handle.emit_all("upload-progress", serde_json::json!({
            "current": index + 1,
            "total": total,
            "filename": Path::new(source_path).file_name()
        })).ok();

        // Upload file
        match upload_file(source_path.clone(), state.clone()).await {
            Ok(file_id) => file_ids.push(file_id),
            Err(e) => {
                eprintln!("Failed to upload {}: {}", source_path, e);
                // Continue with other files
            }
        }
    }

    // Emit completion event
    app_handle.emit_all("upload-complete", serde_json::json!({
        "total": total,
        "success": file_ids.len(),
        "failed": total - file_ids.len()
    })).ok();

    Ok(file_ids)
}
```

### Frontend: Upload Zone (`src/lib/components/UploadZone.svelte`)

```svelte
<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { listen } from '@tauri-apps/api/event';
	import { open } from '@tauri-apps/plugin-dialog';

	let isDragging = $state(false);
	let isUploading = $state(false);
	let uploadProgress = $state(0);
	let uploadTotal = $state(0);
	let currentFile = $state('');

	async function handleDrop(event: DragEvent) {
		event.preventDefault();
		isDragging = false;

		const files = event.dataTransfer?.files;
		if (!files || files.length === 0) return;

		const filePaths: string[] = [];
		for (let i = 0; i < files.length; i++) {
			const file = files[i];
			if (file.path) {
				filePaths.push(file.path);
			}
		}

		await uploadFiles(filePaths);
	}

	async function handleFileSelect() {
		const selected = await open({
			multiple: true,
			filters: [{
				name: 'Images and Videos',
				extensions: ['png', 'jpg', 'jpeg', 'webp', 'mp4', 'avi', 'mov', 'mkv', 'webm', 'gif']
			}]
		});

		if (selected) {
			const paths = Array.isArray(selected) ? selected : [selected];
			await uploadFiles(paths);
		}
	}

	async function uploadFiles(paths: string[]) {
		isUploading = true;
		uploadProgress = 0;
		uploadTotal = paths.length;

		// Listen for progress
		const unlisten = await listen('upload-progress', (event: any) => {
			uploadProgress = event.payload.current;
			currentFile = event.payload.filename;
		});

		try {
			await invoke('upload_multiple_files', { sourcePaths: paths });
			alert(`Successfully uploaded ${uploadProgress} files!`);
		} catch (error) {
			alert(`Upload failed: ${error}`);
		} finally {
			isUploading = false;
			unlisten();
		}
	}
</script>

<div
	class="upload-zone"
	class:dragging={isDragging}
	ondragover={(e) => { e.preventDefault(); isDragging = true; }}
	ondragleave={() => { isDragging = false; }}
	ondrop={handleDrop}
>
	{#if isUploading}
		<div class="upload-progress">
			<p>Uploading {currentFile}</p>
			<progress value={uploadProgress} max={uploadTotal}></progress>
			<p>{uploadProgress} / {uploadTotal} files</p>
		</div>
	{:else}
		<div class="upload-prompt">
			<p>Drag and drop files here</p>
			<p>or</p>
			<button onclick={handleFileSelect}>Select Files</button>
		</div>
	{/if}
</div>

<style>
	.upload-zone {
		border: 2px dashed #666;
		border-radius: 8px;
		padding: 2rem;
		text-align: center;
		transition: all 0.3s ease;
	}

	.upload-zone.dragging {
		border-color: #0066cc;
		background: rgba(0, 102, 204, 0.1);
	}

	progress {
		width: 100%;
		height: 20px;
		margin: 1rem 0;
	}

	button {
		padding: 0.75rem 1.5rem;
		background: #0066cc;
		color: white;
		border: none;
		border-radius: 4px;
		cursor: pointer;
		font-size: 1rem;
	}
</style>
```

---

## 4. Memory Management

### Backend: Bounded Cache (`src-tauri/src/cache.rs`)

```rust
use std::collections::HashMap;
use std::time::{Duration, Instant};

pub struct CachedItem<V> {
    pub value: V,
    pub inserted_at: Instant,
    pub last_accessed: Instant,
    pub access_count: u32,
}

pub struct BoundedCache<K, V>
where
    K: Eq + std::hash::Hash + Clone,
{
    cache: HashMap<K, CachedItem<V>>,
    max_size: usize,
    ttl: Duration,
}

impl<K, V> BoundedCache<K, V>
where
    K: Eq + std::hash::Hash + Clone,
{
    pub fn new(max_size: usize, ttl_seconds: u64) -> Self {
        Self {
            cache: HashMap::with_capacity(max_size),
            max_size,
            ttl: Duration::from_secs(ttl_seconds),
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        // Evict expired items
        self.evict_expired();

        if let Some(item) = self.cache.get_mut(key) {
            item.last_accessed = Instant::now();
            item.access_count += 1;
            Some(&item.value)
        } else {
            None
        }
    }

    pub fn set(&mut self, key: K, value: V) {
        // Evict if at capacity
        if self.cache.len() >= self.max_size {
            self.evict_lru();
        }

        let now = Instant::now();
        self.cache.insert(
            key,
            CachedItem {
                value,
                inserted_at: now,
                last_accessed: now,
                access_count: 0,
            },
        );
    }

    fn evict_expired(&mut self) {
        let now = Instant::now();
        self.cache.retain(|_, item| {
            now.duration_since(item.inserted_at) < self.ttl
        });
    }

    fn evict_lru(&mut self) {
        // Find least recently used item
        if let Some((key, _)) = self.cache.iter()
            .min_by_key(|(_, item)| item.last_accessed)
            .map(|(k, v)| (k.clone(), v))
        {
            self.cache.remove(&key);
        }
    }

    pub fn clear(&mut self) {
        self.cache.clear();
    }

    pub fn len(&self) -> usize {
        self.cache.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_basic() {
        let mut cache = BoundedCache::new(2, 300);
        cache.set("key1".to_string(), "value1".to_string());
        assert_eq!(cache.get(&"key1".to_string()), Some(&"value1".to_string()));
    }

    #[test]
    fn test_cache_eviction() {
        let mut cache = BoundedCache::new(2, 300);
        cache.set("key1".to_string(), "value1".to_string());
        cache.set("key2".to_string(), "value2".to_string());
        cache.set("key3".to_string(), "value3".to_string());
        
        // key1 should be evicted
        assert_eq!(cache.len(), 2);
        assert_eq!(cache.get(&"key1".to_string()), None);
    }

    #[test]
    fn test_cache_lru() {
        let mut cache = BoundedCache::new(2, 300);
        cache.set("key1".to_string(), "value1".to_string());
        cache.set("key2".to_string(), "value2".to_string());
        
        // Access key1 to make it recently used
        cache.get(&"key1".to_string());
        
        // Add key3, key2 should be evicted (LRU)
        cache.set("key3".to_string(), "value3".to_string());
        
        assert_eq!(cache.get(&"key1".to_string()), Some(&"value1".to_string()));
        assert_eq!(cache.get(&"key2".to_string()), None);
        assert_eq!(cache.get(&"key3".to_string()), Some(&"value3".to_string()));
    }
}
```

---

## 5. Integration Steps

### Update `main.rs`:

```rust
mod config;
mod security;
mod cache;

// Add to Tauri builder
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // Existing commands
            commands::initialize_gallery,
            // ... other commands ...
            
            // New commands
            config::load_config,
            config::save_config,
            commands::upload_file,
            commands::upload_multiple_files,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### Update `Cargo.toml`:

```toml
[dependencies]
uuid = { version = "1.0", features = ["v4", "serde"] }
tempfile = "3.8"  # for tests
```

### Update Frontend Types (`src/lib/types.ts`):

```typescript
export interface AppConfig {
	output_path: string;
	input_path: string | null;
	thumbnail_size: number;
	theme: string;
	max_cache_size_mb: number;
}

export interface UploadProgress {
	current: number;
	total: number;
	filename: string;
}

export interface UploadComplete {
	total: number;
	success: number;
	failed: number;
}
```

---

## Testing Checklist

### Configuration System
- [ ] Config loads on first run
- [ ] Config persists between sessions
- [ ] Path selection dialog works
- [ ] Invalid paths are rejected
- [ ] Settings apply immediately
- [ ] Config validation works

### Path Validation
- [ ] Can access files in configured directories
- [ ] Cannot access files outside directories
- [ ] Path traversal attempts blocked
- [ ] Symlinks handled correctly
- [ ] Unit tests pass

### File Upload
- [ ] Single file upload works
- [ ] Multiple file upload works
- [ ] Drag and drop works
- [ ] Progress indicator accurate
- [ ] Workflow extraction works
- [ ] Thumbnails generated
- [ ] Files added to database
- [ ] Duplicate filenames handled

### Memory Management
- [ ] Cache respects size limits
- [ ] LRU eviction works
- [ ] TTL expiration works
- [ ] Memory usage stays bounded
- [ ] Unit tests pass

---

## Next Steps

1. Copy code templates to your project
2. Install required dependencies
3. Run tests: `cargo test --all`
4. Build: `npm run tauri build`
5. Manual testing with real data
6. Iterate based on results

For detailed timelines and priorities, see **NEXT_STEPS.md**.

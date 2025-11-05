# SmartGallery Tauri Migration - Implementation Guide

This document provides a comprehensive guide for completing the migration from Python/Flask/PyWebView to Tauri/Rust/SvelteKit.

## Project Status

### ✅ Completed (Phases 1 & Partial Phase 2)

1. **Foundation** - Full Tauri + SvelteKit project scaffolding
2. **Data Models** - Complete Rust structs and TypeScript interfaces
3. **Database Layer** - Full SQLx implementation with migrations
4. **IPC Bridge** - Basic Tauri command system established

### 🔄 In Progress (Phase 2 - Backend Migration)

The following modules need to be implemented to complete the backend:

## Phase 2 Remaining Work

### 1. File Scanner Module (`src-tauri/src/scanner.rs`)

**Purpose**: Recursively scan directories, detect file changes, and trigger database updates.

**Key Functions to Implement**:

```rust
// Main scanning function
pub async fn full_scan(
    base_path: &Path,
    db_pool: &SqlitePool,
    progress_callback: impl Fn(SyncProgress)
) -> Result<()>

// Get all files in directory tree
pub fn get_disk_files(
    base_path: &Path,
    extensions: &[String]
) -> Result<HashMap<PathBuf, f64>>  // path -> mtime

// Compare disk vs database and determine actions
pub fn compute_changes(
    disk_files: HashMap<PathBuf, f64>,
    db_files: Vec<(String, f64)>
) -> (Vec<PathBuf>, Vec<PathBuf>, Vec<String>)  // (to_add, to_update, to_delete)

// Process files in parallel using rayon
pub async fn process_files_parallel(
    files: Vec<PathBuf>,
    db_pool: &SqlitePool,
    config: &AppConfig
) -> Result<Vec<FileEntry>>
```

**Dependencies**:
- `walkdir` for directory traversal
- `rayon` for parallel processing
- Integration with workflow parser and thumbnail generator

**Tauri Command**:
```rust
#[tauri::command]
async fn sync_folder(
    folder_path: String,
    app: tauri::AppHandle,
    state: State<'_, AppState>
) -> Result<String, String>
```

Use `app.emit()` to send progress events to frontend.

### 2. Workflow Parser Module (`src-tauri/src/parser.rs`)

**Purpose**: Extract ComfyUI workflow metadata from PNG/video files.

**Reference Implementation**: `smartgallery.py` lines 250-773

**Key Structures**:

```rust
pub struct WorkflowParser {
    nodes_by_id: HashMap<String, serde_json::Value>,
    links_map: Option<HashMap<i64, (String, i64)>>,
    format: WorkflowFormat,
}

pub enum WorkflowFormat {
    UI,   // Has 'nodes' array
    API,  // Node ID -> node data dict
}
```

**Key Methods**:

```rust
impl WorkflowParser {
    pub fn new(workflow_data: serde_json::Value, file_path: &Path) -> Result<Self>
    
    pub fn extract_metadata(&self) -> Result<Vec<WorkflowMetadata>>
    
    fn find_sampler_nodes(&self) -> Vec<&serde_json::Value>
    
    fn trace_input_node(&self, node: &serde_json::Value, input_name: &str) 
        -> Option<&serde_json::Value>
    
    fn extract_from_sampler(&self, sampler_node: &serde_json::Value, index: i32) 
        -> Result<WorkflowMetadata>
}
```

**Format Detection Logic**:
```rust
if workflow_data.get("nodes").is_some() {
    // UI format
} else {
    // API format
}
```

**Node Type Constants** (from Python version):
```rust
const SAMPLER_TYPES: &[&str] = &[
    "KSampler", "KSamplerAdvanced", "SamplerCustom", 
    "UltimateSDUpscale", "FreeU_V2", // ...
];

const MODEL_LOADER_TYPES: &[&str] = &[
    "CheckpointLoaderSimple", "UNETLoader", // ...
];
```

**Unit Tests Required**:
- Test with UI format workflow JSON
- Test with API format workflow JSON  
- Test edge cases (missing nodes, broken links)

### 3. Thumbnail Generator Module (`src-tauri/src/thumbnails.rs`)

**Purpose**: Generate and cache thumbnails for images and videos.

**Key Functions**:

```rust
use image::{ImageFormat, GenericImageView};

pub async fn create_thumbnail(
    file_path: &Path,
    thumbnail_dir: &Path,
    file_hash: &str,
    file_type: &str,
    quality: u8
) -> Result<PathBuf>

// Image thumbnail generation
async fn create_image_thumbnail(
    source: &Path,
    dest: &Path,
    max_size: u32,
    quality: u8
) -> Result<()> {
    let img = image::open(source)?;
    let thumb = img.thumbnail(max_size, max_size);
    thumb.save_with_format(dest, ImageFormat::Jpeg)?;
    Ok(())
}

// Video thumbnail via ffmpeg
async fn create_video_thumbnail(
    source: &Path,
    dest: &Path,
    ffmpeg_path: Option<&str>
) -> Result<()> {
    use tokio::process::Command;
    
    let ffmpeg = ffmpeg_path.unwrap_or("ffmpeg");
    let output = Command::new(ffmpeg)
        .args(&[
            "-i", source.to_str().unwrap(),
            "-ss", "00:00:01",
            "-vframes", "1",
            "-q:v", "2",
            dest.to_str().unwrap()
        ])
        .output()
        .await?;
    
    if !output.status.success() {
        return Err(anyhow::anyhow!("ffmpeg failed"));
    }
    
    Ok(())
}

// Generate file hash for thumbnail naming
pub fn generate_file_hash(path: &str, mtime: f64) -> String {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(format!("{}{}", path, mtime).as_bytes());
    format!("{:x}", hasher.finalize())
}
```

### 4. File Metadata Analysis

**Purpose**: Detect file type, dimensions, duration, and check for embedded workflows.

```rust
pub async fn analyze_file(file_path: &Path) -> Result<FileMetadata> {
    let file_type = detect_file_type(file_path)?;
    
    match file_type.as_str() {
        "image" => analyze_image(file_path).await,
        "video" => analyze_video(file_path).await,
        "audio" => analyze_audio(file_path).await,
        _ => Ok(FileMetadata::default()),
    }
}

struct FileMetadata {
    file_type: String,
    dimensions: Option<String>,
    duration: Option<String>,
    has_workflow: bool,
}
```

## Phase 3: Frontend Migration (SvelteKit UI)

### Component Structure

Create these components in `src/lib/components/`:

1. **Sidebar.svelte**
```svelte
<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import type { FolderEntry } from '$lib/types';
  
  let folders = $state<FolderEntry[]>([]);
  let searchQuery = $state('');
  
  async function loadFolders() {
    folders = await invoke<FolderEntry[]>('get_folders');
  }
</script>
```

2. **GalleryGrid.svelte**
```svelte
<script lang="ts">
  import type { FileEntry } from '$lib/types';
  import GalleryItem from './GalleryItem.svelte';
  
  let { files = [] }: { files: FileEntry[] } = $props();
</script>

<div class="gallery-grid">
  {#each files as file (file.id)}
    <GalleryItem {file} />
  {/each}
</div>
```

3. **FilterPanel.svelte** - Port from Alpine.js template
4. **Lightbox.svelte** - Full-screen modal viewer
5. **Notification.svelte** - Toast messages

### State Management (`src/lib/store.ts`)

```typescript
import type { FileEntry, FilterOptions } from './types';

// Svelte 5 runes for reactive state
export const files = $state<FileEntry[]>([]);
export const selectedFiles = $state<Set<string>>(new Set());
export const filters = $state<FilterOptions>({
  search: '',
  file_types: [],
  extensions: [],
  favorites_only: false,
  // ...
});

export const currentPage = $state(1);
export const totalPages = $state(1);
export const isLoading = $state(false);
```

### Tauri Events Integration

```typescript
import { listen } from '@tauri-apps/api/event';

interface SyncProgressPayload {
  current: number;
  total: number;
  status: string;
  message?: string;
}

listen<SyncProgressPayload>('sync-progress', (event) => {
  const { current, total, status } = event.payload;
  // Update UI progress bar
});

listen('sync-complete', () => {
  // Refresh file list
  loadFiles();
});
```

## Phase 4: Integration & Polish

### Tauri Commands to Expose

```rust
// In lib.rs, add to invoke_handler:
.invoke_handler(tauri::generate_handler![
    greet,
    init_database,
    sync_folder,
    get_files,
    get_file_by_id,
    get_folders,
    update_favorite,
    delete_files,
    move_files,
    rename_file,
    get_workflow_metadata,
    search_files,
])
```

### Error Handling Pattern

```rust
#[tauri::command]
async fn get_files(
    folder_key: String,
    filters: FilterOptions,
    pagination: PaginationParams,
    state: State<'_, AppState>
) -> Result<PaginatedResult<FileEntry>, String> {
    let db_pool = state.db_pool.lock().unwrap();
    let pool = db_pool.as_ref().ok_or("Database not initialized")?;
    
    database::query_files(pool, &filters, &pagination, Some(&folder_key))
        .await
        .map_err(|e| format!("Failed to query files: {}", e))
}
```

Frontend error handling:
```typescript
try {
  const result = await invoke('get_files', { 
    folderKey, filters, pagination 
  });
  files = result.items;
} catch (error) {
  console.error('Failed to load files:', error);
  showNotification('Error loading files', 'error');
}
```

## Phase 5: Build & Distribution

### Configure `tauri.conf.json`

```json
{
  "identifier": "com.smartgallery.app",
  "productName": "SmartGallery",
  "version": "2.0.0",
  "bundle": {
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ],
    "targets": ["msi", "dmg", "deb", "appimage"]
  }
}
```

### Security Capabilities

Edit `src-tauri/capabilities/default.json`:

```json
{
  "permissions": [
    "core:default",
    "fs:read-all",
    "fs:write-all",
    "shell:allow-execute"
  ]
}
```

### GitHub Actions CI/CD

Create `.github/workflows/build.yml`:

```yaml
name: Build

on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    strategy:
      matrix:
        platform: [macos-latest, ubuntu-latest, windows-latest]
    
    runs-on: ${{ matrix.platform }}
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Node
        uses: actions/setup-node@v4
        with:
          node-version: 20
      
      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable
      
      - name: Install dependencies (Ubuntu)
        if: matrix.platform == 'ubuntu-latest'
        run: |
          sudo apt-get update
          sudo apt-get install -y libwebkit2gtk-4.1-dev \
            build-essential curl wget file libssl-dev \
            libayatana-appindicator3-dev librsvg2-dev
      
      - name: Install frontend dependencies
        run: |
          cd smartgallery-tauri
          npm install
      
      - name: Build
        run: |
          cd smartgallery-tauri
          npm run tauri build
      
      - name: Upload artifacts
        uses: actions/upload-artifact@v4
        with:
          name: smartgallery-${{ matrix.platform }}
          path: smartgallery-tauri/src-tauri/target/release/bundle/
```

## Testing Strategy

### Rust Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_database_init() {
        let db_path = std::env::temp_dir().join("test.db");
        let pool = init_db(&db_path).await.unwrap();
        // Assert tables exist
    }
    
    #[test]
    fn test_workflow_parser_ui_format() {
        let json = include_str!("../test_data/workflow_ui.json");
        let workflow: serde_json::Value = serde_json::from_str(json).unwrap();
        let parser = WorkflowParser::new(workflow, Path::new("test.png")).unwrap();
        let metadata = parser.extract_metadata().unwrap();
        assert!(!metadata.is_empty());
    }
}
```

### Integration Tests

```rust
// tests/integration_test.rs
use smartgallery_tauri_lib::*;

#[tokio::test]
async fn test_full_workflow() {
    // 1. Init database
    // 2. Scan test directory
    // 3. Extract workflows
    // 4. Query files
    // 5. Assert results
}
```

## Performance Targets

Based on Python version benchmarks:

- **Database queries**: < 50ms for filtered queries
- **Thumbnail generation**: < 100ms per image
- **Full directory scan**: 10-20x faster than Python (parallel processing)
- **Memory usage**: < 200MB for 10,000 files
- **Startup time**: < 2 seconds

## Migration Checklist

Use this checklist to track progress:

- [x] Phase 1: Foundation complete
- [x] Database module implemented
- [ ] Scanner module implemented
- [ ] Parser module implemented
- [ ] Thumbnail module implemented
- [ ] All Tauri commands exposed
- [ ] Sidebar component created
- [ ] Gallery grid component created
- [ ] Filter panel component created
- [ ] Lightbox component created
- [ ] State management implemented
- [ ] Event system integrated
- [ ] Error handling comprehensive
- [ ] Build configuration complete
- [ ] CI/CD pipeline set up
- [ ] Windows installer tested
- [ ] macOS installer tested
- [ ] Linux installer tested
- [ ] Documentation updated

## Estimated Timeline

- **Remaining Phase 2**: 3-4 days (scanner, parser, thumbnails)
- **Phase 3**: 3-4 days (frontend components)
- **Phase 4**: 2-3 days (integration, testing)
- **Phase 5**: 1-2 days (build, CI/CD)

**Total remaining**: ~10-13 days of focused development

## Resources

- [Tauri Documentation](https://tauri.app)
- [SvelteKit Documentation](https://kit.svelte.dev)
- [SQLx Documentation](https://docs.rs/sqlx)
- [Rust Async Book](https://rust-lang.github.io/async-book/)
- Original Python implementation: `smartgallery.py`

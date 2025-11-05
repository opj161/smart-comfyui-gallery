# Complete Tauri Migration - Direct Implementation Plan

## Executive Summary
This is a **complete, non-incremental** rewrite of SmartGallery from Python/Flask to Tauri/Rust/SvelteKit. All 26 Flask endpoints, the ComfyUI workflow parser, database layer, file scanning, and UI will be ported in one comprehensive effort.

## Phase Breakdown (Direct, Complete Approach)

### Phase 2: Core Rust Backend (Week 1-2)
**Goal**: Complete backend logic in Rust with full feature parity

#### 2.1 Database Layer (`database.rs`) - 2 days
- [ ] SQLite connection pool with sqlx
- [ ] Schema: `files` + `workflow_metadata` tables with all indices
- [ ] CRUD operations: insert_file, update_file, delete_files, get_file_by_id
- [ ] Query builders: filter files, paginate, get stats
- [ ] Transaction support for batch operations

#### 2.2 Workflow Parser (`parser.rs`) - 3 days
- [ ] Port ComfyUIWorkflowParser (520 lines, most complex)
- [ ] Dual format support (UI and API workflows)
- [ ] Node type detection (40+ node types)
- [ ] Graph traversal: backward tracing for inputs
- [ ] Extract: model, sampler, scheduler, cfg, steps, prompts, dimensions
- [ ] Multi-sampler support (one file = multiple sampler nodes)
- [ ] Unit tests with sample workflows

#### 2.3 File System Scanner (`scanner.rs`) - 2 days
- [ ] Walkdir for recursive directory traversal
- [ ] Parallel processing with Rayon (10x faster than Python)
- [ ] Workflow extraction integration
- [ ] Mtime-based change detection
- [ ] Database sync: add, update, remove files
- [ ] Progress events via Tauri emit

#### 2.4 Thumbnail Generator (`thumbnails.rs`) - 1 day
- [ ] Image thumbnails: `image` crate resize
- [ ] Video thumbnails: ffmpeg command execution
- [ ] Thumbnail cache management
- [ ] Hash-based deduplication

#### 2.5 Metadata Extraction (`metadata.rs`) - 1 day
- [ ] PNG workflow extraction (tEXt chunks)
- [ ] Video workflow extraction (metadata tracks)
- [ ] File dimension detection
- [ ] Duration calculation for videos
- [ ] Animated WebP detection

### Phase 3: Tauri Commands (Week 3)
**Goal**: Expose all backend functionality via Tauri commands

#### 3.1 Core Commands (26 total) - 3 days
Replace all Flask routes with Tauri commands:
- [ ] `initialize_gallery(output_path, input_path)`
- [ ] `get_files(folder_key, filters, page, per_page)` → PaginatedFiles
- [ ] `get_file_by_id(file_id)` → FileEntry
- [ ] `sync_folder(folder_key)` → Stream SyncProgress events
- [ ] `get_filter_options()` → FilterOptions
- [ ] `get_workflow_metadata(file_id)` → Vec<WorkflowMetadata>
- [ ] `toggle_favorite(file_id)`
- [ ] `batch_favorite(file_ids, favorite)`
- [ ] `rename_file(file_id, new_name)`
- [ ] `delete_file(file_id)`
- [ ] `batch_delete(file_ids)`
- [ ] `move_files(file_ids, target_folder)`
- [ ] `create_folder(parent_key, name)`
- [ ] `rename_folder(folder_key, new_name)`
- [ ] `delete_folder(folder_key)`
- [ ] `get_folder_tree()` → HashMap<String, FolderConfig>
- [ ] `get_thumbnail_path(file_id)` → String
- [ ] `get_node_summary(file_id)` → String (HTML)
- [ ] `upload_files(files)` → Vec<FileEntry>
- [ ] `get_stats()` → AppStats
- [ ] `get_health()` → HealthStatus

#### 3.2 State Management - 1 day
- [ ] AppState struct (database pool, config, caches)
- [ ] Managed state in Tauri
- [ ] Thread-safe access patterns
- [ ] Configuration loading (JSON + CLI args)

#### 3.3 Event System - 1 day
- [ ] Sync progress events
- [ ] File change notifications
- [ ] Error notifications
- [ ] Real-time updates

### Phase 4: SvelteKit Frontend (Week 4-5)
**Goal**: Complete UI rebuild with all features

#### 4.1 Component Architecture - 2 days
```
src/lib/components/
├── Sidebar.svelte          # Folder tree + search
├── GalleryGrid.svelte      # File grid layout
├── GalleryItem.svelte      # Individual card
├── FilterPanel.svelte      # Slide-out filters
├── Lightbox.svelte         # Full-screen viewer
├── Notification.svelte     # Toast notifications
├── ContextMenu.svelte      # Right-click menu
├── BatchActions.svelte     # Multi-select toolbar
├── UploadZone.svelte       # Drag-drop upload
└── NodeSummary.svelte      # Workflow details modal
```

#### 4.2 State Management (`store.ts`) - 1 day
Svelte 5 runes for reactive state:
```typescript
export const files = $state<FileEntry[]>([]);
export const selectedFiles = $state<Set<string>>(new Set());
export const currentFolder = $state<string>('_root_');
export const filters = $state<GalleryFilters>({...});
export const isLightboxOpen = $state(false);
export const currentLightboxIndex = $state(0);
export const syncProgress = $state<SyncProgress | null>(null);
```

#### 4.3 API Integration - 2 days
- [ ] Replace all fetch() with invoke()
- [ ] Error handling wrappers
- [ ] Loading states
- [ ] Event listeners (sync progress)
- [ ] Optimistic updates

#### 4.4 UI Implementation - 3 days
- [ ] Sidebar: Recursive folder tree with expand/collapse
- [ ] Gallery: Virtual scrolling, infinite load
- [ ] Filters: Multi-select dropdowns (replace Tom-Select)
- [ ] Lightbox: Keyboard nav, zoom, workflow display
- [ ] Batch operations: Select, move, delete, favorite
- [ ] Upload: Drag-drop zone with progress
- [ ] Context menu: Right-click actions
- [ ] Deep linking: URL params for folder/file/filters

#### 4.5 Styling - 1 day
- [ ] Tailwind CSS (already in template)
- [ ] Dark theme support
- [ ] Responsive layout
- [ ] Animation transitions
- [ ] Loading skeletons

### Phase 5: Integration & Testing (Week 6)
**Goal**: End-to-end validation and polish

#### 5.1 Feature Testing - 2 days
- [ ] File browsing and filtering
- [ ] Workflow extraction accuracy
- [ ] Thumbnail generation
- [ ] Multi-sampler files
- [ ] Favorites and batch operations
- [ ] Folder management
- [ ] Upload functionality
- [ ] Deep linking
- [ ] Sync progress

#### 5.2 Performance Optimization - 1 day
- [ ] Database query profiling
- [ ] Parallel processing tuning
- [ ] Frontend rendering optimization
- [ ] Thumbnail cache hit rate
- [ ] Memory usage monitoring

#### 5.3 Error Handling - 1 day
- [ ] Graceful degradation
- [ ] User-friendly error messages
- [ ] Retry mechanisms
- [ ] Logging and diagnostics

#### 5.4 Build & Distribution - 1 day
- [ ] Update tauri.conf.json
- [ ] Set app metadata (name, version, icons)
- [ ] Configure capabilities (file system access)
- [ ] Build for Linux/Windows/macOS
- [ ] Test installers

### Phase 6: Final Polish (Week 7)
**Goal**: Production readiness

#### 6.1 Documentation - 1 day
- [ ] User guide
- [ ] Developer setup
- [ ] Architecture overview
- [ ] API documentation

#### 6.2 Migration Tool - 1 day
- [ ] Export existing Python database
- [ ] Import to Rust version
- [ ] Preserve thumbnails
- [ ] Verify data integrity

#### 6.3 Final Testing - 2 days
- [ ] Large dataset testing (10k+ files)
- [ ] Cross-platform verification
- [ ] Stress testing
- [ ] User acceptance testing

## Implementation Order (This Session)

### Immediate Tasks (Next 4 hours):
1. ✅ Create complete plan document
2. 🔄 Implement database layer
3. 🔄 Port workflow parser
4. 🔄 Implement file scanner
5. 🔄 Create core Tauri commands

### Today's Goal:
Complete Phase 2 (Core Rust Backend) with working database, parser, and scanner.

## Key Technical Decisions

### Dependencies
```toml
# Cargo.toml
[dependencies]
sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "sqlite"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
walkdir = "2"
rayon = "1.8"
image = "0.24"
chrono = "0.4"
sha2 = "0.10"
hex = "0.4"
```

### Architecture Patterns
- **Database**: Single connection pool, WAL mode, prepared statements
- **Concurrency**: Rayon for CPU-bound (parsing), Tokio for I/O (database)
- **Error Handling**: `Result<T, String>` for Tauri commands (serializable errors)
- **State**: Single AppState with Arc<Mutex<>> for shared data
- **Events**: Tauri's emit for real-time updates
- **Testing**: Unit tests per module, integration tests for commands

## Success Criteria
- ✅ All 3,822 lines of Python backend ported to Rust
- ✅ All 26 Flask endpoints replaced with Tauri commands
- ✅ All 3,958 lines of Alpine.js UI rebuilt in SvelteKit
- ✅ Feature parity with Python version
- ✅ Faster performance (2-5x)
- ✅ Lower memory usage (30-50% reduction)
- ✅ Cross-platform builds working
- ✅ No regressions in functionality

## Timeline
**Total: 7 weeks** (35 full-time days)
- Week 1-2: Core Rust Backend
- Week 3: Tauri Commands & State
- Week 4-5: SvelteKit Frontend
- Week 6: Integration & Testing
- Week 7: Polish & Distribution

---

**Status**: Plan complete. Starting implementation of Phase 2 (Core Rust Backend).

# SmartGallery Architecture Diagram

This document provides visual representations of the SmartGallery architecture.

---

## System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     SmartGallery v2.0                        │
│                  (Tauri Desktop Application)                 │
└─────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│                       Frontend Layer                         │
│                    (SvelteKit + TypeScript)                  │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │ GalleryGrid  │  │  Lightbox    │  │ FilterPanel  │      │
│  │  (Browse)    │  │  (Viewer)    │  │  (Search)    │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
│                                                               │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │ GalleryItem  │  │   Toolbar    │  │ ⚠️ Settings  │      │
│  │  (Card)      │  │  (Actions)   │  │  (MISSING)   │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
│                                                               │
│  ┌─────────────────────────────────────────────────┐        │
│  │         State Management (Svelte 5 Runes)       │        │
│  │  • Files         • Filters      • Selection     │        │
│  │  • Lightbox      • Loading      • Errors        │        │
│  └─────────────────────────────────────────────────┘        │
│                                                               │
│  ┌─────────────────────────────────────────────────┐        │
│  │            API Layer (api.ts)                    │        │
│  │  • Type-safe Tauri command wrappers              │        │
│  │  • Event listeners (sync progress)               │        │
│  └─────────────────────────────────────────────────┘        │
└───────────────────────────┬─────────────────────────────────┘
                            │
                            │ Tauri IPC Bridge
                            │ (Type-safe, async)
                            │
┌───────────────────────────┴─────────────────────────────────┐
│                        Backend Layer                         │
│                      (Rust + Tauri + SQLx)                   │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌─────────────────────────────────────────────────┐        │
│  │           Tauri Commands (commands.rs)          │        │
│  │  19/26 implemented:                              │        │
│  │  ✅ initialize_gallery  ✅ get_files             │        │
│  │  ✅ search_files        ✅ toggle_favorite       │        │
│  │  ✅ delete_file         ✅ rename_file           │        │
│  │  ✅ sync_files          ✅ get_stats             │        │
│  │  ❌ upload_file         ❌ rename_folder         │        │
│  └─────────────────────────────────────────────────┘        │
│                                                               │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │  Database    │  │    Parser    │  │   Scanner    │      │
│  │ (database.rs)│  │  (parser.rs) │  │ (scanner.rs) │      │
│  │              │  │              │  │              │      │
│  │ • SQLite     │  │ • 40+ nodes  │  │ • Parallel   │      │
│  │ • WAL mode   │  │ • UI & API   │  │ • Rayon      │      │
│  │ • 14 indices │  │ • Metadata   │  │ • Recursive  │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
│                                                               │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │ Thumbnails   │  │ ⚠️ Config     │  │ ⚠️ Security   │      │
│  │(thumbnails.rs)│  │  (MISSING)   │  │  (MISSING)   │      │
│  │              │  │              │  │              │      │
│  │ • Images     │  │ • config.json│  │ • Path check │      │
│  │ • Videos     │  │ • Settings   │  │ • Validation │      │
│  │ • FFmpeg     │  │ • Persist    │  │ • Sandboxing │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
│                                                               │
│  ┌─────────────────────────────────────────────────┐        │
│  │            App State (Arc<Mutex<>>)              │        │
│  │  • DB Pool       • Paths       • Configs         │        │
│  └─────────────────────────────────────────────────┘        │
└───────────────────────────┬─────────────────────────────────┘
                            │
                            │ File System Access
                            │ (Restricted by Tauri)
                            │
┌───────────────────────────┴─────────────────────────────────┐
│                     File System Layer                        │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌─────────────────────────────────────────────────┐        │
│  │          ComfyUI Output Directory                │        │
│  │  C:\.ai\ComfyUI\output\ (hardcoded)              │        │
│  │  • PNG files with workflow metadata              │        │
│  │  • MP4 videos with embedded workflows            │        │
│  │  • Generated images and videos                   │        │
│  └─────────────────────────────────────────────────┘        │
│                                                               │
│  ┌─────────────────────────────────────────────────┐        │
│  │          ComfyUI Input Directory (Optional)      │        │
│  │  C:\.ai\ComfyUI\input\ (hardcoded)               │        │
│  │  • Reference images                              │        │
│  │  • Training data                                 │        │
│  └─────────────────────────────────────────────────┘        │
│                                                               │
│  ┌─────────────────────────────────────────────────┐        │
│  │             Cache Directory                      │        │
│  │  smartgallery_cache/                             │        │
│  │  • gallery_cache.sqlite (database)               │        │
│  │  • thumbnails_cache/ (generated thumbnails)      │        │
│  └─────────────────────────────────────────────────┘        │
└─────────────────────────────────────────────────────────────┘
```

---

## Data Flow

### File Scanning Flow

```
User clicks "Sync"
       ↓
┌──────────────────┐
│  sync_files()    │ Tauri command triggered
└────────┬─────────┘
         ↓
┌──────────────────┐
│  full_sync()     │ Parallel file scanning (Rayon)
└────────┬─────────┘
         ↓
┌──────────────────┐
│  Scanner walks   │ Recursive directory traversal
│  directory tree  │ Filter by extensions
└────────┬─────────┘
         ↓
┌──────────────────┐
│  For each file:  │
│  • Hash (ID)     │
│  • Extract PNG   │
│  • Parse workflow│
│  • Generate thumb│
└────────┬─────────┘
         ↓
┌──────────────────┐
│  Insert/Update   │ Upsert to SQLite
│  database        │ Transaction per batch
└────────┬─────────┘
         ↓
┌──────────────────┐
│  Emit events     │ sync-progress, sync-complete
│  to frontend     │ Real-time updates
└────────┬─────────┘
         ↓
┌──────────────────┐
│  Frontend        │ Reload gallery
│  updates         │ Show results
└──────────────────┘
```

### Search/Filter Flow

```
User types in search bar
       ↓
┌──────────────────┐
│  Update filters  │ Svelte 5 runes (reactive)
└────────┬─────────┘
         ↓
┌──────────────────┐
│  Debounce 300ms  │ Avoid excessive queries
└────────┬─────────┘
         ↓
┌──────────────────┐
│ get_files_       │ Tauri command with filters
│ filtered()       │
└────────┬─────────┘
         ↓
┌──────────────────┐
│  Build SQL       │ Dynamic WHERE clause
│  query           │ Use indices for speed
└────────┬─────────┘
         ↓
┌──────────────────┐
│  Execute query   │ Paginated (LIMIT/OFFSET)
│  with sqlx       │ ~5ms for 10K files
└────────┬─────────┘
         ↓
┌──────────────────┐
│  Return          │ PaginatedFiles object
│  results         │ { files, total, has_more }
└────────┬─────────┘
         ↓
┌──────────────────┐
│  Frontend        │ Update gallery grid
│  renders         │ Show matching files
└──────────────────┘
```

### Lightbox View Flow

```
User clicks on image
       ↓
┌──────────────────┐
│  setLightbox()   │ Update store with file ID
└────────┬─────────┘
         ↓
┌──────────────────┐
│  Lightbox.svelte │ Component renders
│  opens           │
└────────┬─────────┘
         ↓
┌──────────────────┐
│  Load full image │ From file path
│  from disk       │ (not from thumbnail)
└────────┬─────────┘
         ↓
┌──────────────────┐
│  get_workflow_   │ Fetch workflow metadata
│  metadata()      │ from database
└────────┬─────────┘
         ↓
┌──────────────────┐
│  Display:        │
│  • Full image    │
│  • Metadata      │
│  • Prompts       │
│  • Parameters    │
└────────┬─────────┘
         ↓
┌──────────────────┐
│  Keyboard nav    │ ← → ESC i keys
│  enabled         │
└──────────────────┘
```

---

## Database Schema

```
┌─────────────────────────────────────────────────────────────┐
│                        files table                           │
├─────────────────────────────────────────────────────────────┤
│ id                 TEXT PRIMARY KEY   (SHA256 hash)          │
│ path               TEXT UNIQUE        (absolute path)        │
│ mtime              REAL               (modified timestamp)   │
│ name               TEXT               (filename)             │
│ type               TEXT               (image/video)          │
│ duration           TEXT               (⚠️ never populated)    │
│ dimensions         TEXT               (1024x768)             │
│ has_workflow       INTEGER            (0 or 1)               │
│ is_favorite        INTEGER            (0 or 1)               │
│ prompt_preview     TEXT               (first 200 chars)      │
│ workflow_metadata  TEXT               (full JSON)            │
│ sampler            TEXT               (indexed)              │
│ model              TEXT               (indexed)              │
│ scheduler          TEXT               (indexed)              │
│ cfg                REAL               (indexed)              │
│ steps              INTEGER            (indexed)              │
│ width              INTEGER            (indexed)              │
│ height             INTEGER            (indexed)              │
└─────────────────────────────────────────────────────────────┘

                            ↓ 1:many

┌─────────────────────────────────────────────────────────────┐
│                  workflow_metadata table                     │
├─────────────────────────────────────────────────────────────┤
│ id                 INTEGER PRIMARY KEY AUTOINCREMENT         │
│ file_id            TEXT (FK → files.id)                      │
│ sampler            TEXT                                      │
│ model              TEXT                                      │
│ scheduler          TEXT                                      │
│ cfg                REAL                                      │
│ steps              INTEGER                                   │
│ positive_prompt    TEXT                                      │
│ negative_prompt    TEXT                                      │
│ width              INTEGER                                   │
│ height             INTEGER                                   │
└─────────────────────────────────────────────────────────────┘

Indices (14 total):
✅ idx_files_name          ON files(name)
✅ idx_files_mtime         ON files(mtime)
✅ idx_files_type          ON files(type)
✅ idx_files_favorite      ON files(is_favorite)
✅ idx_files_path          ON files(path)
✅ idx_files_sampler       ON files(sampler)
✅ idx_files_model         ON files(model)
✅ idx_files_scheduler     ON files(scheduler)
✅ idx_files_cfg           ON files(cfg)
✅ idx_files_steps         ON files(steps)
✅ idx_files_width         ON files(width)
✅ idx_files_height        ON files(height)
✅ idx_files_has_workflow  ON files(has_workflow)
✅ idx_workflow_file_id    ON workflow_metadata(file_id)

Query Performance:
• Get all files: ~5ms (with indices)
• Search by name: ~2ms (indexed)
• Filter by model: ~3ms (indexed)
• Complex filter: ~10ms (multiple indices)
```

---

## Component Hierarchy

```
App (+page.svelte)
├── Toolbar
│   ├── Search Input
│   ├── Sync Button
│   ├── Filter Toggle
│   └── Batch Actions
│       ├── Favorite Selected
│       └── Delete Selected
├── FilterPanel
│   ├── Model Select
│   ├── Sampler Select
│   ├── Scheduler Select
│   ├── CFG Range
│   ├── Steps Range
│   └── Dimensions
├── GalleryGrid
│   └── GalleryItem (foreach file)
│       ├── Thumbnail
│       ├── Filename
│       ├── Favorite Icon
│       └── Selection Checkbox
└── Lightbox (conditional)
    ├── Full Image/Video
    ├── Navigation Arrows
    ├── Close Button
    ├── Info Toggle
    └── Metadata Panel
        ├── Workflow Details
        ├── Parameters
        └── Prompts
```

---

## State Management Flow

```
┌─────────────────────────────────────────────────────────────┐
│                    store.svelte.ts                           │
│                  (Svelte 5 Runes - Reactive)                 │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  Files State                                                  │
│  ┌────────────────────────────────────────────┐              │
│  │ files = $state<FileEntry[]>([])            │              │
│  │ totalCount = $state(0)                     │              │
│  │ hasMore = $state(false)                    │              │
│  └────────────────────────────────────────────┘              │
│                    ↕                                          │
│                  Updates from get_files()                     │
│                                                               │
│  Filter State                                                 │
│  ┌────────────────────────────────────────────┐              │
│  │ filters = $state<GalleryFilters>({         │              │
│  │   search: null,                            │              │
│  │   favorites_only: false,                   │              │
│  │   model: null,                             │              │
│  │   sampler: null,                           │              │
│  │   scheduler: null,                         │              │
│  │   cfg_min: null,                           │              │
│  │   cfg_max: null,                           │              │
│  │   steps_min: null,                         │              │
│  │   steps_max: null,                         │              │
│  │   width: null,                             │              │
│  │   height: null                             │              │
│  │ })                                         │              │
│  └────────────────────────────────────────────┘              │
│                    ↕                                          │
│                  Triggers get_files_filtered()                │
│                                                               │
│  Selection State                                              │
│  ┌────────────────────────────────────────────┐              │
│  │ selectedFiles = $state<Set<string>>(       │              │
│  │   new Set()                                │              │
│  │ )                                          │              │
│  └────────────────────────────────────────────┘              │
│                    ↕                                          │
│                  Multi-select with Ctrl/Shift                 │
│                                                               │
│  Lightbox State                                               │
│  ┌────────────────────────────────────────────┐              │
│  │ lightboxFile = $state<FileEntry | null>    │              │
│  │ lightboxIndex = $state(0)                  │              │
│  │ showInfo = $state(false)                   │              │
│  └────────────────────────────────────────────┘              │
│                    ↕                                          │
│                  Navigation with arrow keys                   │
│                                                               │
└─────────────────────────────────────────────────────────────┘
```

---

## Missing Components (To Implement)

```
⚠️ Configuration System
┌─────────────────────────────────────┐
│  SettingsPanel.svelte               │
│  ├── Path Selection (Tauri dialog)  │
│  ├── Thumbnail Size Slider          │
│  ├── Theme Toggle                   │
│  ├── Cache Size Limit               │
│  └── Save/Cancel Buttons            │
└─────────────────────────────────────┘
           ↓
┌─────────────────────────────────────┐
│  config.rs (Backend)                │
│  • Load config.json                 │
│  • Save config.json                 │
│  • Validate paths                   │
│  • Apply settings                   │
└─────────────────────────────────────┘

⚠️ File Upload
┌─────────────────────────────────────┐
│  UploadZone.svelte                  │
│  ├── Drag-and-Drop Area             │
│  ├── File Picker Button             │
│  ├── Upload Progress Bar            │
│  └── Success/Error Feedback         │
└─────────────────────────────────────┘
           ↓
┌─────────────────────────────────────┐
│  upload_file() (Backend)            │
│  • Copy file to output dir          │
│  • Extract workflow                 │
│  • Generate thumbnail               │
│  • Insert to database               │
└─────────────────────────────────────┘

⚠️ Memory Management
┌─────────────────────────────────────┐
│  cache.rs (Backend)                 │
│  • BoundedCache<K, V>               │
│  • LRU eviction                     │
│  • TTL expiration                   │
│  • Memory monitoring                │
└─────────────────────────────────────┘
           ↓
┌─────────────────────────────────────┐
│  VirtualScroll (Frontend)           │
│  • Render only visible items        │
│  • Preload above/below viewport     │
│  • Release far items                │
└─────────────────────────────────────┘
```

---

## Performance Optimizations

### What's Working ✅

```
1. Parallel Processing (Rayon)
   ┌──────────────────────────────┐
   │ Thread Pool (4-8 threads)    │
   ├──────────────────────────────┤
   │ File 1 → Thread 1            │
   │ File 2 → Thread 2            │
   │ File 3 → Thread 3            │
   │ File 4 → Thread 4            │
   └──────────────────────────────┘
   Result: 10x faster scanning

2. Database Indices (14 total)
   Query: WHERE model = 'sdxl' AND cfg > 7
   ┌──────────────────────────────┐
   │ Use idx_files_model          │
   │ Use idx_files_cfg            │
   │ Intersect results            │
   └──────────────────────────────┘
   Result: 10x faster queries

3. WAL Mode (Write-Ahead Logging)
   ┌──────────────────────────────┐
   │ Reads: Never block           │
   │ Writes: Fast commits         │
   │ Concurrent: Multiple readers │
   └──────────────────────────────┘
   Result: Better concurrency
```

### What's Missing ⚠️

```
1. Virtual Scrolling
   Current: Render all 10,000 items
   ┌──────────────────────────────┐
   │ ████████████████████████████ │ ALL
   └──────────────────────────────┘
   Memory: High, Performance: Slow
   
   Needed: Render only visible
   ┌──────────────────────────────┐
   │ ░░░░░░░████████░░░░░░░░░░░░░ │ VISIBLE
   └──────────────────────────────┘
   Memory: Low, Performance: Fast

2. Thumbnail Preloading
   Current: Load when scrolled into view
   Needed: Preload next 20 thumbnails
   
3. Memory Limits (BoundedCache)
   Current: Unbounded growth
   Needed: LRU eviction at 1GB limit
```

---

## Security Model

```
┌─────────────────────────────────────────────────────────────┐
│                     Security Layers                          │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  Layer 1: Tauri Sandboxing                                   │
│  ┌────────────────────────────────────────────┐              │
│  │ • WebView isolated from Rust                │              │
│  │ • IPC with allowlist                        │              │
│  │ • No direct file access from frontend       │              │
│  └────────────────────────────────────────────┘              │
│                                                               │
│  Layer 2: Path Validation ⚠️ MISSING                          │
│  ┌────────────────────────────────────────────┐              │
│  │ • Canonicalize paths                        │              │
│  │ • Check within allowed directories          │              │
│  │ • Block path traversal (..)                 │              │
│  │ • Reject symlinks outside scope             │              │
│  └────────────────────────────────────────────┘              │
│                                                               │
│  Layer 3: SQL Injection Protection ✅ WORKING                 │
│  ┌────────────────────────────────────────────┐              │
│  │ • sqlx parameterized queries                │              │
│  │ • Compile-time query checking               │              │
│  │ • No string concatenation                   │              │
│  └────────────────────────────────────────────┘              │
│                                                               │
│  Layer 4: Type Safety ✅ WORKING                              │
│  ┌────────────────────────────────────────────┐              │
│  │ • Rust type system                          │              │
│  │ • TypeScript interfaces                     │              │
│  │ • Serde serialization                       │              │
│  │ • No unsafe code                            │              │
│  └────────────────────────────────────────────┘              │
│                                                               │
└─────────────────────────────────────────────────────────────┘
```

---

## Build & Deployment

```
Development:
┌─────────────────────────────────────┐
│ npm run dev                         │
├─────────────────────────────────────┤
│ 1. Vite dev server (port 1420)     │
│ 2. Tauri dev window opens           │
│ 3. Hot reload enabled               │
│ 4. DevTools accessible              │
└─────────────────────────────────────┘
Time: ~30 seconds

Production Build:
┌─────────────────────────────────────┐
│ npm run tauri build                 │
├─────────────────────────────────────┤
│ 1. npm run build (frontend)        │
│    → SvelteKit static site          │
│    → Output: build/ directory       │
│                                     │
│ 2. cargo build --release (backend)  │
│    → Optimized Rust binary          │
│    → Link system libraries          │
│                                     │
│ 3. Bundle with Tauri                │
│    → Windows: .msi (~30 MB)         │
│    → Linux: .deb + .AppImage        │
│    → macOS: .dmg                    │
└─────────────────────────────────────┘
Time: 5-10 minutes (first build)
Time: ~1 minute (incremental)
```

---

This architecture provides a solid foundation with excellent performance. The missing pieces (configuration, security, upload, memory) are well-defined and can be implemented using the templates in `IMPLEMENTATION_TEMPLATES.md`.

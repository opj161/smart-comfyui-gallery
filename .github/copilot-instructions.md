# SmartGallery - AI Agent Instructions

## Project Overview

**SmartGallery** is a standalone desktop application (formerly a ComfyUI plugin) that provides a web-based gallery for browsing, organizing, and analyzing AI-generated media files. The core innovation is **universal workflow extraction** - it parses embedded ComfyUI workflow JSON from PNG, video, and other media files to display generation parameters (models, prompts, samplers, CFG, steps, seeds).

**Architecture**: Flask backend + Alpine.js frontend wrapped in PyWebView for native desktop deployment via PyInstaller.

**Key Technologies**:
- Backend: Flask 3.0, SQLite with WAL mode, PIL/OpenCV for thumbnails, multiprocessing for parallel sync
- Frontend: Alpine.js 3.x with Tom-Select dropdowns, pure CSS (no framework)
- Desktop: PyWebView (EdgeHTML on Windows), Waitress WSGI server (production stability)
- Build: PyInstaller with custom `.spec` file, Nuitka comments in `main.py` for alternative compiler

## Critical Architecture Patterns

### 1. Dual-Mode Application Structure

SmartGallery can run as:
- **Desktop app**: `main.py` → PyWebView wrapper → launches Flask server in background thread
- **Standalone server**: `python smartgallery.py --output-path /path --input-path /path`

**Desktop mode critical patterns**:
```python
# main.py - MUST include at top level to prevent infinite process spawning
if __name__ == '__main__':
    multiprocessing.freeze_support()  # CRITICAL for PyInstaller
```

**Thread lifecycle management** (v2.1.0 fixes):
- Server runs in **NON-DAEMON thread** (allows proper cleanup)
- Uses `threading.Event()` for coordinated shutdown
- `atexit.register()` ensures cleanup on any termination
- `waitress` WSGI server (not Flask dev server) for production stability

### 2. Configuration Hierarchy

Config locations checked in order (see `main.py:load_config()`):
1. `%LOCALAPPDATA%\SmartGallery\config.json` (Windows, preferred)
2. `appdirs.user_data_dir("SmartGallery")/config.json` (cross-platform)
3. `config.json` next to executable (frozen) or in CWD (development)

**User data paths** (writable, outside bundled app):
- Config: User data dir (above)
- Database: `USER_DATA_PATH/smartgallery_cache/gallery_cache.sqlite`
- Thumbnails: `USER_DATA_PATH/thumbnails_cache/`
- Logs: `USER_DATA_PATH/smartgallery_logs/`

**Do NOT write to `sys._MEIPASS`** (PyInstaller temp dir, read-only).

### 3. Workflow Parser Architecture

The `ComfyUIWorkflowParser` class (lines 251-773) handles **both UI and API workflow formats** natively:

**Format detection**:
- UI format: Has `nodes` array, uses `links` array and `widget_idx_map`
- API format: Dict of `{node_id: node_data}`, inputs reference node IDs directly

**Key design principle**: **Fail gracefully per-field**. If model extraction fails, still extract samplers, prompts, etc.

**Multi-sampler support** (v1.39.0+):
- One file can have multiple sampler nodes → multiple DB rows in `workflow_metadata` table
- Each sampler gets `sampler_index` (0, 1, 2...)
- Frontend shows "2 samplers" badge with hover tooltip listing all names
- `files.sampler_names` stores comma-separated unique names for quick display

**Node traversal pattern**:
```python
def _get_input_source_node(self, node, input_name):
    """Backward trace: sampler → model loader → checkpoint"""
    # Handles Primitive nodes, link chains, format differences
```

**Supported node types** - comprehensive lists at top of file:
- `SAMPLER_TYPES`: KSampler, KSamplerAdvanced, SamplerCustom, UltimateSDUpscale...
- `MODEL_LOADER_TYPES`: CheckpointLoaderSimple, UNETLoader, DualCLIPLoader...
- `PROMPT_NODE_TYPES`: CLIPTextEncode, CLIPTextEncodeSDXL...
- `SCHEDULER_NODE_TYPES`: BasicScheduler, KarrasScheduler...

**Debug mode** (disabled in production):
```python
DEBUG_WORKFLOW_EXTRACTION = False  # Line 925
# When True, saves workflow processing stages to output/workflow_debug/{filename}/
```

### 4. Database Schema & Performance

**Schema version**: Track in code comments (currently v1.50+, has `prompt_preview` and `sampler_names`)

**Critical tables**:
```sql
files (id, path, name, type, mtime, has_workflow, is_favorite, 
       prompt_preview TEXT,  -- First 150 chars of positive prompt
       sampler_names TEXT)   -- Comma-separated unique sampler names

workflow_metadata (id, file_id, sampler_index, model_name, sampler_name, 
                   scheduler, cfg, steps, positive_prompt, negative_prompt, 
                   width, height)
-- UNIQUE INDEX on (file_id, sampler_index)
```

**Performance indices** (v1.41.0 - CRITICAL):
```python
# Files table - enable O(log n) queries
idx_files_name, idx_files_mtime, idx_files_type, idx_files_favorite, idx_files_path

# Workflow metadata - fast filtering
idx_model_name, idx_sampler_name, idx_scheduler, idx_cfg, idx_steps, idx_width, idx_height
```

**Pagination pattern** (v1.41.0 fix):
```python
# ❌ OLD (memory bloat): Load ALL files, slice in Python
# ✅ NEW (efficient): True SQL pagination
SELECT * FROM files WHERE ... LIMIT ? OFFSET ?
SELECT COUNT(*) FROM files WHERE ...  # Separate count query
```

**Multi-sampler filtering** (v1.39.0+):
```python
# Use EXISTS subquery to prevent duplicate file results
WHERE EXISTS (
    SELECT 1 FROM workflow_metadata wm 
    WHERE wm.file_id = f.id AND wm.model_name = ?
)
```

### 5. Memory Management (v2.1.0 Critical Fixes)

**BoundedCache class** (lines 970-1067):
```python
# Replaces unbounded dicts that caused memory leaks
_filter_options_cache = BoundedCache(max_size=50, ttl_seconds=300)
request_timing_log = BoundedCache(max_size=500, ttl_seconds=600)
```

**Features**: Thread-safe, TTL expiration, LRU eviction, built-in statistics

**PIL operations** - ALWAYS use context manager:
```python
with safe_image_operation(filepath) as img:
    # Prevents file handle leaks in long-running apps
    img.thumbnail((size, size))
```

**Multiprocessing sync** - passes `debug_dir` as parameter (not global) to workers

### 6. Flask Route Patterns

**Initialization guard**:
```python
@app.route('/galleryout/...')
@require_initialization  # Ensures initialize_gallery() was called
def endpoint():
    conn = get_db()  # Uses Flask g object, auto-cleanup via teardown
```

**Database connection** (lines 1709-1754):
- Stored in `g.db` (per-request)
- WAL mode + performance PRAGMAs
- Auto-closed via `teardown_appcontext`

**SSE streaming** (real-time sync progress):
```python
@app.route('/galleryout/sync_folder')
def sync_folder():
    def generate():
        for progress in sync_folder_on_demand(...):
            yield f"data: {json.dumps(progress)}\n\n"
    return Response(stream_with_context(generate()), mimetype='text/event-stream')
```

### 7. Frontend Architecture (Alpine.js)

**Single-component pattern** - one `x-data="gallery()"` manages all state (lines 2700-3900+)

**Key reactive properties**:
```javascript
files: [],              // Current page files
selectedFiles: new Set(), // Selection state
isLightboxOpen: false,  // Modal states
currentLightboxIndex: 0,
filters: { type: [], extension: [], ... }
```

**Tom-Select integration** (custom directive):
```javascript
Alpine.directive('tom-select', ...)  // Line 2490+
// Auto-syncs with x-model, handles multi-select plugins
```

**Global stores**:
```javascript
Alpine.store('gallery', { folders, currentFolderKey, expandedFolderKeys })
Alpine.store('notifications', { list, add(), remove() })
```

**Deep-linking pattern** (v1.36+):
```javascript
// URL params: ?folder=subfolder&file_id=abc123&sort=date&order=desc
// On load, parse params → set filters → scroll to file → open lightbox
```

**SSE event handling** (non-blocking sync):
```javascript
const eventSource = new EventSource('/galleryout/sync_folder');
eventSource.onmessage = (event) => {
    const data = JSON.parse(event.data);
    // Update progress overlay, refresh on complete
};
```

## Development Workflows

### Testing Locally (Python)
```bash
# Development mode (Flask dev server)
python smartgallery.py --output-path C:/AI/Output --input-path C:/AI/Input

# Desktop mode
python main.py  # Reads config.json from user data dir or CWD
```

### Building Executable
```bash
# Prerequisites
pip install -r requirements.txt  # Includes waitress>=3.0.2
# Place ffprobe.exe in bin/ directory

# Build
pyinstaller smartgallery.spec  # Or: pyinstaller --clean smartgallery.spec

# Output: dist/SmartGallery.exe (~50-80 MB with UPX)
```

**Spec file critical sections**:
- `datas`: Templates, static folders, ffprobe binary
- `hiddenimports`: Flask, pywebview, waitress, PIL
- `console=False`: Hides console window (set True for debugging)
- `icon='assets/icon.ico'`: Application icon

**Nuitka alternative**: See `main.py` comments (lines 3-16) for Nuitka project directives

### Database Migrations

**Pattern** (see `initialize_gallery()` lines 2580-2595):
```python
cursor = conn.execute("PRAGMA table_info(files)")
columns = [row['name'] for row in cursor.fetchall()]
if 'new_column' not in columns:
    logging.info("Migrating: Adding new_column...")
    conn.execute("ALTER TABLE files ADD COLUMN new_column TYPE")
conn.commit()
```

**Safe for existing DBs**: New columns accept NULL, existing queries work unchanged

### Debugging Workflow Extraction

```python
# In smartgallery.py line 925
DEBUG_WORKFLOW_EXTRACTION = True  # Enable debug output

# Restart app, process files, check output/workflow_debug/{filename}/
# Files: 01_raw.json, 02_parsed.json, 03_format_detection.txt, 
#        04_parser_input.json, 05_parser_output.json
```

## Common Pitfalls & Solutions

1. **"Address already in use" on restart**
   - Root cause: Server thread not cleaned up
   - Solution: v2.1.0 fixed with non-daemon threads + shutdown events

2. **Memory grows unbounded**
   - Root cause: Caches without eviction
   - Solution: Use `BoundedCache` (v2.1.0), not plain dicts

3. **Infinite process spawning (PyInstaller)**
   - Root cause: Module-level code re-runs in worker processes
   - Solution: `multiprocessing.freeze_support()` at top of `__main__`

4. **PIL file handle leaks**
   - Root cause: Image objects not closed in long-running apps
   - Solution: `with safe_image_operation(path) as img:` context manager

5. **Tom-Select not clearing**
   - Root cause: Instances not stored globally (v1.40.6 bug)
   - Solution: Store in `tomSelectInstances` object, call `.clear()` API

6. **Duplicate files in results (multi-sampler)**
   - Root cause: JOIN creates duplicate rows when file has 2+ samplers
   - Solution: Use `EXISTS` subquery, not JOIN (v1.39.0 fix)

## Code Style Conventions

- **SQL**: Uppercase keywords, formatted with line breaks
- **Logging**: Use `logging.info()`, not `print()`, except in `main.py` config loading
- **Error handling**: Log with traceback, return JSON `{status: 'error', message: ...}`
- **Comments**: Multi-line docstrings for functions, inline `# CRITICAL:` for gotchas
- **Filenames**: `snake_case` for Python, `kebab-case` for static assets

## Key Files Reference

- `main.py`: Desktop app entry point, PyWebView wrapper, config loading
- `smartgallery.py`: Flask app, all backend logic (3821 lines, well-documented)
- `templates/index.html`: Complete frontend (Alpine.js SPA, 3958 lines)
- `smartgallery.spec`: PyInstaller build configuration
- `config.json.example`: Config template (copy to config.json)
- `BUILD_GUIDE.md`: Build instructions, troubleshooting, verification tests
- `CONFIGURATION.md`: Config options, paths, auto-detection behavior

## When Making Changes

1. **Backend changes**: Test both `python smartgallery.py` and `python main.py` modes
2. **Database schema**: Add migration in `initialize_gallery()`, update `init_db()`
3. **Workflow parser**: Add node types to constants at top, test with sample workflows
4. **Frontend state**: Update Alpine.js component in `index.html`, ensure reactivity
5. **Build changes**: Update `.spec` file, test `pyinstaller --clean`, verify executable
6. **Memory critical**: Use `BoundedCache` for any new caching, add cleanup handlers

## Testing Checklist

- [ ] Run `python smartgallery.py` (server mode)
- [ ] Run `python main.py` (desktop mode)
- [ ] Build with PyInstaller, run `dist/SmartGallery.exe`
- [ ] Test with large collection (1000+ files) - check memory stability
- [ ] Test workflow extraction with UI and API format workflows
- [ ] Test multi-sampler files (verify no duplicates in gallery)
- [ ] Test filtering (model, sampler, dimensions, date ranges)
- [ ] Test selection, favorites, delete, rename, move operations
- [ ] Close and reopen - verify clean shutdown (no orphaned processes)

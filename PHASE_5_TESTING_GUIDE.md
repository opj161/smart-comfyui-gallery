# Phase 5: Integration Testing & Validation Guide

## Overview
This document provides comprehensive testing instructions for the complete Tauri/Rust/SvelteKit migration of SmartGallery. All backend logic (1,565 lines) and UI components (1,620 lines) have been implemented and need end-to-end validation.

## Prerequisites

### System Requirements
- **Linux**: GTK3, WebKit2GTK (installed via apt)
- **macOS**: Xcode Command Line Tools
- **Windows**: WebView2 (pre-installed on Windows 11)
- **All Platforms**: Node.js 18+, Rust 1.70+

### Installation
```bash
cd tauri-sveltekit-main

# Install frontend dependencies
npm install

# Build frontend
npm run sveltekit:build

# Check Rust backend compiles
cd src-tauri && cargo check && cd ..
```

## Testing Categories

### 1. Automated Build & Compilation Tests ✅ (Can Run Automatically)

#### 1.1 Rust Backend Compilation
```bash
cd tauri-sveltekit-main/src-tauri
cargo check
cargo clippy -- -D warnings
cargo test
```

**Expected**: All checks pass, no compilation errors, warnings allowed for unused code.

**Current Status**: ✅ Compiles successfully with 29 warnings (unused functions - expected)

#### 1.2 Frontend Type Checking
```bash
cd tauri-sveltekit-main
npm run check
npm run lint
```

**Expected**: No TypeScript errors, all types match Rust models.

### 2. Unit Tests (Automated) ✅

#### 2.1 Workflow Parser Tests
Location: `src-tauri/src/parser.rs` (has `#[cfg(test)]` module)

```bash
cd tauri-sveltekit-main/src-tauri
cargo test parser::tests
```

**Test Coverage**:
- ✅ UI format workflow parsing
- ✅ API format workflow parsing
- ✅ Node type detection (40+ types)
- ✅ Multi-sampler extraction
- ✅ Prompt extraction (positive/negative)
- ✅ Model name extraction

#### 2.2 Thumbnail Tests
Location: `src-tauri/src/thumbnails.rs`

```bash
cargo test thumbnails::tests
```

**Test Coverage**:
- ✅ Hash generation
- ✅ Image resizing
- ✅ Aspect ratio preservation

### 3. Integration Tests (Manual - Requires User Data) 🔴

These tests require real ComfyUI output files and cannot be fully automated in CI.

#### 3.1 Database Initialization Test

**Test File**: `test_data/test_database.sh`

```bash
# Create test database
cd tauri-sveltekit-main
./test_data/test_database.sh
```

**Manual Verification**:
1. Run `npm run dev`
2. App should open in WebView window
3. Click "Initialize Gallery" button
4. Provide path to directory with PNG files
5. Check console for "Database initialized successfully"

**Expected Behavior**:
- Database file created at specified location
- `files` and `workflow_metadata` tables created with all indices
- No errors in terminal

#### 3.2 File Scanning & Workflow Extraction Test

**Requirements**:
- Directory with 10+ PNG files from ComfyUI
- Files should have embedded workflow JSON (tEXt chunk "workflow")

**Test Steps**:
1. Run `npm run dev`
2. Initialize gallery with test directory path
3. Click "Sync Files" button
4. Observe progress bar

**Expected Behavior**:
- Progress bar shows 0% → 100%
- Console shows "Processing file X/Y"
- On completion: "Sync complete: X files processed, Y workflows found"
- Gallery grid populates with file cards
- Files with workflows show "Workflow" badge
- Multi-sampler files show "2 samplers" badge

**Validation SQL**:
```sql
-- Check files table
SELECT COUNT(*) as total_files FROM files;

-- Check workflow metadata
SELECT COUNT(*) as total_metadata FROM workflow_metadata;

-- Check multi-sampler files
SELECT f.name, COUNT(wm.id) as sampler_count
FROM files f
JOIN workflow_metadata wm ON f.id = wm.file_id
GROUP BY f.id
HAVING COUNT(wm.id) > 1;
```

#### 3.3 Thumbnail Generation Test

**Test Steps**:
1. After sync completes, thumbnails should auto-generate
2. Check thumbnail cache directory (should be auto-created)
3. Open lightbox by clicking any file card

**Expected Behavior**:
- Thumbnails appear on file cards within 1-2 seconds
- Thumbnail cache directory contains `.jpg` files
- Thumbnails have hash-based filenames (e.g., `abc123def456.jpg`)
- Image thumbnails use Lanczos3 filtering (high quality)
- Video thumbnails show frame at 1 second

**Manual Check**:
```bash
# Check thumbnail directory
ls -lh ~/.local/share/smartgallery/thumbnails_cache/
# Should show JPG files with sizes 50-200KB
```

#### 3.4 Search & Filtering Test

**Test Steps**:
1. In gallery view, use search bar
2. Type partial file name
3. Type prompt keywords (if available)
4. Open filter panel (button in toolbar)
5. Select model from dropdown
6. Apply filters

**Expected Behavior**:
- Search results update instantly
- Only matching files shown
- Filter panel shows available options (models, samplers, schedulers)
- Combined search + filter works correctly
- "Clear filters" button resets all

**Test Cases**:
- Search: "landscape" → shows files with "landscape" in name or prompt
- Filter: Model = "sd_xl_base_1.0" → shows only files using that model
- Filter: Sampler = "euler" → shows files using Euler sampler
- Filter: Steps = min 20, max 50 → shows files in that range
- Combined: Search "portrait" + Model "sd15" + CFG 7-8

#### 3.5 Lightbox & Navigation Test

**Test Steps**:
1. Click any file card
2. Lightbox opens full-screen
3. Press → (right arrow)
4. Press ← (left arrow)
5. Press 'i' key
6. Press ESC

**Expected Behavior**:
- ✅ Lightbox opens immediately
- ✅ Image loads at full resolution
- ✅ → moves to next file
- ✅ ← moves to previous file
- ✅ 'i' toggles metadata sidebar
- ✅ Metadata sidebar shows:
  - All samplers (if multiple)
  - Model name
  - Scheduler
  - CFG, Steps
  - Positive/negative prompts
  - Dimensions
- ✅ ESC closes lightbox
- ✅ Click backdrop closes lightbox

#### 3.6 Favorites & Selection Test

**Test Steps**:
1. Click heart icon on a file card
2. Click checkbox on 3-5 file cards
3. In selection bar, click "Add to Favorites"
4. Toggle "Favorites Only" filter
5. Select favorites, click "Remove from Favorites"

**Expected Behavior**:
- ✅ Heart icon toggles filled/unfilled
- ✅ Checkbox selects file (blue highlight)
- ✅ Shift+click selects range
- ✅ Ctrl+click adds to selection
- ✅ Selection bar shows "X files selected"
- ✅ Batch favorite adds all selected
- ✅ Favorites filter shows only favorited files
- ✅ Database persists favorite status

#### 3.7 Batch Delete Test

**⚠️ DESTRUCTIVE TEST - Use test directory only**

**Test Steps**:
1. Select 2-3 test files
2. Click "Delete Selected" in toolbar
3. Confirm deletion in dialog

**Expected Behavior**:
- ✅ Confirmation dialog appears
- ✅ Shows count of files to delete
- ✅ On confirm: files removed from grid
- ✅ Files deleted from filesystem
- ✅ Files deleted from database
- ✅ Thumbnails deleted from cache

#### 3.8 Rename & Move Test

**Test Steps**:
1. Right-click file (context menu - if implemented)
2. OR: Use rename command via API test UI
3. Provide new filename
4. Check file system

**Expected Behavior**:
- ✅ File renamed on disk
- ✅ Database path updated
- ✅ Gallery reflects new name immediately

### 4. Performance Tests (Manual Validation) 🔴

#### 4.1 Large Dataset Test

**Requirements**: 1,000+ files

**Test Steps**:
1. Initialize gallery with large directory
2. Run sync
3. Measure time

**Benchmarks** (approximate):
- 1,000 files: < 30 seconds
- 5,000 files: < 2 minutes
- 10,000 files: < 5 minutes

**Metrics to Check**:
- Memory usage stable (< 500MB for 10k files)
- No memory leaks during sync
- UI remains responsive

#### 4.2 Parallel Processing Verification

**Check Logs**:
```
Processing file 1/1000...
Processing file 50/1000...  (should jump quickly, not sequential)
```

**Expected**: Rayon parallelism uses all CPU cores, processes multiple files simultaneously.

### 5. Cross-Platform Tests (Manual) 🔴

#### 5.1 Build Test - Linux
```bash
npm run build
# Check dist/ for .deb, .AppImage
```

#### 5.2 Build Test - macOS
```bash
npm run build
# Check dist/ for .dmg, .app
```

#### 5.3 Build Test - Windows
```bash
npm run build
# Check dist/ for .msi, .exe
```

## Test Results Template

```markdown
### Test Run: [Date]
**Tester**: [Name]
**Platform**: Linux/macOS/Windows
**Dataset**: [Number of files]

#### ✅ Passed Tests
- [ ] Backend compilation
- [ ] Frontend type checking
- [ ] Database initialization
- [ ] File scanning
- [ ] Workflow extraction
- [ ] Thumbnail generation
- [ ] Search functionality
- [ ] Filtering
- [ ] Lightbox navigation
- [ ] Favorites (individual)
- [ ] Favorites (batch)
- [ ] Delete (with confirmation)
- [ ] Rename files
- [ ] Performance (< 30s for 1k files)

#### ❌ Failed Tests
(List any failures with details)

#### 🐛 Bugs Found
(List bugs with reproduction steps)

#### 📊 Performance Metrics
- Sync time for X files: Y seconds
- Memory usage peak: Z MB
- UI responsiveness: Good/Fair/Poor
```

## Tests That Cannot Be Automated

### 1. Visual/UI Tests
- [ ] Layout responsiveness (desktop → tablet → mobile)
- [ ] Dark theme consistency
- [ ] Hover effects and transitions
- [ ] Loading states appearance
- [ ] Error message clarity

### 2. User Experience Tests
- [ ] Keyboard shortcuts work intuitively
- [ ] Drag-drop selection feels natural
- [ ] Filter panel slide animation smooth
- [ ] Lightbox transitions smooth
- [ ] Progress indicators accurate

### 3. Real-World Workflow Tests
- [ ] Organize 100+ files from actual ComfyUI session
- [ ] Find specific generation by searching prompt keywords
- [ ] Filter to specific model/sampler combination
- [ ] Review all generations in lightbox
- [ ] Mark favorites for later reference

## How to Start Testing (Step-by-Step)

### For New Testers:

1. **Setup** (5 minutes)
   ```bash
   cd tauri-sveltekit-main
   npm install
   npm run dev
   ```

2. **Initial Test** (2 minutes)
   - App window opens?
   - UI renders correctly?
   - No errors in terminal?

3. **Basic Functionality** (10 minutes)
   - Initialize with directory: `/path/to/comfyui/output`
   - Sync files (observe progress)
   - Browse gallery grid
   - Open lightbox
   - Try search

4. **Advanced Features** (15 minutes)
   - Test all filters
   - Test favorites
   - Test selection + batch operations
   - Test keyboard shortcuts

5. **Report Results**
   - Copy test results template above
   - Fill in results
   - Note any issues

## Known Limitations

- **No Python migration tool yet**: Old database not auto-imported
- **Deep linking not implemented**: URL params for folder/file
- **Context menu not implemented**: Right-click for file operations
- **Drag-drop upload not implemented**: Must use sync from existing directory

## Success Criteria for Phase 5

- [ ] All automated tests pass (build, compile, unit tests)
- [ ] All core features work with real data
- [ ] No data loss or corruption
- [ ] Performance meets benchmarks (< 30s for 1k files)
- [ ] UI is responsive and polished
- [ ] No critical bugs blocking user workflows

## Next Phase (Phase 6)

After Phase 5 testing validates all features work correctly:
- Build production installers for all platforms
- Write user documentation
- Create migration guide from Python version
- Final polish and release

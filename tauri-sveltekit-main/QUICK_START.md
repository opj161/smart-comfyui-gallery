# SmartGallery - Quick Start Testing Guide

## ✅ Automated Tests Complete!

All automated tests have passed:
- ✅ Rust backend compiles successfully
- ✅ Unit tests pass (2/2)
- ✅ Frontend builds successfully
- ✅ TypeScript type checking passes
- ✅ Configuration updated for production

## 🚀 How to Test the Application

### Step 1: Start Development Mode

```bash
cd tauri-sveltekit-main
npm run dev
```

**Expected**: Application window opens with the SmartGallery UI

### Step 2: Prepare Test Data

You need a directory with ComfyUI-generated PNG files that contain embedded workflow metadata. If you don't have one:

1. Use your existing ComfyUI output folder (e.g., `C:\ComfyUI\output`)
2. Or create a test directory with some PNG files from ComfyUI

### Step 3: Initialize the Gallery

1. In the app, you should see an "Initialize Gallery" option
2. Provide the path to your ComfyUI output directory
3. Wait for the database to be created

**Expected**: "Database initialized successfully" message

### Step 4: Sync Files

1. Click the "Sync" button (circular arrow icon)
2. Watch the progress bar fill from 0% to 100%

**Expected**:
- Progress updates in real-time
- Gallery grid populates with file cards
- Thumbnails appear on each card
- Files with workflows show a "Workflow" badge

### Step 5: Test Features

#### Search & Filter
- Type a filename or prompt keyword in the search bar
- Click "Filters" button (gear icon)
- Try filtering by:
  - Model (select from dropdown)
  - Sampler (select from dropdown)
  - CFG scale (enter min/max range)
  - Steps (enter min/max range)
  - Dimensions (enter width × height)

#### Lightbox
- Click any file card
- Use arrow keys (← →) to navigate
- Press 'i' to toggle metadata sidebar
- Press ESC to close

#### Favorites
- Click the heart icon on a file card
- Toggle "Favorites Only" filter to see only favorited files

#### Selection & Batch Operations
- Click checkboxes to select multiple files
- Hold Shift and click to select a range
- Hold Ctrl and click to add to selection
- Use batch buttons in toolbar:
  - "Add to Favorites"
  - "Remove from Favorites"
  - "Delete Selected" (⚠️ permanent!)

### Step 6: Verify Everything Works

**Checklist**:
- [ ] Gallery loads files from your directory
- [ ] Thumbnails generate and display correctly
- [ ] Search finds files by name/prompt
- [ ] Filters work (model, sampler, CFG, steps, dimensions)
- [ ] Lightbox opens and shows full-resolution images
- [ ] Metadata sidebar shows workflow details
- [ ] Favorites can be toggled
- [ ] Selection works (single, shift-click, ctrl-click)
- [ ] Batch operations work (favorite, delete)

### Step 7: Test Performance

Try with your real data:
- **Small dataset**: 100-500 files (should be instant)
- **Medium dataset**: 1,000-2,000 files (should take < 30 seconds to sync)
- **Large dataset**: 5,000-10,000 files (should take < 2 minutes to sync)

**Monitor**:
- Memory usage (should stay < 500MB even with 10k files)
- UI responsiveness (should remain smooth)
- No crashes or freezes

## 🐛 If You Encounter Issues

### Application Won't Start
```bash
# Check if ports are available
netstat -ano | findstr :5173

# Try clearing cache
rm -rf .svelte-kit
rm -rf node_modules/.vite
npm run dev
```

### Database Errors
- Check that the output directory exists and is writable
- Check that you have permission to create files in that location
- Try a different directory

### No Files Showing
- Verify the directory contains PNG files from ComfyUI
- Check that PNGs have embedded workflow metadata
- Look at the browser console (F12) for error messages

### Sync Takes Forever
- Check how many files are in the directory
- Large directories (10k+ files) will take 2-5 minutes on first sync
- Subsequent syncs should be faster (only new files are processed)

## 📦 Ready to Build for Production?

Once all tests pass:

```bash
cd tauri-sveltekit-main
npm run tauri build
```

This will:
1. Build the optimized frontend (1-2 minutes)
2. Compile Rust in release mode (10-15 minutes)
3. Create Windows installer (`.msi` file)

**Output location**: `src-tauri/target/release/bundle/msi/`

**Installer size**: ~25-35 MB

## 🎉 Success Criteria

Your testing is complete when:
- ✅ All features from the checklist work
- ✅ No crashes or errors during 10+ minutes of use
- ✅ Performance is acceptable for your dataset size
- ✅ Memory usage stays reasonable

## 📊 Reporting Results

If you want to document your testing:

1. Copy `TEST_RESULTS.md`
2. Fill in the manual testing checkboxes
3. Note any issues you found
4. Add your performance metrics (file count, sync time, memory usage)

## 🚀 Next Steps

After successful testing:
1. Run production build: `npm run tauri build`
2. Test the installer on a clean machine (optional but recommended)
3. Distribute to users!

## 💡 Tips

- **Use Favorites**: Mark your best generations for easy filtering later
- **Use Search**: Type prompt keywords to find specific styles/concepts
- **Use Filters**: Combine multiple filters to narrow down to specific parameters
- **Keyboard Shortcuts**: Learn the lightbox keys (← → for navigation, i for info, ESC to close)

## 📖 For More Details

- Full testing guide: `PHASE_5_TESTING_GUIDE.md`
- Build instructions: `PHASE_6_BUILD_GUIDE.md`
- Project architecture: `.github/copilot-instructions.md`

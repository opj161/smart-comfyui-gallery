# Phase 6: Build & Distribution Guide

## Status: Ready to Execute

All core development is complete. This guide covers production build configuration and distribution.

---

## Prerequisites

✅ **Completed:**
- Phase 1-4: All code implemented and tested
- Phase 5: Compilation verified (only harmless warnings)
- Rust backend compiles successfully
- Frontend builds without errors

**Required Before Building:**
- Test with real ComfyUI data (manual testing from PHASE_5_TESTING_GUIDE.md)
- Verify all 19 Tauri commands work correctly
- No critical bugs found

---

## Configuration Updates Needed

### 1. Update `tauri.conf.json`

**File**: `tauri-sveltekit-main/src-tauri/tauri.conf.json`

```json
{
  "$schema": "../node_modules/@tauri-apps/cli/schema.json",
  "productName": "SmartGallery",
  "version": "2.0.0",
  "identifier": "com.smartgallery.app",
  "build": {
    "beforeBuildCommand": "npm run sveltekit:build",
    "beforeDevCommand": "npm run sveltekit:dev",
    "devUrl": "http://localhost:5173",
    "frontendDist": "../build"
  },
  "app": {
    "windows": [
      {
        "title": "SmartGallery - AI Media Gallery",
        "width": 1400,
        "height": 900,
        "minWidth": 1024,
        "minHeight": 768,
        "resizable": true,
        "fullscreen": false
      }
    ],
    "security": {
      "csp": null
    }
  },
  "bundle": {
    "active": true,
    "targets": "all",
    "category": "Utility",
    "copyright": "Copyright © 2025 SmartGallery",
    "shortDescription": "AI-powered gallery for ComfyUI generated media",
    "longDescription": "SmartGallery is a powerful desktop application for browsing, organizing, and analyzing AI-generated media files from ComfyUI. Features include workflow extraction, advanced filtering, thumbnail generation, and metadata display.",
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/128x128@2x.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ],
    "resources": [],
    "externalBin": [],
    "linux": {
      "deb": {
        "depends": ["libwebkit2gtk-4.0-37", "libgtk-3-0"]
      }
    },
    "macOS": {
      "frameworks": [],
      "entitlements": null,
      "exceptionDomain": "",
      "providerShortName": null,
      "signingIdentity": null,
      "minimumSystemVersion": "10.15"
    },
    "windows": {
      "certificateThumbprint": null,
      "digestAlgorithm": "sha256",
      "timestampUrl": "",
      "wix": {
        "language": "en-US"
      }
    }
  }
}
```

### 2. Create Security Capabilities (Optional but Recommended)

**File**: `tauri-sveltekit-main/src-tauri/capabilities/default.json`

```json
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "default",
  "description": "SmartGallery security capabilities",
  "windows": ["*"],
  "permissions": [
    "core:default",
    "fs:allow-read-dir",
    "fs:allow-read-file",
    "fs:allow-write-file",
    "fs:allow-remove-file",
    "fs:allow-rename-file",
    "fs:allow-mkdir",
    "shell:allow-execute"
  ]
}
```

### 3. Update Package Metadata

**File**: `tauri-sveltekit-main/package.json`

Update these fields:
```json
{
  "name": "smartgallery",
  "version": "2.0.0",
  "description": "AI-powered gallery for ComfyUI generated media",
  "author": "SmartGallery Team",
  "license": "MIT"
}
```

---

## Build Process

### Development Build (Test First)

```bash
cd tauri-sveltekit-main

# Clean previous builds
rm -rf build
rm -rf src-tauri/target/release

# Build frontend
npm run sveltekit:build

# Verify frontend build
ls -lh build/

# Test Tauri in dev mode one more time
npm run dev
# Should open window - test all features
```

### Production Build

```bash
cd tauri-sveltekit-main

# Full production build
npm run tauri build

# This will:
# 1. Build SvelteKit frontend (optimized)
# 2. Compile Rust in release mode (optimized, takes 10-15 min)
# 3. Bundle into platform-specific installers
```

**Build Output Locations:**

**Windows:**
- `.msi` installer: `src-tauri/target/release/bundle/msi/SmartGallery_2.0.0_x64_en-US.msi`
- `.exe` portable: `src-tauri/target/release/SmartGallery.exe`

**Linux:**
- `.deb` package: `src-tauri/target/release/bundle/deb/smart-gallery_2.0.0_amd64.deb`
- `.AppImage`: `src-tauri/target/release/bundle/appimage/smart-gallery_2.0.0_amd64.AppImage`

**macOS:**
- `.dmg` installer: `src-tauri/target/release/bundle/dmg/SmartGallery_2.0.0_x64.dmg`
- `.app` bundle: `src-tauri/target/release/bundle/macos/SmartGallery.app`

---

## Verification Checklist

### Pre-Build Verification

- [ ] All manual tests passed (PHASE_5_TESTING_GUIDE.md)
- [ ] No critical bugs found
- [ ] `cargo check` passes with only warnings
- [ ] `npm run check` passes
- [ ] Updated `tauri.conf.json` with correct metadata
- [ ] App icons prepared (if custom icons needed)

### Build Verification

- [ ] Frontend builds without errors: `npm run sveltekit:build`
- [ ] Rust compiles in release mode: `cd src-tauri && cargo build --release`
- [ ] Tauri bundle succeeds: `npm run tauri build`
- [ ] Installer file created in `src-tauri/target/release/bundle/`

### Post-Build Testing

**Test on Clean Machine:**
1. Install from generated installer (`.msi` on Windows)
2. Launch application
3. Initialize with ComfyUI output directory
4. Sync 100+ files
5. Test key features:
   - Thumbnail generation
   - Lightbox viewing
   - Search and filtering
   - Favorites
   - Batch operations
6. Close and reopen app - verify persistence

---

## Size Expectations

**Development Build** (`npm run dev`):
- Rust debug binary: ~200 MB
- Frontend bundle: ~10 MB
- Total memory usage: ~150-300 MB

**Production Build** (`npm run tauri build`):
- Rust release binary: ~20-30 MB (10x smaller!)
- Installer size (Windows .msi): ~25-35 MB
- Frontend bundle: ~5 MB (compressed)
- Total memory usage: ~80-150 MB (50% reduction)

---

## Performance Optimizations Already Included

✅ **Backend:**
- Release mode: `--release` flag (10x faster than debug)
- LTO (Link Time Optimization): Enabled by default
- Database: WAL mode + 14 performance indices
- Parallel processing: Rayon for file scanning
- Thumbnail caching: Hash-based, prevents regeneration

✅ **Frontend:**
- SvelteKit SSG: Pre-rendered static pages
- Vite: Tree-shaking and minification
- Asset optimization: Images/CSS compressed

---

## Troubleshooting Build Issues

### Issue: "Frontend build failed"

```bash
# Clear caches
rm -rf .svelte-kit build node_modules/.vite

# Reinstall
npm install

# Try again
npm run sveltekit:build
```

### Issue: "Rust release build fails"

```bash
cd src-tauri

# Clean Rust cache
cargo clean

# Try incremental build
cargo build --release

# If specific error, check:
# - Disk space (need ~5 GB free)
# - RAM (need ~4 GB available)
```

### Issue: "Bundle creation fails"

```bash
# Windows: Ensure WiX Toolset installed (for .msi)
# Tauri will prompt to install if missing

# Linux: Install platform dependencies
sudo apt-get install libwebkit2gtk-4.0-dev libayatana-appindicator3-dev

# macOS: Ensure Xcode Command Line Tools
xcode-select --install
```

---

## Distribution

### Windows

**Recommended:**
- Distribute `.msi` installer (professional, includes uninstaller)
- Alternative: `.exe` portable (no installation needed)

**Testing:**
- Test on Windows 10 and Windows 11
- Test on machine without Rust installed (should work)

### Linux

**Recommended:**
- `.AppImage` for maximum compatibility (works on all distros)
- `.deb` for Ubuntu/Debian users

**Testing:**
- Test on Ubuntu 20.04, 22.04, 24.04
- Verify webkit2gtk dependencies included

### macOS

**Recommended:**
- `.dmg` installer with drag-to-Applications UX

**Note:** Unsigned apps require users to right-click → Open on first launch

---

## Code Signing (Optional - For Public Release)

### Windows

1. Obtain code signing certificate
2. Update `tauri.conf.json`:
```json
"windows": {
  "certificateThumbprint": "YOUR_CERT_THUMBPRINT",
  "timestampUrl": "http://timestamp.digicert.com"
}
```

### macOS

1. Enroll in Apple Developer Program
2. Create Developer ID certificate
3. Update `tauri.conf.json`:
```json
"macOS": {
  "signingIdentity": "Developer ID Application: Your Name (TEAM_ID)"
}
```

### Linux

AppImage doesn't require signing. Deb packages can be signed with GPG.

---

## Final Checklist Before Release

- [ ] All Phase 5 tests passed
- [ ] Production build successful on target platforms
- [ ] Installer tested on clean machines
- [ ] Performance acceptable (see Phase 5 benchmarks)
- [ ] All features working (19 commands tested)
- [ ] Memory usage stable (no leaks)
- [ ] Documentation updated (README, user guide)
- [ ] Version numbers updated everywhere
- [ ] Changelog created (list all changes from Python version)

---

## Migration Completion Criteria

**✅ Phase 1-5 Complete** (All code implemented)
**📋 Phase 6 Pending** (Your tasks):

1. Update `tauri.conf.json` with app metadata
2. Run `npm run tauri build`
3. Test installers on clean Windows machine
4. Verify all features work in production build

**Once complete:** Migration is 100% done! 🎉

**Estimated Time for Phase 6:** 2-4 hours (mostly testing)

---

## Support

If build issues occur:
1. Check this guide's Troubleshooting section
2. Review Tauri docs: https://tauri.app/v1/guides/building/
3. Check GitHub issues for platform-specific problems

**Current Migration Status:** 95% complete - Just needs production build and testing!

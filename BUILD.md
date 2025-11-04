# Building SmartGallery Standalone Executable

This document describes how to build SmartGallery as a standalone Windows executable using PyWebView and PyInstaller.

## Overview

SmartGallery can be packaged as a native desktop application that:
- Launches in a native window (using Microsoft Edge WebView2)
- Requires no Python installation for end users
- Includes all dependencies in a single executable
- Stores user data in the appropriate user directory
- Bundles ffprobe for video metadata extraction

## Prerequisites

1. **Python 3.10 or higher** (Python 3.13.3 tested)
2. **Microsoft Edge WebView2 Runtime** (pre-installed on Windows 10/11)
3. **FFmpeg tools** (specifically `ffprobe.exe` in the `bin` directory)

## Architecture

### Entry Point: `main.py`

The `main.py` script acts as a wrapper that:
1. Finds a free port for the Flask server
2. Starts the Flask server in a background daemon thread
3. Opens a PyWebView window pointing to the local server
4. Uses the `edgechromium` backend on Windows

### Application Module: `smartgallery.py`

The main application has been refactored to:
- Support both script and frozen (bundled) execution modes
- Store user data in `AppData\Local\SmartGallery` (via `appdirs`)
- Load templates and static files from `sys._MEIPASS` when frozen
- Look for bundled `ffprobe.exe` first before checking system PATH
- Provide `run_app()` and `main()` functions for different execution contexts

### Build Specification: `smartgallery.spec`

The PyInstaller spec file defines:
- Hidden imports for modules PyInstaller might miss
- Data files (templates, static assets) to include
- Binary files (ffprobe.exe) to bundle
- Executable settings (no console window, application icon)

## Building the Executable

### Step 1: Install Dependencies

```bash
pip install -r requirements.txt
```

This installs:
- `pywebview>=5.0.0` - Native window rendering
- `appdirs>=1.4.4` - Cross-platform user data directory
- `pyinstaller>=6.0.0` - Executable bundler
- All other SmartGallery dependencies

### Step 2: Verify FFprobe Binary

Ensure `bin/ffprobe.exe` exists:

```bash
ls bin/ffprobe.exe
```

If missing, download from [FFmpeg](https://ffmpeg.org/download.html) (essentials build).

### Step 3: Run PyInstaller

```bash
pyinstaller --clean smartgallery.spec
```

The `--clean` flag removes any cached files from previous builds.

### Step 4: Locate the Executable

The standalone executable will be created at:
```
dist/SmartGallery.exe
```

Size: Approximately 135-140 MB (includes Python runtime, all libraries, and assets).

## Running the Executable

### First Run

1. Double-click `SmartGallery.exe`
2. The application will create a user data directory at:
   ```
   C:\Users\<username>\AppData\Local\SmartGallery
   ```
3. Create a `config.json` file in this directory (or in the same directory as the executable):

```json
{
    "base_output_path": "C:/Path/To/Your/AI/Output",
    "base_input_path": "C:/Path/To/Your/AI/Input",
    "server_port": 8008
}
```

4. Restart the application to load your media gallery

### User Data Storage

The following are stored in the user data directory:
- `config.json` - User configuration (optional, can also be local)
- `.sqlite_cache/` - Database with indexed file metadata
- `.thumbnail_cache/` - Generated thumbnails
- `smartgallery_logs/` - Application logs

## Troubleshooting

### Build Errors

**Error: `ffprobe.exe not found`**
- Download ffprobe and place in `bin/` directory
- Video metadata extraction will not work without it

**Error: `icon.ico not found`**
- Ensure `assets/icon.ico` exists
- Or remove the `icon=` line from `smartgallery.spec`

**Error: Module not found during runtime**
- Add the module to `hiddenimports` in `smartgallery.spec`
- Rebuild with `--clean` flag

### Runtime Errors

**Error: WebView2 not available**
- Install [Microsoft Edge WebView2 Runtime](https://developer.microsoft.com/en-us/microsoft-edge/webview2/)
- Pre-installed on Windows 10/11 with updates

**Error: Application won't start**
- Check logs in `%LOCALAPPDATA%\SmartGallery\smartgallery_logs\`
- Verify config.json paths are correct
- Ensure paths exist and are readable

**Error: 503 Service Unavailable**
- The gallery hasn't been initialized yet
- Create a valid `config.json` with correct paths
- Restart the application

## Advanced Customization

### One-File Executable

To create a true single-file executable (slower startup, easier distribution):

1. Modify `smartgallery.spec`:
   ```python
   exe = EXE(
       pyz,
       a.scripts,
       [],  # Remove this line
       exclude_binaries=True,  # Add this line
       ...
   )
   
   coll = COLLECT(
       exe,
       a.binaries,
       a.zipfiles,
       a.datas,
       strip=False,
       upx=True,
       upx_exclude=[],
       name='SmartGallery',
   )
   ```

2. Or use the command-line flag:
   ```bash
   pyinstaller --onefile smartgallery.spec
   ```

### Custom Icon

Replace `assets/icon.ico` with your own icon file (must be .ico format).

### Debug Mode

To enable console output for debugging:

1. Change in `smartgallery.spec`:
   ```python
   console=True,  # Changed from False
   ```

2. Rebuild the executable

## Distribution

The `dist/SmartGallery.exe` file can be distributed to end users who:
1. Have Windows 10/11 with WebView2 runtime
2. Create a `config.json` with their media paths
3. Have the necessary storage space for database and thumbnails

No Python installation or other dependencies are required.

## Version History

- **v2.0.0** - Initial standalone executable support
  - PyWebView integration with edgechromium backend
  - User data directory support via appdirs
  - Bundled ffprobe for video metadata
  - Full PyInstaller packaging

## References

- [PyWebView Documentation](https://pywebview.flowrl.com/)
- [PyInstaller Manual](https://pyinstaller.org/en/stable/)
- [Edge WebView2 Runtime](https://developer.microsoft.com/en-us/microsoft-edge/webview2/)
- [SmartGallery Repository](https://github.com/opj161/smart-comfyui-gallery)

# Quick Start - Building SmartGallery.exe

## Prerequisites
- Python 3.8+ installed
- Windows 10/11 with Edge WebView2 runtime (pre-installed)

## Step 1: Install Dependencies
```powershell
pip install -r requirements.txt
```

**Key new dependency**: `waitress` (production WSGI server)

## Step 2: Test with Python (Optional but Recommended)
```powershell
python main.py
```

Expected output:
```
--- Attempting to load configuration ---
Checking for config at: C:\Users\...\SmartGallery\config.json
[OK] Successfully loaded configuration from: ...
Starting SmartGallery with waitress on 127.0.0.1:xxxxx
```

## Step 3: Build Executable
```powershell
pyinstaller smartgallery.spec
```

Expected output:
```
...
✓ Including ffprobe binary: bin\ffprobe.exe
...
Building EXE from EXE-00.toc completed successfully.
```

## Step 4: Run the Executable
```powershell
.\dist\SmartGallery.exe
```

## Troubleshooting

### Issue: "waitress not installed" warning
**Solution**: Run `pip install waitress`

### Issue: "No config.json found"
**Solution**: Create config.json in one of these locations:
- `%LOCALAPPDATA%\SmartGallery\config.json` (recommended)
- `config.json` (next to executable)

Example config.json:
```json
{
    "base_output_path": "C:/Path/To/Your/AI/Output",
    "base_input_path": "C:/Path/To/Your/AI/Input",
    "server_port": 8008
}
```

### Issue: "ffprobe binary not found" warning
**Solution**: Place `ffprobe.exe` in the `bin/` directory before building

### Issue: Memory leak still occurs
**Checklist**:
1. Verify waitress is installed: `pip show waitress`
2. Check console for "Starting SmartGallery with waitress" message
3. Rebuild with fresh spec: `pyinstaller --clean smartgallery.spec`
4. Check Task Manager when closing (should see clean shutdown logs)

## Verifying the Fix

### Test 1: Clean Shutdown
1. Run `SmartGallery.exe`
2. Browse some files
3. Close the window
4. Check Task Manager → No orphaned "SmartGallery.exe" processes

### Test 2: Memory Stability
1. Run `SmartGallery.exe`
2. Open Task Manager → Performance → Memory
3. Note initial memory usage (~150-200 MB)
4. Browse files for 5 minutes
5. Memory should stabilize, not continuously grow

### Test 3: Console Logs (Python)
Run `python main.py` and watch console output:
```
INFO:__main__:Creating PyWebView window...
INFO:__main__:Starting PyWebView...
INFO:__main__:Server thread starting on 127.0.0.1:xxxxx
INFO:root:Starting SmartGallery with waitress on 127.0.0.1:xxxxx
INFO:__main__:✓ Server ready at http://...
```

On close:
```
INFO:__main__:PyWebView window closed by user
INFO:__main__:Application cleanup starting...
INFO:__main__:Waiting for server thread to finish...
INFO:__main__:Server thread exiting
INFO:__main__:Server thread finished cleanly
INFO:__main__:Application cleanup complete
```

## Build Options

### Clean Build (Recommended)
```powershell
pyinstaller --clean smartgallery.spec
```

### Debug Build (Show Console)
Edit `smartgallery.spec`, change:
```python
console=False,  # Change to True for debugging
```

Then rebuild:
```powershell
pyinstaller --clean smartgallery.spec
```

## Distribution

The executable is located at:
```
dist\SmartGallery.exe
```

**Size**: ~50-80 MB (with UPX compression)

**Requirements for end users**:
- Windows 10/11 (Edge WebView2 runtime)
- Config.json file (see above)

## Performance Tips

1. **First Run**: Database sync may take time for large collections
2. **Thumbnail Cache**: Stored in `output_path/.thumbnails_cache/`
3. **Database**: Stored in `output_path/.sqlite_cache/gallery_cache.sqlite`
4. **Memory Usage**: ~150-300 MB typical (stable, no leaks)

## Support

If issues persist:
1. Check logs in console (if debug build)
2. Verify config.json paths are correct
3. Ensure waitress is installed
4. Report issue with console output on GitHub

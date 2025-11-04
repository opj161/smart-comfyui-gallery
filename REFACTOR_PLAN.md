# PyWebView Startup Function Pattern Refactor Plan

## Current Implementation vs. Best Practice

### Current Pattern (Manual Threading)
```python
# Current approach in main.py:
server_thread = threading.Thread(target=start_server, args=(host, port), daemon=True)
server_thread.start()
time.sleep(1.5)  # Manual wait
window = webview.create_window(title, url, ...)
webview.start(debug=False)
```

**Issues:**
- Manual thread management (not idiomatic)
- Hardcoded `time.sleep(1.5)` unreliable
- No server readiness verification
- Doesn't leverage pywebview's lifecycle

### Recommended Pattern (Startup Function)
```python
# Documented best practice:
window = webview.create_window(title, initial_html, ...)
webview.start(startup_func, window)  # Automatic threading

def startup_func(window):
    # Runs in separate thread automatically
    # Server starts here
    # Then load actual URL when ready
    window.load_url(url)
```

**Benefits:**
- ✅ Automatic thread management by pywebview
- ✅ Better lifecycle integration
- ✅ Cleaner separation of concerns
- ✅ No arbitrary sleep delays
- ✅ Follows documented patterns

---

## Implementation Strategy

### Phase 1: Refactor Server Startup Function

**Before:**
```python
def start_server(host, port):
    """Function to run the Flask server in a thread."""
    try:
        smartgallery.run_app(host=host, port=port)
    except Exception as e:
        print(f"Failed to start server: {e}")
```

**After:**
```python
def start_server_and_load(window, host, port, url):
    """Startup function called by pywebview in separate thread."""
    try:
        # Start Flask server in daemon thread (Flask's run() blocks)
        server_thread = threading.Thread(
            target=smartgallery.run_app,
            args=(host, port),
            daemon=True
        )
        server_thread.start()
        
        # Wait for server to be ready (proper check)
        if wait_for_server(host, port, timeout=5):
            print(f"✓ Server ready at http://{host}:{port}")
            window.load_url(url)
        else:
            print("✗ Server failed to start")
            show_error_in_window(window, "Server startup timeout")
    except Exception as e:
        print(f"✗ Server startup error: {e}")
        show_error_in_window(window, str(e))
```

**New Helper Function:**
```python
def wait_for_server(host, port, timeout=5):
    """Poll server until ready or timeout."""
    import socket
    start_time = time.time()
    while time.time() - start_time < timeout:
        try:
            with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
                s.connect((host, port))
                return True
        except (ConnectionRefusedError, OSError):
            time.sleep(0.1)
    return False

def show_error_in_window(window, error_msg):
    """Display error message in existing window."""
    error_html = f"""
    <html>
    <head>
        <style>
            body {{ font-family: system-ui; padding: 40px; background: #f5f5f5; }}
            .error {{ background: white; padding: 30px; border-radius: 8px; 
                     border-left: 4px solid #d32f2f; }}
            h1 {{ color: #d32f2f; margin: 0 0 15px 0; }}
        </style>
    </head>
    <body>
        <div class="error">
            <h1>⚠️ Server Error</h1>
            <p><strong>Failed to start SmartGallery server:</strong></p>
            <pre>{error_msg}</pre>
            <p style="margin-top: 20px; color: #666;">Close this window to exit.</p>
        </div>
    </body>
    </html>
    """
    window.load_html(error_html)
```

---

### Phase 2: Modify Main Execution Flow

**Before:**
```python
# Start server in daemon thread
server_thread = threading.Thread(target=start_server, args=(host, port), daemon=True)
server_thread.start()

# Wait for server
time.sleep(1.5)

# Create window with URL
window = webview.create_window(window_title, url, width=1600, height=900, ...)

# Start GUI loop
webview.start(debug=False)
```

**After:**
```python
# Create window with loading screen initially
loading_html = """
<html>
<head>
    <style>
        body { 
            margin: 0; 
            display: flex; 
            justify-content: center; 
            align-items: center;
            height: 100vh; 
            font-family: system-ui;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
        }
        .loader {
            text-align: center;
        }
        .spinner {
            border: 4px solid rgba(255,255,255,0.3);
            border-top: 4px solid white;
            border-radius: 50%;
            width: 50px;
            height: 50px;
            animation: spin 1s linear infinite;
            margin: 0 auto 20px;
        }
        @keyframes spin {
            0% { transform: rotate(0deg); }
            100% { transform: rotate(360deg); }
        }
    </style>
</head>
<body>
    <div class="loader">
        <div class="spinner"></div>
        <h2>SmartGallery</h2>
        <p>Starting server...</p>
    </div>
</body>
</html>
"""

window = webview.create_window(
    window_title,
    html=loading_html,  # Initial loading screen
    width=1600,
    height=900,
    resizable=True,
    min_size=(900, 600)
)

window.events.closed += on_closed

# Start GUI loop with startup function
# pywebview will call start_server_and_load in separate thread
webview.start(start_server_and_load, (window, host, port, url))
```

---

### Phase 3: Update Error Handling Pattern

**Error Dialog Case** (config missing) stays separate since it needs immediate error display:

```python
if not initialize_app():
    # This case still uses separate error window (no server needed)
    error_window = webview.create_window(error_title, html=error_html, ...)
    webview.start()  # No startup function needed
    sys.exit(1)
```

**Server Error Case** (new) uses loading window approach:
- Window created with loading screen
- Startup function detects server failure
- Calls `window.load_html()` with error content
- User sees seamless transition from loading → error

---

## Implementation Checklist

### Step 1: Add Helper Functions
- [ ] `wait_for_server(host, port, timeout)` - Proper server ready check
- [ ] `show_error_in_window(window, error_msg)` - Display errors in existing window
- [ ] Create `loading_html` template string

### Step 2: Refactor start_server()
- [ ] Rename to `start_server_and_load(window, host, port, url)`
- [ ] Accept window as first parameter
- [ ] Start Flask in nested daemon thread (Flask blocks)
- [ ] Call `wait_for_server()` instead of `time.sleep()`
- [ ] Call `window.load_url(url)` when ready
- [ ] Handle errors via `show_error_in_window()`

### Step 3: Update Main Execution Flow
- [ ] Create window with `html=loading_html` instead of `url`
- [ ] Pass `start_server_and_load` to `webview.start()`
- [ ] Pass `(window, host, port, url)` as args tuple
- [ ] Remove manual threading code
- [ ] Remove `time.sleep(1.5)`

### Step 4: Simplify GUI Parameter
- [ ] Change `webview.start(gui='edgechromium' if ...)` to `webview.start()`
- [ ] Let pywebview auto-select best backend

### Step 5: Testing
- [ ] Test normal startup flow
- [ ] Test config missing (error dialog path)
- [ ] Test server port conflict
- [ ] Test server startup timeout
- [ ] Test keyboard shortcuts still work (Ctrl+A, etc.)
- [ ] Test all gallery features work as before

---

## Code Changes Summary

### Files Modified
1. **main.py** - Complete refactor of startup flow

### New Functions
```python
wait_for_server(host, port, timeout=5) -> bool
show_error_in_window(window, error_msg) -> None
start_server_and_load(window, host, port, url) -> None
```

### Removed Code
- Manual `threading.Thread()` creation for server
- `time.sleep(1.5)` hardcoded delay
- Explicit `gui='edgechromium'` parameter

### Modified Functions
- `start_server()` → `start_server_and_load()` (signature change)
- Main execution block (complete refactor)

---

## Risk Assessment

### Low Risk
- Helper functions (`wait_for_server`, `show_error_in_window`) - Pure additions
- Loading HTML template - Simple UI change
- Removing explicit `gui` parameter - pywebview handles fallback

### Medium Risk
- Startup function signature change - Requires careful arg passing
- Window lifecycle changes - Need thorough testing

### Mitigation
- Keep config error path unchanged (proven pattern)
- Test each phase incrementally
- Add logging for debugging
- Document rollback procedure (git revert)

---

## Rollback Plan

If issues arise:
```bash
git checkout main.py  # Revert to current working version
git stash             # Save refactored changes for later
```

Current implementation is functional and can remain if refactor causes issues.

---

## Expected Outcome

**Before (Manual):**
```
[Start] → [Thread] → [Sleep 1.5s] → [Create Window] → [GUI Loop]
```

**After (Idiomatic):**
```
[Start] → [Create Window w/Loading] → [GUI Loop + Startup Function]
    └→ [Server Ready] → [Load URL]
```

**Benefits Realized:**
- ✅ No hardcoded delays
- ✅ Proper server readiness check
- ✅ Better error handling
- ✅ More maintainable code
- ✅ Follows pywebview best practices
- ✅ Professional loading screen

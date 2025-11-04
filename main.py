# main.py

# nuitka-project: --mode=onefile
# nuitka-project: --onefile-tempdir-spec="{CACHE_DIR}/{PRODUCT}/{VERSION}"
# nuitka-project: --output-dir=dist
#
# nuitka-project: --include-data-dir=templates=templates
# nuitka-project: --include-data-dir=static=static
# nuitka-project: --include-data-file=bin/ffprobe.exe=ffprobe.exe
#
# nuitka-project: --windows-console-mode=disable
# nuitka-project: --windows-icon-from-ico=assets/icon.ico
# nuitka-project: --product-name="ComfyGallery"
# nuitka-project: --company-name="ComfyGallery Team"
# nuitka-project: --file-version="2.1.0"
# nuitka-project: --product-version="2.1.0"
# nuitka-project: --file-description="ComfyUI Media Gallery"


"""
ComfyGallery Desktop Application Entry Point

This wrapper script creates a native desktop application using PyWebView.
It uses proper thread lifecycle management to prevent memory leaks in PyInstaller builds.

CRITICAL FIXES (v2.1.0):
- Non-daemon server thread with shutdown event
- Proper cleanup with atexit handler
- Signal handling for Windows (Ctrl+C)
- Production WSGI server (waitress) instead of Flask dev server
- multiprocessing.freeze_support() to prevent infinite process spawning in PyInstaller
"""
import webview
import threading
import socket
import time
import sys
import os
import json
import appdirs
import logging
import atexit
import signal
import multiprocessing

# This will be the refactored, importable smartgallery module
import smartgallery

# ================================================================================
# GLOBAL SHUTDOWN MANAGEMENT
# ================================================================================
# These globals coordinate clean shutdown across threads
_server_shutdown_event = threading.Event()
_server_thread = None
_flask_app = None

# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(levelname)s - %(message)s'
)
logger = logging.getLogger(__name__)

# Configure pywebview settings before any webview operations
# Include all standard settings as documented in pywebview API
webview.settings = {
    'ALLOW_DOWNLOADS': True,  # For downloading files from gallery
    'ALLOW_FILE_URLS': True,  # Allow access to file:// URLs
    'DRAG_REGION_SELECTOR': 'pywebview-drag-region',  # CSS selector for drag regions
    'OPEN_EXTERNAL_LINKS_IN_BROWSER': True,  # Open external links in browser
    'OPEN_DEVTOOLS_IN_DEBUG': False,  # Don't auto-open devtools
    'IGNORE_SSL_ERRORS': False,  # Don't ignore SSL errors
    'REMOTE_DEBUGGING_PORT': None,  # No remote debugging
    'SHOW_DEFAULT_MENUS': True,  # Show default context menus
    'WEBVIEW2_RUNTIME_PATH': None,  # Use system WebView2 runtime
}

def find_free_port():
    """Finds and returns an available local port."""
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.bind(('', 0))
        return s.getsockname()[1]

def start_server_background(host, port, shutdown_event):
    """
    Run Flask server in background thread with shutdown support.
    
    This is a NON-DAEMON thread that can be properly cleaned up.
    Uses waitress production server for stability in PyInstaller builds.
    
    Args:
        host: Host to bind to
        port: Port to listen on
        shutdown_event: Threading event to signal shutdown
    """
    global _flask_app
    _flask_app = smartgallery.app
    
    try:
        logger.info(f"Server thread starting on {host}:{port}")
        smartgallery.run_app(host=host, port=port, debug=False)
    except Exception as e:
        logger.error(f"Server error: {e}")
    finally:
        shutdown_event.set()
        logger.info("Server thread exiting")

def wait_for_server(host, port, timeout=10):
    """
    Poll server until ready or timeout.
    
    Args:
        host: Server host
        port: Server port
        timeout: Maximum wait time in seconds
        
    Returns:
        bool: True if server is ready, False on timeout
    """
    start_time = time.time()
    while time.time() - start_time < timeout:
        try:
            with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
                s.settimeout(0.5)
                s.connect((host, port))
                return True
        except (ConnectionRefusedError, OSError):
            time.sleep(0.2)
    return False

def show_error_in_window(window, error_msg):
    """Display error message in existing window."""
    error_html = f"""
    <html>
    <head>
        <style>
            body {{ 
                font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
                padding: 40px;
                background: #f5f5f5;
                margin: 0;
            }}
            .error {{ 
                background: white;
                padding: 30px;
                border-radius: 8px;
                border-left: 4px solid #d32f2f;
                box-shadow: 0 2px 8px rgba(0,0,0,0.1);
                max-width: 600px;
                margin: 0 auto;
            }}
            h1 {{ 
                color: #d32f2f;
                margin: 0 0 15px 0;
                font-size: 24px;
            }}
            pre {{
                background: #f5f5f5;
                padding: 15px;
                border-radius: 4px;
                overflow-x: auto;
                font-size: 13px;
            }}
            p {{
                line-height: 1.6;
                color: #333;
            }}
        </style>
    </head>
    <body>
        <div class="error">
            <h1>Server Error</h1>
            <p><strong>Failed to start SmartGallery server:</strong></p>
            <pre>{error_msg}</pre>
            <p style="margin-top: 20px; color: #666;">
                Close this window to exit the application.
            </p>
        </div>
    </body>
    </html>
    """
    window.load_html(error_html)

def load_config():
    """Load configuration from standard locations with improved frozen-app support.

    Order of checks:
      1. %LOCALAPPDATA%\\SmartGallery\\config.json (preferred on Windows)
      2. appdirs.user_data_dir("SmartGallery")\\config.json (fallback)
      3. config.json next to the executable (when frozen) or in CWD (when running as script)
    """
    print("--- Attempting to load configuration ---")

    # --- Path 1: User AppData (Windows) ---
    config_path_user = None
    if sys.platform == 'win32':
        local_app_data = os.environ.get('LOCALAPPDATA')
        if local_app_data:
            user_data_path = os.path.join(local_app_data, 'SmartGallery')
            config_path_user = os.path.join(user_data_path, 'config.json')

    # Fallback to appdirs if needed (cross-platform)
    if not config_path_user:
        try:
            user_data_path = appdirs.user_data_dir('SmartGallery', appauthor=False)
            config_path_user = os.path.join(user_data_path, 'config.json')
        except Exception as e:
            print(f"Warning: appdirs failed to determine user data dir: {e}")
            config_path_user = None

    # --- Path 2: Executable or CWD ---
    if getattr(sys, 'frozen', False):
        # When frozen by PyInstaller, use the executable directory
        base_dir = os.path.dirname(sys.executable)
    else:
        # When running from source, prefer the current working directory
        base_dir = os.getcwd()

    config_path_local = os.path.join(base_dir, 'config.json')

    # Check candidate paths in order
    paths_to_check = [config_path_user, config_path_local]

    for p in paths_to_check:
        if not p:
            continue
        print(f"Checking for config at: {p}")
        if os.path.exists(p):
            try:
                with open(p, 'r', encoding='utf-8') as f:
                    config_data = json.load(f)
                print(f"[OK] Successfully loaded configuration from: {p}")
                return config_data
            except Exception as e:
                print(f"[ERROR] Found config at '{p}' but failed to parse it: {e}")
                # continue checking other locations
        else:
            print("  - Not found.")

    print("[ERROR] No valid config.json found in any checked location.")
    return None

def initialize_app():
    """Initialize the Flask application with configuration."""
    config_data = load_config()
    
    if not config_data:
        print("\nERROR: No config.json found!")
        USER_DATA_PATH = appdirs.user_data_dir("SmartGallery", appauthor=False)
        print(f"\nCreate a config.json file in one of these locations:")
        print(f"  1. {USER_DATA_PATH}\\config.json")
        print(f"  2. {os.getcwd()}\\config.json")
        print("\nExample config.json:")
        print('{')
        print('    "base_output_path": "C:/Path/To/Your/AI/Output",')
        print('    "base_input_path": "C:/Path/To/Your/AI/Input",')
        print('    "server_port": 8008')
        print('}')
        return False
    
    output_path = config_data.get('base_output_path')
    input_path = config_data.get('base_input_path')
    ffprobe_path = config_data.get('ffprobe_manual_path', '')
    
    if not output_path or not input_path:
        print("\nERROR: config.json is missing required paths!")
        print("Required fields: base_output_path, base_input_path")
        return False
    
    if not os.path.isdir(output_path):
        print(f"\nERROR: Output path does not exist: {output_path}")
        return False
    
    if not os.path.isdir(input_path):
        print(f"\nERROR: Input path does not exist: {input_path}")
        return False
    
    # Populate Flask app config
    smartgallery.app.config['BASE_OUTPUT_PATH'] = output_path
    smartgallery.app.config['BASE_INPUT_PATH'] = input_path
    smartgallery.app.config['FFPROBE_MANUAL_PATH'] = ffprobe_path
    
    # Apply additional config options if present
    if 'thumbnail_quality' in config_data:
        smartgallery.app.config['THUMBNAIL_QUALITY'] = config_data['thumbnail_quality']
    if 'enable_upload' in config_data:
        smartgallery.app.config['ENABLE_UPLOAD'] = config_data['enable_upload']
    if 'max_upload_size_mb' in config_data:
        smartgallery.app.config['MAX_UPLOAD_SIZE_MB'] = config_data['max_upload_size_mb']
    
    # Update ALL_MEDIA_EXTENSIONS after potential config updates
    smartgallery.app.config['ALL_MEDIA_EXTENSIONS'] = (
        smartgallery.app.config['VIDEO_EXTENSIONS'] + 
        smartgallery.app.config['IMAGE_EXTENSIONS'] + 
        smartgallery.app.config['ANIMATED_IMAGE_EXTENSIONS'] + 
        smartgallery.app.config['AUDIO_EXTENSIONS']
    )
    
    # Initialize derived paths and database
    smartgallery.initialize_gallery(smartgallery.app)
    
    print("\n" + "="*60)
    print("SmartGallery - Desktop Application")
    print("="*60)
    print(f"Output Path: {output_path}")
    print(f"Input Path:  {input_path}")
    print("="*60 + "\n")
    
    return True

def on_startup(window):
    """
    PyWebView startup callback - runs in dedicated thread.
    
    This function is called by PyWebView in a separate thread when the window is created.
    It starts the Flask server and loads the URL once ready.
    
    Args:
        window: PyWebView window instance
    """
    host = '127.0.0.1'
    port = find_free_port()
    url = f"http://{host}:{port}/galleryout/"
    
    # Clear shutdown event
    _server_shutdown_event.clear()
    
    # Start server in NON-DAEMON background thread
    _server_thread = threading.Thread(
        target=start_server_background,
        args=(host, port, _server_shutdown_event),
        daemon=False,  # NON-DAEMON for clean shutdown
        name="FlaskServerThread"
    )
    _server_thread.start()
    logger.info("Server thread started")
    
    # Wait for server readiness with timeout
    if wait_for_server(host, port, timeout=10):
        logger.info(f"✓ Server ready at {url}")
        window.load_url(url)
    else:
        logger.error("✗ Server failed to start within timeout")
        error_html = """
        <html>
        <head>
            <style>
                body { 
                    font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
                    padding: 40px;
                    background: #f5f5f5;
                    margin: 0;
                    display: flex;
                    justify-content: center;
                    align-items: center;
                    height: 100vh;
                }
                .error { 
                    background: white;
                    padding: 30px;
                    border-radius: 8px;
                    border-left: 4px solid #d32f2f;
                    box-shadow: 0 2px 8px rgba(0,0,0,0.1);
                    max-width: 600px;
                }
                h1 { 
                    color: #d32f2f;
                    margin: 0 0 15px 0;
                    font-size: 24px;
                }
            </style>
        </head>
        <body>
            <div class="error">
                <h1>⚠️ Server Startup Failed</h1>
                <p>The server failed to start within 10 seconds.</p>
                <p>Please close this window and try again.</p>
            </div>
        </body>
        </html>
        """
        window.load_html(error_html)

def cleanup_and_exit():
    """
    Explicit cleanup on exit.
    
    Registered with atexit to ensure cleanup happens even on abnormal termination.
    Signals server shutdown and waits for thread to finish.
    """
    logger.info("Application cleanup starting...")
    
    # Signal server shutdown
    _server_shutdown_event.set()
    
    # Wait for server thread to finish (with timeout)
    if _server_thread and _server_thread.is_alive():
        logger.info("Waiting for server thread to finish...")
        _server_thread.join(timeout=2.0)
        if _server_thread.is_alive():
            logger.warning("Server thread did not finish in time")
        else:
            logger.info("Server thread finished cleanly")
    
    logger.info("Application cleanup complete")

# Register cleanup handler
atexit.register(cleanup_and_exit)

# Add Windows signal handler for Ctrl+C
if sys.platform == 'win32':
    def signal_handler(signum, frame):
        logger.info("Received interrupt signal")
        cleanup_and_exit()
        sys.exit(0)
    
    signal.signal(signal.SIGINT, signal_handler)
    signal.signal(signal.SIGTERM, signal_handler)

if __name__ == '__main__':
    # CRITICAL: Prevent infinite process spawning in PyInstaller builds
    # This MUST be the first line - prevents module-level code from re-executing in worker processes
    multiprocessing.freeze_support()
    
    # 0. Initialize the application with config
    if not initialize_app():
        # Show error dialog instead of trying to read from stdin
        error_title = "SmartGallery - Configuration Error"
        error_msg = """SmartGallery cannot start without valid configuration.

Please create a config.json file in one of these locations:
  • %LOCALAPPDATA%\\SmartGallery\\config.json
  • Current directory: config.json

Example config.json:
{
    "base_output_path": "C:/Path/To/Your/AI/Output",
    "base_input_path": "C:/Path/To/Your/AI/Input",
    "server_port": 8008
}

For more information, see the documentation."""
        
        # Create a simple error window
        try:
            window = webview.create_window(error_title, html=f"""
                <html>
                <head>
                    <style>
                        body {{ 
                            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; 
                            padding: 30px; 
                            background: #f5f5f5;
                            margin: 0;
                        }}
                        .container {{
                            background: white;
                            padding: 30px;
                            border-radius: 8px;
                            box-shadow: 0 2px 8px rgba(0,0,0,0.1);
                            max-width: 600px;
                            margin: 0 auto;
                        }}
                        h1 {{ 
                            color: #d32f2f; 
                            margin-top: 0;
                            font-size: 24px;
                        }}
                        pre {{ 
                            background: #f5f5f5; 
                            padding: 15px; 
                            border-radius: 4px;
                            overflow-x: auto;
                            font-size: 13px;
                        }}
                        p {{ 
                            line-height: 1.6; 
                            color: #333;
                        }}
                        .path {{ 
                            color: #1976d2; 
                            font-family: 'Courier New', monospace;
                            background: #e3f2fd;
                            padding: 2px 6px;
                            border-radius: 3px;
                        }}
                    </style>
                </head>
                <body>
                    <div class="container">
                        <h1>⚠️ Configuration Error</h1>
                        <p><strong>SmartGallery cannot start without valid configuration.</strong></p>
                        <p>Please create a <span class="path">config.json</span> file in one of these locations:</p>
                        <ul>
                            <li><span class="path">%LOCALAPPDATA%\\SmartGallery\\config.json</span></li>
                            <li><span class="path">config.json</span> (in the same directory as the executable)</li>
                        </ul>
                        <p><strong>Example config.json:</strong></p>
                        <pre>{{
    "base_output_path": "C:/Path/To/Your/AI/Output",
    "base_input_path": "C:/Path/To/Your/AI/Input",
    "server_port": 8008
}}</pre>
                        <p style="margin-top: 20px; color: #666; font-size: 14px;">
                            Close this window to exit the application.
                        </p>
                    </div>
                </body>
                </html>
            """, width=700, height=600, resizable=False)
            webview.start()
        except Exception as e:
            logging.error(f"Failed to create error window: {e}")
            pass  # If webview fails, just exit silently
        
        sys.exit(1)
    
    # 1. Create loading HTML template
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
                font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
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
            h2 {
                margin: 0 0 10px 0;
                font-size: 28px;
                font-weight: 600;
            }
            p {
                margin: 0;
                font-size: 16px;
                opacity: 0.9;
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

    # 2. Create window with loading screen
    logger.info("Creating PyWebView window...")
    window = webview.create_window(
        title='SmartGallery',
        html=loading_html,
        width=1600,
        height=900,
        resizable=True,
        min_size=(900, 600)
    )

    def on_closed():
        logger.info("PyWebView window closed by user")
        cleanup_and_exit()

    window.events.closed += on_closed

    # 3. Start the GUI loop with startup function
    # PyWebView will call on_startup in a separate thread automatically
    logger.info("Starting PyWebView...")
    webview.start(on_startup, window)

    # 4. When window closes, this point is reached
    logger.info("Main application thread exiting")
    cleanup_and_exit()

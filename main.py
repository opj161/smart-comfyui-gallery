# main.py
"""
SmartGallery Desktop Application Entry Point

This wrapper script creates a native desktop application using PyWebView.
It starts the Flask server in a background thread and displays the UI in a native window.
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

# This will be the refactored, importable smartgallery module
import smartgallery

# Configure pywebview settings
webview.settings = {
    'ALLOW_DOWNLOADS': True,  # For downloading files from gallery
}

def find_free_port():
    """Finds and returns an available local port."""
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.bind(('', 0))
        return s.getsockname()[1]

def load_config():
    """Load configuration from config.json in user data directory or current directory."""
    USER_DATA_PATH = appdirs.user_data_dir("SmartGallery", appauthor=False)
    config_path_user = os.path.join(USER_DATA_PATH, "config.json")
    config_path_local = "config.json"
    
    config_to_load = None
    if os.path.exists(config_path_user):
        config_to_load = config_path_user
    elif os.path.exists(config_path_local):
        config_to_load = config_path_local
    
    if config_to_load:
        try:
            with open(config_to_load, 'r', encoding='utf-8') as f:
                config_data = json.load(f)
            print(f"✓ Loaded configuration from: {config_to_load}")
            return config_data
        except Exception as e:
            print(f"Failed to load config file '{config_to_load}': {e}")
            return None
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

def start_server(host, port):
    """Function to run the Flask server in a thread."""
    try:
        smartgallery.run_app(host=host, port=port)
    except Exception as e:
        print(f"Failed to start server: {e}")

if __name__ == '__main__':
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
            webview.start(gui='edgechromium' if sys.platform == 'win32' else None)
        except:
            pass  # If webview fails, just exit silently
        
        sys.exit(1)
    
    # 1. Determine a free port for the Flask server
    port = find_free_port()
    host = '127.0.0.1'
    url = f"http://{host}:{port}/galleryout/"
    window_title = "SmartGallery"

    # 2. Start the Flask server in a background daemon thread
    # A daemon thread will automatically terminate when the main program exits.
    server_thread = threading.Thread(target=start_server, args=(host, port), daemon=True)
    server_thread.start()

    # Give the server a moment to start up.
    time.sleep(1.5)

    # 3. Create and show the PyWebView window
    # The URL points to our local server.
    window = webview.create_window(
        window_title,
        url,
        width=1600,
        height=900,
        resizable=True,
        min_size=(900, 600)
    )

    def on_closed():
        print("PyWebView window is closed. Application will now exit.")
        # Since the server thread is a daemon, it will be automatically
        # terminated when this main thread exits. No explicit shutdown needed.

    window.events.closed += on_closed

    # 4. Start the GUI loop (this is a blocking call)
    # The 'edgechromium' GUI uses Microsoft Edge WebView2 on Windows (recommended).
    # On other OSes, PyWebView will choose the best available.
    webview.start(debug=False)

    print("Main application thread is exiting.")

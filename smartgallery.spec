# smartgallery.spec
"""
PyInstaller specification file for SmartGallery Desktop Application

This file instructs PyInstaller on how to build the standalone executable.
It defines the entry point, data files, binaries, and other settings.

Note: Uses Microsoft Edge WebView2 on Windows (edgechromium backend).
Users must have Edge WebView2 runtime installed (pre-installed on Windows 10/11).

For more details, see the PyInstaller documentation: spec-files.rst
"""

import os
from PyInstaller.utils.hooks import collect_data_files, collect_submodules

# Create an Analysis object. This is the core of the spec file.
# It analyzes 'main.py' to find all dependencies.
a = Analysis(
    ['main.py'],
    pathex=[],
    binaries=[],
    datas=[],
    hiddenimports=[
        # Add any modules that PyInstaller might miss.
        'pkg_resources.extern',
        'PIL._tkinter_finder',
        'engineio.async_drivers.threading',
    ],
    hookspath=[],
    runtime_hooks=[],
    excludes=[],
    win_no_prefer_redirects=False,
    win_private_assemblies=False,
    cipher=None,
    noarchive=False,
)

# --- 1. Add Bundled Data Files ---
# This section ensures that your application's assets are included.
# The format is a list of tuples: (dest_name_in_bundle, source_file, 'DATA')
a.datas += [
    ('templates\\index.html', os.path.join('.', 'templates', 'index.html'), 'DATA'),
    ('static\\galleryout\\favicon.ico', os.path.join('.', 'static', 'galleryout', 'favicon.ico'), 'DATA'),
]

# --- 2. Add External Binaries ---
# This is crucial for bundling ffprobe.
# Place ffprobe.exe (for Windows) in a 'bin' directory next to this spec file.
# The format is (destination_name, source_path, 'BINARY')
ffprobe_binary_name = 'ffprobe.exe' if os.name == 'nt' else 'ffprobe'
ffprobe_path = os.path.join('bin', ffprobe_binary_name)
if os.path.exists(ffprobe_path):
    a.binaries += [(ffprobe_binary_name, ffprobe_path, 'BINARY')]
    print(f"✓ Including ffprobe binary: {ffprobe_path}")
else:
    print(f"⚠ WARNING: ffprobe binary not found at {ffprobe_path}")
    print(f"  Video metadata extraction will not work in the bundled application.")


# PYZ is a compressed archive of all the Python modules.
pyz = PYZ(a.pure, a.zipped_data, cipher=None)

# EXE creates the final executable.
exe = EXE(
    pyz,
    a.scripts,
    a.binaries,
    a.zipfiles,
    a.datas,
    [],
    name='SmartGallery',
    debug=False,
    bootloader_ignore_signals=False,
    strip=False,
    upx=True,  # Use UPX to compress binaries and reduce final size
    upx_exclude=[],
    runtime_tmpdir=None,
    console=False,  # This is crucial for a GUI app. No console window will open.
    disable_windowed_traceback=False,
    target_arch=None,
    codesign_identity=None,
    entitlements_file=None,
    icon='assets/icon.ico',  # Specify the path to your application icon
)

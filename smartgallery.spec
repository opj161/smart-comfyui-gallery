# -*- mode: python ; coding: utf-8 -*-

import os

a = Analysis(
    ['main.py'],
    pathex=[],
    binaries=[],
    datas=[('templates', 'templates'), ('static', 'static')],
    hiddenimports=['flask', 'pywebview', 'waitress', 'PIL'],
    hookspath=[],
    hooksconfig={},
    runtime_hooks=[],
    excludes=[],
    noarchive=False,
    optimize=0,
)


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

pyz = PYZ(a.pure)

exe = EXE(
    pyz,
    a.scripts,
    a.binaries,
    a.datas,
    [],
    name='SmartGallery',
    debug=False,
    bootloader_ignore_signals=False,
    strip=False,
    upx=True,
    upx_exclude=[],
    runtime_tmpdir=None,
    console=False,
    disable_windowed_traceback=False,
    argv_emulation=False,
    target_arch=None,
    codesign_identity=None,
    entitlements_file=None,
    icon='assets/icon.ico',  # Specify the path to your application icon

)

# SmartGallery Tauri Setup Guide

## Prerequisites Installation

The error `failed to get cargo metadata: program not found` means Rust is not installed or not in your PATH.

### Required Tools

1. **Rust & Cargo** (Required for Tauri backend)
2. **Node.js & npm** (v18 or later)
3. **System Dependencies** (platform-specific)

---

## Installation Steps

### 1. Install Rust

**Windows:**
```powershell
# Download and run rustup-init.exe from:
# https://rustup.rs/

# Or via winget:
winget install Rustlang.Rustup

# After installation, restart your terminal
```

**Linux/macOS:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Follow prompts, then reload shell
source $HOME/.cargo/env
```

**Verify Installation:**
```bash
cargo --version
# Should output: cargo 1.7x.x or later
```

### 2. Install System Dependencies

**Windows:**
- WebView2 (usually pre-installed on Windows 11)
- Visual Studio Build Tools or Microsoft C++ Build Tools
  ```powershell
  # Download from:
  # https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022
  ```

**Linux (Ubuntu/Debian):**
```bash
sudo apt update
sudo apt install -y \
    libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
```

**macOS:**
```bash
# Xcode Command Line Tools
xcode-select --install
```

### 3. Install Node.js Dependencies

```bash
cd tauri-sveltekit-main
npm install
```

### 4. Verify Setup

```bash
# Check Rust
cargo --version
rustc --version

# Check Node
node --version
npm --version

# Check Tauri CLI (will be installed by npm)
npx tauri --version
```

---

## Running the Application

### Development Mode

```bash
cd tauri-sveltekit-main

# Run development server with hot reload
npm run dev

# This will:
# 1. Build the SvelteKit frontend
# 2. Compile the Rust backend
# 3. Launch the Tauri window
```

**First Run Notes:**
- First compilation takes 5-10 minutes (downloads and compiles Rust dependencies)
- Subsequent runs are much faster (30-60 seconds)
- You'll see a native window open with the SmartGallery UI

### Frontend-Only Development (No Rust)

If you want to work on the frontend without Tauri:

```bash
npm run sveltekit:dev
# Opens in browser at http://localhost:5173
# Note: Tauri commands will fail, but UI can be developed
```

### Building for Production

```bash
# Build release version (optimized, smaller binary)
npm run tauri build

# Output will be in:
# - Windows: src-tauri/target/release/bundle/msi/
# - Linux: src-tauri/target/release/bundle/appimage/ or .deb
# - macOS: src-tauri/target/release/bundle/dmg/
```

---

## Troubleshooting

### Error: "program not found" (cargo)

**Solution:**
1. Install Rust from https://rustup.rs/
2. Restart your terminal/IDE
3. Verify with `cargo --version`

### Error: "webkit2gtk" not found (Linux)

**Solution:**
```bash
sudo apt install libwebkit2gtk-4.0-dev
```

### Error: npm ERR! Missing script: "tauri"

**Solution:**
```bash
npm install
# Ensures @tauri-apps/cli is installed
```

### Error: Compilation takes forever

**Explanation:**
- First build downloads and compiles ~200 Rust dependencies
- This is normal and only happens once
- Subsequent builds use cached dependencies

**Progress Indication:**
You'll see output like:
```
   Compiling serde v1.0.x
   Compiling tokio v1.x.x
   Compiling tauri v2.x.x
```

### Error: "Failed to bundle project"

**Solutions:**
1. Check disk space (needs ~5GB for Rust toolchain + dependencies)
2. Try clean build:
   ```bash
   cd src-tauri
   cargo clean
   cd ..
   npm run dev
   ```

### Error: Port already in use

**Solution:**
```bash
# Kill process on port 1420 (Tauri default)
# Windows:
netstat -ano | findstr :1420
taskkill /PID <pid> /F

# Linux/macOS:
lsof -ti:1420 | xargs kill -9
```

---

## Project Structure

```
tauri-sveltekit-main/
├── src/                    # SvelteKit frontend
│   ├── routes/            # Pages (+page.svelte)
│   └── lib/               # Components, stores, API
├── src-tauri/             # Rust backend
│   ├── src/
│   │   ├── lib.rs        # Entry point
│   │   ├── commands.rs   # Tauri commands
│   │   ├── database.rs   # SQLite operations
│   │   ├── parser.rs     # Workflow parser
│   │   ├── scanner.rs    # File scanning
│   │   └── thumbnails.rs # Thumbnail generation
│   ├── Cargo.toml        # Rust dependencies
│   └── tauri.conf.json   # Tauri configuration
├── package.json           # Node dependencies & scripts
└── vite.config.ts        # Vite build config
```

---

## Development Workflow

### 1. Start Development Server
```bash
npm run dev
```

### 2. Make Code Changes
- **Frontend**: Edit files in `src/` → hot reload in Tauri window
- **Rust**: Edit files in `src-tauri/src/` → requires restart (Ctrl+C, `npm run dev`)

### 3. Test Your Changes
- **Manual**: Use the UI in the Tauri window
- **Automated**: Run test scripts (see PHASE_5_TESTING_GUIDE.md)

### 4. Build for Release
```bash
npm run tauri build
```

---

## Available npm Scripts

```bash
# Development
npm run dev              # Full Tauri development mode
npm run sveltekit:dev    # Frontend-only (browser)

# Building
npm run sveltekit:build  # Build frontend to /build
npm run tauri build      # Build complete application

# Tauri commands
npm run tauri dev        # Same as npm run dev
npm run tauri build      # Build release version

# Linting & Type Checking
npm run check            # TypeScript type checking
npm run lint             # ESLint
```

---

## Next Steps After Setup

Once you have the app running:

1. **Initialize Gallery**
   - Click "Initialize Gallery" button
   - Enter path to ComfyUI output directory
   - Example: `C:\ComfyUI\output` (Windows) or `/home/user/ComfyUI/output` (Linux)

2. **Sync Files**
   - Click "Sync Files" button
   - Watch progress bar as files are processed
   - Should complete in < 30 seconds for 1,000 files

3. **Browse Gallery**
   - Files appear in grid
   - Click any file to open lightbox
   - Use keyboard shortcuts: ←/→ (navigate), ESC (close), i (metadata)

4. **Test Features**
   - Search bar: Type keywords
   - Filters button: Model, sampler, dimensions
   - Select files: Click checkboxes, Shift+click for range
   - Batch actions: Favorite/delete multiple files

---

## Performance Expectations

**First Build:**
- 5-10 minutes (one-time Rust dependency compilation)

**Subsequent Builds:**
- 30-60 seconds (incremental compilation)

**Runtime Performance:**
- File sync: 30-50 files/second
- Thumbnail generation: 10-20 images/second
- UI responsiveness: 60 FPS scrolling

---

## Getting Help

**Build Issues:**
- Check Tauri docs: https://tauri.app/v1/guides/getting-started/prerequisites
- Rust installation: https://rustup.rs/
- Node/npm issues: https://nodejs.org/

**Feature Testing:**
- See PHASE_5_TESTING_GUIDE.md for comprehensive test procedures
- Report issues using test results template in guide

**Architecture Questions:**
- See IMPLEMENTATION_PLAN.md for project structure
- Review code comments in src-tauri/src/ files

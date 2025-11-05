# SmartGallery - Tauri/Rust/SvelteKit Edition

This is the modern refactor of SmartGallery using Tauri 2.0, Rust backend, and SvelteKit frontend.

## Architecture

### Backend (Rust)
- **Tauri 2.0**: Native desktop application framework
- **SQLx**: Async database operations with compile-time SQL verification
- **Rayon**: Data parallelism for file scanning
- **Walkdir**: Efficient directory traversal
- **Image crate**: Thumbnail generation
- **Serde**: Serialization/deserialization for IPC

### Frontend (SvelteKit)
- **Svelte 5**: Modern reactive framework with runes
- **SvelteKit**: Application framework with adapter-static
- **TypeScript**: Type-safe development
- **Vite**: Fast build tooling

### Communication
- **Tauri IPC**: Type-safe commands via `invoke()`
- **Tauri Events**: Real-time progress updates

## Project Structure

```
smartgallery-tauri/
├── src/                      # Frontend source
│   ├── lib/
│   │   ├── components/      # Svelte components
│   │   ├── stores/          # State management
│   │   └── types.ts         # TypeScript types
│   └── routes/              # SvelteKit routes
│       └── +page.svelte     # Main gallery page
├── src-tauri/               # Backend source
│   ├── src/
│   │   ├── models.rs        # Core data structures
│   │   ├── database.rs      # Database operations
│   │   ├── scanner.rs       # File system scanner
│   │   ├── parser.rs        # Workflow parser
│   │   ├── thumbnails.rs    # Thumbnail generator
│   │   └── lib.rs           # Main entry point
│   ├── Cargo.toml           # Rust dependencies
│   └── tauri.conf.json      # Tauri configuration
├── package.json             # Node dependencies
└── vite.config.js           # Build configuration
```

## Development Setup

### Prerequisites

1. **Node.js 20+** and npm
2. **Rust 1.70+** via rustup
3. **System dependencies** (Linux):
   ```bash
   # Debian/Ubuntu
   sudo apt update
   sudo apt install libwebkit2gtk-4.1-dev \
     build-essential \
     curl \
     wget \
     file \
     libxdo-dev \
     libssl-dev \
     libayatana-appindicator3-dev \
     librsvg2-dev
   
   # Fedora
   sudo dnf install webkit2gtk4.1-devel \
     openssl-devel \
     curl \
     wget \
     file \
     libappindicator-gtk3-devel \
     librsvg2-devel
   ```
4. **System dependencies** (macOS):
   ```bash
   xcode-select --install
   ```
5. **System dependencies** (Windows):
   - Install [Microsoft Visual Studio C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
   - Install [WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/)

### Installation

1. Install npm dependencies:
   ```bash
   npm install
   ```

2. The Rust dependencies will be automatically installed when running Tauri commands.

### Development

Run the development server:
```bash
npm run tauri:dev
```

This will:
1. Start the Vite dev server for hot-reloading frontend
2. Compile and run the Rust backend
3. Open the application window

### Type Checking

Check TypeScript types:
```bash
npm run check
```

Watch mode:
```bash
npm run check:watch
```

### Building

Build the application for production:
```bash
npm run tauri:build
```

This creates native installers in `src-tauri/target/release/bundle/`.

## Migration Status

### Phase 1: Foundation and Scaffolding ✅
- [x] Tauri + SvelteKit project initialized
- [x] Core Rust data structures (models.rs)
- [x] TypeScript type definitions
- [x] Basic IPC bridge verified

### Phase 2: Backend Migration (In Progress)
- [ ] Database module (database.rs)
- [ ] File scanner (scanner.rs)
- [ ] Workflow parser (parser.rs)
- [ ] Thumbnail generator (thumbnails.rs)

### Phase 3: Frontend Migration (Pending)
- [ ] Component structure
- [ ] State management
- [ ] Replace Flask API calls with Tauri commands

### Phase 4: Integration & Polish (Pending)
- [ ] End-to-end testing
- [ ] Error handling
- [ ] Performance optimization

### Phase 5: Build & Distribution (Pending)
- [ ] Configure tauri.conf.json
- [ ] Set up CI/CD
- [ ] Create installers

## Key Differences from Python Version

### Performance
- **Rust backend**: Compiled code, ~10-50x faster than Python
- **True parallelism**: Rayon uses all CPU cores without GIL limitations
- **Memory efficiency**: No Python interpreter overhead

### Security
- **Tauri capabilities**: Fine-grained permission system
- **No remote code execution**: Compiled binary, no eval()
- **Sandboxed frontend**: Web view with controlled backend access

### Distribution
- **Single executable**: No Python runtime required
- **Small bundle**: ~30-50MB vs ~100MB+ for PyInstaller
- **Auto-updates**: Built-in updater support

### Developer Experience
- **Type safety**: Rust + TypeScript end-to-end
- **Hot reloading**: Instant frontend updates
- **Better tooling**: cargo + npm ecosystem

## Contributing

This is an in-progress migration. The goal is to maintain 100% feature parity with the Python version while gaining the benefits of the modern stack.

## License

MIT License - Same as original SmartGallery

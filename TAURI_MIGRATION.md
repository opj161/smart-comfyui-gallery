# SmartGallery - Tauri/Rust/SvelteKit Refactor

## Phase 1: Foundation and Scaffolding ✅

This directory contains the new Tauri/Rust/SvelteKit implementation of SmartGallery, which is a complete architectural refactor of the original Python/Flask/PyWebView application.

### Current Status

**Phase 1 Complete** - The foundation has been established:

- ✅ Tauri + SvelteKit template extracted and configured
- ✅ Core data structures defined in Rust (`src-tauri/src/models.rs`)
- ✅ TypeScript types created (`src/lib/types.ts`)
- ✅ Rust-Frontend IPC bridge established (test `greet` command)
- ✅ Build system verified (both Rust and SvelteKit build successfully)

### Architecture Overview

```
smart-comfyui-gallery/
├── src/                      # SvelteKit frontend
│   ├── lib/
│   │   └── types.ts         # TypeScript type definitions
│   └── routes/
│       └── +page.svelte     # Main page with IPC test
│
├── src-tauri/                # Rust backend
│   ├── src/
│   │   ├── models.rs        # Core data structures
│   │   ├── lib.rs           # Main library with Tauri commands
│   │   └── main.rs          # Entry point
│   └── Cargo.toml           # Rust dependencies
│
├── static/                   # Static assets
├── package.json              # Node.js dependencies
└── tauri.conf.json          # Tauri configuration
```

### Key Technologies

- **Frontend**: SvelteKit v2, Vite v6, TypeScript
- **Backend**: Rust, Tauri v2
- **UI**: TailwindCSS v3
- **IPC**: Tauri's invoke/command system

### Development Commands

```bash
# Install dependencies
npm install

# Run in development mode (hot-reload enabled)
npm run dev

# Build SvelteKit frontend only
npm run sveltekit:build

# Build complete application (creates distributable)
npm run build

# Type checking
npm run check

# Linting
npm run lint

# Format code
npm run format
```

### Project Structure

#### Rust Backend (`src-tauri/`)

- **models.rs**: Core data structures that mirror the Python implementation:
  - `FileEntry`: Gallery file metadata
  - `WorkflowMetadata`: ComfyUI workflow data
  - `FilterRequest`: Query parameters
  - `AppConfig`: Application configuration
  
- **lib.rs**: Tauri commands (IPC handlers)
  - Currently: `greet` (test command)
  - Future: Database, filesystem, workflow parsing commands

#### SvelteKit Frontend (`src/`)

- **types.ts**: TypeScript interfaces matching Rust structs
- **+page.svelte**: Main UI (currently a test page)
- Future: Component library, state management, API integration

### Migration Roadmap

#### ✅ Phase 1: Foundation (Complete)
- Template setup
- Data structures
- IPC bridge
- Build verification

#### 🔄 Phase 2: Backend Migration (Next)
- Database layer (SQLite with sqlx)
- File system scanner
- Workflow parser
- Thumbnail generation

#### 📋 Phase 3: Frontend Migration
- Component breakdown
- State management (Svelte 5 Runes)
- API integration
- Real-time sync UI

#### 📋 Phase 4: Integration & Polish
- End-to-end testing
- Error handling
- Deep linking
- Performance optimization

#### 📋 Phase 5: Build & Distribution
- Cross-platform builds
- Security hardening
- Signed installers

### Testing the Current Implementation

1. Start the development server:
   ```bash
   npm run dev
   ```

2. The application will open in a native window

3. Test the IPC bridge:
   - Enter your name in the input field
   - Click "Greet"
   - You should see a message from Rust!

### Key Benefits of New Architecture

1. **Performance**: Rust backend is significantly faster than Python
2. **Memory Safety**: Rust's ownership system prevents memory leaks
3. **Type Safety**: Full type safety from Rust to TypeScript
4. **Modern UI**: SvelteKit with Runes for reactive state management
5. **Better Distribution**: Tauri creates smaller, more secure installers
6. **Cross-Platform**: Single codebase for Windows, macOS, and Linux

### Compatibility Notes

- The existing Python implementation (`smartgallery.py`, `main.py`) remains intact
- Both versions can coexist during the transition
- The new version will maintain full compatibility with existing workflow files
- Database schema will be ported to maintain data continuity

### Contributing

This is an active migration project. The original Python implementation can be found in:
- `smartgallery.py` - Flask backend
- `main.py` - PyWebView wrapper
- `templates/index.html` - Alpine.js frontend

### Resources

- [Tauri Documentation](https://tauri.app/)
- [SvelteKit Documentation](https://kit.svelte.dev/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Migration Plan](../tauri-sveltekit-main.xml) - Contains the starter template

---

**Status**: Phase 1 Complete ✅  
**Next**: Implement database layer and file scanner in Rust

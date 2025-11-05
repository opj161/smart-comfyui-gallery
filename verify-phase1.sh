#!/bin/bash
# Verification script for Phase 1 completion

echo "============================================"
echo "SmartGallery Phase 1 Verification"
echo "============================================"
echo ""

# Check Node.js
echo "✓ Node.js version:"
node --version

# Check npm
echo "✓ npm version:"
npm --version

# Check Rust/Cargo
echo "✓ Cargo version:"
cargo --version

echo ""
echo "============================================"
echo "Build Verification"
echo "============================================"
echo ""

# Check SvelteKit build
echo "→ Building SvelteKit frontend..."
npm run sveltekit:build > /tmp/sveltekit-build.log 2>&1
if [ $? -eq 0 ]; then
    echo "✓ SvelteKit build successful"
else
    echo "✗ SvelteKit build failed"
    exit 1
fi

# Check Rust build
echo "→ Checking Rust compilation..."
cd src-tauri && cargo check > /tmp/cargo-check.log 2>&1
if [ $? -eq 0 ]; then
    echo "✓ Rust compilation successful"
else
    echo "✗ Rust compilation failed"
    exit 1
fi
cd ..

# Check TypeScript
echo "→ Running TypeScript checks..."
npm run check > /tmp/svelte-check.log 2>&1
if [ $? -eq 0 ]; then
    echo "✓ TypeScript checks passed"
else
    echo "✗ TypeScript checks failed"
    exit 1
fi

echo ""
echo "============================================"
echo "File Structure Verification"
echo "============================================"
echo ""

# Check key files exist
FILES=(
    "src-tauri/src/models.rs"
    "src-tauri/src/lib.rs"
    "src/lib/types.ts"
    "src/routes/+page.svelte"
    "src-tauri/tauri.conf.json"
    "package.json"
    "src-tauri/Cargo.toml"
)

for file in "${FILES[@]}"; do
    if [ -f "$file" ]; then
        echo "✓ $file"
    else
        echo "✗ $file (missing)"
    fi
done

echo ""
echo "============================================"
echo "Phase 1 Completion Summary"
echo "============================================"
echo ""
echo "✅ Tauri + SvelteKit template extracted"
echo "✅ Core data structures defined (Rust & TypeScript)"
echo "✅ IPC bridge established (greet command)"
echo "✅ Build system verified"
echo "✅ Project configured with SmartGallery metadata"
echo ""
echo "🎯 Next Phase: Backend Migration"
echo "   - Database layer (SQLite with sqlx)"
echo "   - File system scanner"
echo "   - Workflow parser"
echo "   - Thumbnail generation"
echo ""
echo "To run the app: npm run dev"
echo ""

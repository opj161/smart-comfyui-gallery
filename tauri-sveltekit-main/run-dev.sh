#!/bin/bash

# SmartGallery Development Launcher
# This script makes it easy to test the application

echo "🎨 SmartGallery - Development Mode"
echo "=================================="
echo ""

# Check if we're in the right directory
if [ ! -f "package.json" ]; then
    echo "❌ Error: Please run this script from the tauri-sveltekit-main directory"
    exit 1
fi

# Check if node_modules exists
if [ ! -d "node_modules" ]; then
    echo "📦 Installing dependencies..."
    npm install
    echo ""
fi

echo "✅ All checks passed!"
echo ""
echo "Starting SmartGallery in development mode..."
echo "The application window will open in a few seconds."
echo ""
echo "📝 Quick Tips:"
echo "  - The app will open in a new window"
echo "  - Initialize with your ComfyUI output directory"
echo "  - Click 'Sync' to scan for files"
echo "  - Press Ctrl+C here to stop the dev server"
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Start the development server
npm run dev

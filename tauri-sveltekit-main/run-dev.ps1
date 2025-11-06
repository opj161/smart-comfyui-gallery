# SmartGallery Development Launcher
# This script makes it easy to test the application

Write-Host "🎨 SmartGallery - Development Mode" -ForegroundColor Cyan
Write-Host "==================================" -ForegroundColor Cyan
Write-Host ""

# Check if we're in the right directory
if (-not (Test-Path "package.json")) {
    Write-Host "❌ Error: Please run this script from the tauri-sveltekit-main directory" -ForegroundColor Red
    exit 1
}

# Check if node_modules exists
if (-not (Test-Path "node_modules")) {
    Write-Host "📦 Installing dependencies..." -ForegroundColor Yellow
    npm install
    Write-Host ""
}

Write-Host "✅ All checks passed!" -ForegroundColor Green
Write-Host ""
Write-Host "Starting SmartGallery in development mode..." -ForegroundColor Cyan
Write-Host "The application window will open in a few seconds." -ForegroundColor Cyan
Write-Host ""
Write-Host "📝 Quick Tips:" -ForegroundColor Yellow
Write-Host "  - The app will open in a new window"
Write-Host "  - Initialize with your ComfyUI output directory"
Write-Host "  - Click 'Sync' to scan for files"
Write-Host "  - Press Ctrl+C here to stop the dev server"
Write-Host ""
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Gray
Write-Host ""

# Start the development server
npm run dev

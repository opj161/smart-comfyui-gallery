#!/bin/bash
# Test Database Initialization Script
# This script tests if the Tauri app can create and initialize the SQLite database

set -e  # Exit on error

echo "=== SmartGallery Phase 5: Database Initialization Test ==="
echo ""

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Test directory
TEST_DIR="/tmp/smartgallery_test"
DB_FILE="$TEST_DIR/gallery_cache.sqlite"

echo -e "${YELLOW}Step 1: Creating test directory${NC}"
mkdir -p "$TEST_DIR"
echo "✓ Test directory: $TEST_DIR"
echo ""

echo -e "${YELLOW}Step 2: Checking if database will be created${NC}"
echo "Expected database location: $DB_FILE"
echo ""

echo -e "${YELLOW}Step 3: Manual testing required${NC}"
echo "Please run the following:"
echo ""
echo "  cd tauri-sveltekit-main"
echo "  npm run dev"
echo ""
echo "In the app:"
echo "  1. Click 'Initialize Gallery' button"
echo "  2. Enter output path: $TEST_DIR"
echo "  3. Enter input path: $TEST_DIR (or any directory)"
echo "  4. Check terminal for success message"
echo ""

echo -e "${YELLOW}Step 4: Verify database structure${NC}"
echo "After initialization, run:"
echo ""
echo "  sqlite3 $DB_FILE"
echo "  .tables"
echo "  .schema files"
echo "  .schema workflow_metadata"
echo ""

echo -e "${GREEN}Expected tables:${NC}"
echo "  - files"
echo "  - workflow_metadata"
echo ""

echo -e "${GREEN}Expected indices (14 total):${NC}"
echo "  - idx_files_name"
echo "  - idx_files_mtime"
echo "  - idx_files_type"
echo "  - idx_files_favorite"
echo "  - idx_files_path"
echo "  - idx_model_name"
echo "  - idx_sampler_name"
echo "  - idx_scheduler"
echo "  - idx_cfg"
echo "  - idx_steps"
echo "  - idx_width"
echo "  - idx_height"
echo "  - idx_workflow_file_id"
echo "  - idx_workflow_file_sampler"
echo ""

echo -e "${GREEN}Test completed setup. Ready for manual testing.${NC}"

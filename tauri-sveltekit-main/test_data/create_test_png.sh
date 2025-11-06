#!/bin/bash
# Create Test PNG Files with Embedded Workflow JSON
# This creates sample PNG files for testing the scanner and parser

set -e

echo "=== Creating Test PNG Files with ComfyUI Workflow Metadata ==="

TEST_DIR="/tmp/smartgallery_test_files"
mkdir -p "$TEST_DIR"

# Sample ComfyUI workflow JSON (UI format)
WORKFLOW_JSON='{
  "last_node_id": 10,
  "last_link_id": 15,
  "nodes": [
    {
      "id": 1,
      "type": "CheckpointLoaderSimple",
      "pos": [10, 10],
      "size": [315, 98],
      "flags": {},
      "order": 0,
      "mode": 0,
      "outputs": [
        {"name": "MODEL", "type": "MODEL", "links": [5], "slot_index": 0},
        {"name": "CLIP", "type": "CLIP", "links": [6, 7], "slot_index": 1},
        {"name": "VAE", "type": "VAE", "links": [8], "slot_index": 2}
      ],
      "properties": {"Node name for S&R": "CheckpointLoaderSimple"},
      "widgets_values": ["sd_xl_base_1.0.safetensors"]
    },
    {
      "id": 2,
      "type": "CLIPTextEncode",
      "pos": [350, 10],
      "size": [400, 200],
      "flags": {},
      "order": 1,
      "mode": 0,
      "inputs": [{"name": "clip", "type": "CLIP", "link": 6}],
      "outputs": [{"name": "CONDITIONING", "type": "CONDITIONING", "links": [10]}],
      "properties": {"Node name for S&R": "CLIPTextEncode"},
      "widgets_values": ["a beautiful landscape with mountains and a lake, sunset, masterpiece, best quality"]
    },
    {
      "id": 3,
      "type": "CLIPTextEncode",
      "pos": [350, 230],
      "size": [400, 200],
      "flags": {},
      "order": 2,
      "mode": 0,
      "inputs": [{"name": "clip", "type": "CLIP", "link": 7}],
      "outputs": [{"name": "CONDITIONING", "type": "CONDITIONING", "links": [11]}],
      "properties": {"Node name for S&R": "CLIPTextEncode"},
      "widgets_values": ["ugly, blurry, low quality, artifacts"]
    },
    {
      "id": 4,
      "type": "KSampler",
      "pos": [800, 10],
      "size": [315, 262],
      "flags": {},
      "order": 3,
      "mode": 0,
      "inputs": [
        {"name": "model", "type": "MODEL", "link": 5},
        {"name": "positive", "type": "CONDITIONING", "link": 10},
        {"name": "negative", "type": "CONDITIONING", "link": 11},
        {"name": "latent_image", "type": "LATENT", "link": 12}
      ],
      "outputs": [{"name": "LATENT", "type": "LATENT", "links": [13]}],
      "properties": {"Node name for S&R": "KSampler"},
      "widgets_values": [42, "fixed", 30, 7.5, "euler", "normal", 1.0]
    }
  ],
  "links": [
    [5, 1, 0, 4, 0, "MODEL"],
    [6, 1, 1, 2, 0, "CLIP"],
    [7, 1, 1, 3, 0, "CLIP"],
    [10, 2, 0, 4, 1, "CONDITIONING"],
    [11, 3, 0, 4, 2, "CONDITIONING"]
  ],
  "groups": [],
  "config": {},
  "extra": {},
  "version": 0.4
}'

# Note: Creating actual PNG files with tEXt chunks requires Python or ImageMagick
# This is a placeholder script that shows what would be needed

echo "⚠️  This script requires Python with PIL to create actual PNG files"
echo ""
echo "To create test files with workflow metadata, you need:"
echo "  1. Python 3.x with Pillow (PIL)"
echo "  2. OR ImageMagick with PNG support"
echo ""
echo "Sample Python code to create test PNG:"
echo ""
cat << 'PYTHON_CODE'
from PIL import Image, PngImagePlugin

# Create a simple image
img = Image.new('RGB', (1024, 1024), color='blue')

# Prepare workflow JSON as tEXt chunk
workflow_json = '...'  # Your workflow JSON here

# Create PngInfo object
png_info = PngImagePlugin.PngInfo()
png_info.add_text("workflow", workflow_json)
png_info.add_text("prompt", "test prompt")

# Save with metadata
img.save("/tmp/test_image.png", pnginfo=png_info)
PYTHON_CODE

echo ""
echo "For now, use actual PNG files from ComfyUI output for testing."
echo "Test directory ready at: $TEST_DIR"

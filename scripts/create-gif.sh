#!/bin/bash

# Script to convert video recordings to optimized GIFs for ProManager documentation

# Check if ffmpeg is installed
if ! command -v ffmpeg &> /dev/null; then
    echo "ffmpeg is required but not installed. Install with: brew install ffmpeg"
    exit 1
fi

# Check if input file is provided
if [ $# -eq 0 ]; then
    echo "Usage: $0 <input-video-file> [output-name]"
    echo "Example: $0 dashboard-demo.mov dashboard-demo"
    exit 1
fi

INPUT_FILE="$1"
OUTPUT_NAME="${2:-demo}"
OUTPUT_FILE="../assets/${OUTPUT_NAME}.gif"

# Check if input file exists
if [ ! -f "$INPUT_FILE" ]; then
    echo "Error: Input file '$INPUT_FILE' not found"
    exit 1
fi

echo "Converting $INPUT_FILE to $OUTPUT_FILE..."

# Create optimized GIF with good quality and small file size
# - fps=15: 15 frames per second for smooth animation
# - scale=1000:-1: Max width 1000px, maintain aspect ratio
# - palettegen/paletteuse: Better color optimization

# Generate palette
ffmpeg -v warning -i "$INPUT_FILE" -vf "fps=15,scale=1000:-1:flags=lanczos,palettegen=stats_mode=diff" -y palette.png

# Create GIF using the palette
ffmpeg -v warning -i "$INPUT_FILE" -i palette.png -lavfi "fps=15,scale=1000:-1:flags=lanczos [x]; [x][1:v] paletteuse=dither=bayer:bayer_scale=5:diff_mode=rectangle" -y "$OUTPUT_FILE"

# Clean up palette file
rm palette.png

echo "✅ GIF created: $OUTPUT_FILE"

# Check file size
FILE_SIZE=$(du -h "$OUTPUT_FILE" | cut -f1)
echo "📊 File size: $FILE_SIZE"

echo ""
echo "💡 Tips for better GIFs:"
echo "   - Keep recordings short (5-15 seconds)"
echo "   - Use smooth, deliberate movements"
echo "   - Avoid rapid scrolling or clicking"
echo "   - Record at consistent frame rate"

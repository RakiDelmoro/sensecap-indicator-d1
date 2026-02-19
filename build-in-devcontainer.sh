#!/bin/bash
# Build firmware inside devcontainer (no Docker needed)
# Outputs binary for flashing with espflash on host

set -e

FIRMWARE_PATH="/workspaces/sensecap-indicator-d1/firmware"

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${GREEN}SenseCAP Indicator D1 - Devcontainer Build${NC}"
echo "=========================================="
echo ""

# Source ESP-IDF
echo "Sourcing ESP-IDF..."
. /home/esp/esp-idf/export.sh

cd "$FIRMWARE_PATH"

# Check if already built
if [ ! -d "build" ]; then
    echo "[1/2] Setting target to ESP32-S3..."
    idf.py set-target esp32s3
fi

echo "[2/2] Building firmware..."
idf.py build

echo ""
echo -e "${GREEN}✓ Build complete!${NC}"
echo ""
echo "Binary location:"
echo "  $FIRMWARE_PATH/build/sensecap-indicator-d1.bin"
echo ""
echo "To flash on host:"
echo "  espflash flash $FIRMWARE_PATH/build/sensecap-indicator-d1.bin"
echo ""

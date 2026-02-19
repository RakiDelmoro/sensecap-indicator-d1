#!/bin/bash
# Build firmware for SenseCAP Indicator D1 using ESP-IDF Docker
# Outputs binary to firmware/build/ directory for flashing with espflash

set -e

PROJECT_PATH="/workspaces/sensecap-indicator-d1"
FIRMWARE_PATH="$PROJECT_PATH/firmware"
DOCKER_IMAGE="espressif/idf:v5.0.2"

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${GREEN}SenseCAP Indicator D1 - Build Script${NC}"
echo "======================================"
echo ""

# Check if already built
if [ -f "$FIRMWARE_PATH/build/sensecap-indicator-d1.bin" ]; then
    echo -e "${YELLOW}Note: Firmware already built${NC}"
    echo "To rebuild, delete: $FIRMWARE_PATH/build/"
    echo ""
fi

echo "Building with ESP-IDF Docker..."
echo "This may take a few minutes on first build."
echo ""

# Build using Docker (non-interactive)
docker run --rm \
    -v "$PROJECT_PATH:/project" \
    -w /project/firmware \
    -e HOME=/tmp \
    "$DOCKER_IMAGE" \
    bash -c "
        echo '[1/3] Setting target to ESP32-S3...'
        idf.py set-target esp32s3 2>&1 | tail -5
        
        echo '[2/3] Building...'
        idf.py build 2>&1 | tail -20
        
        echo '[3/3] Done!'
    "

echo ""
echo -e "${GREEN}✓ Build complete!${NC}"
echo ""
echo "Firmware files:"
echo "  Binary: $FIRMWARE_PATH/build/sensecap-indicator-d1.bin"
echo "  ELF:    $FIRMWARE_PATH/build/sensecap-indicator-d1.elf"
echo ""
echo "To flash with espflash:"
echo "  espflash flash $FIRMWARE_PATH/build/sensecap-indicator-d1.bin"
echo ""
echo "To flash + monitor:"
echo "  espflash flash $FIRMWARE_PATH/build/sensecap-indicator-d1.bin --monitor"
echo ""

#!/bin/bash
# Flash and monitor script for SenseCAP Indicator D1
# Uses ESP-IDF Docker container

set -e

PROJECT_PATH="/workspaces/sensecap-indicator-d1"
DOCKER_IMAGE="espressif/idf:v5.0.2"
USB_DEVICE="/dev/ttyUSB0"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${GREEN}SenseCAP Indicator D1 - Flash & Monitor${NC}"
echo "========================================"
echo ""

# Check USB device
if [ ! -e "$USB_DEVICE" ]; then
    echo -e "${RED}Error: USB device $USB_DEVICE not found${NC}"
    echo "Make sure the device is connected."
    echo ""
    echo "Available USB devices:"
    ls -la /dev/ttyUSB* 2>/dev/null || ls -la /dev/ttyACM* 2>/dev/null || echo "No USB serial devices found"
    exit 1
fi

echo -e "${GREEN}USB device found: $USB_DEVICE${NC}"
echo "Flashing firmware and starting monitor..."
echo ""

# Flash and monitor
docker run --rm -it \
    --privileged \
    --device="$USB_DEVICE" \
    -v "$PROJECT_PATH:/project" \
    -w /project/firmware \
    "$DOCKER_IMAGE" \
    idf.py flash monitor

echo ""
echo -e "${GREEN}Done!${NC}"

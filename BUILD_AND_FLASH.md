# Building and Flashing Guide

## Quick Start

### 1. Build Inside Devcontainer

```bash
# Use the helper script
./build-in-devcontainer.sh
```

Or manually:
```bash
cd /workspaces/sensecap-indicator-d1/firmware
. /home/esp/esp-idf/export.sh
idf.py set-target esp32s3
idf.py build
```

**Output:** Binary at `firmware/build/sensecap-indicator-d1.bin`

### 2. Flash on Host with espflash

On your **host machine** (not in devcontainer):

```bash
# Install espflash if needed
cargo install espflash

# Flash the device
espflash flash /workspaces/sensecap-indicator-d1/firmware/build/sensecap-indicator-d1.bin

# Flash and monitor
espflash flash /workspaces/sensecap-indicator-d1/firmware/build/sensecap-indicator-d1.bin --monitor
```

## Alternative: Flash Inside Devcontainer

If you want to flash directly from the devcontainer (device must be passed through):

```bash
# From devcontainer
cd /workspaces/sensecap-indicator-d1/firmware
. /home/esp/esp-idf/export.sh
idf.py flash

# Or flash + monitor
idf.py flash monitor
```

## Build Details

The devcontainer includes:
- ESP-IDF v5.0.2 at `/home/esp/esp-idf/`
- All required ESP32-S3 toolchain components
- No Rust needed (pure C firmware)

## Troubleshooting

**Build fails with "command not found: idf.py"**
```bash
# Source ESP-IDF environment
. /home/esp/esp-idf/export.sh
```

**Device not found on host**
```bash
# Check device
ls /dev/ttyUSB*
# Should show /dev/ttyUSB0 (or similar)

# Flash with explicit port
espflash flash --port /dev/ttyUSB0 firmware/build/sensecap-indicator-d1.bin
```

**Permission denied on device**
```bash
# Add user to dialout group (on host)
sudo usermod -aG dialout $USER
# Log out and back in
```

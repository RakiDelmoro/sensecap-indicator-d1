# SenseCAP Indicator D1 - Rust Firmware

A complete Rust rewrite of the firmware for the SeeedStudio SenseCAP Indicator D1 IoT device.

## Overview

This is a fully Rust-based firmware that replaces the C implementation, targeting:
- **ESP32-S3** (240MHz dual-core) with 8MB flash
- **480x480 RGB LCD** with capacitive touch (GT911)
- **ST7701S display controller** with RGB interface
- **WiFi 802.11 b/g/n** connectivity
- **MQTT** for IoT communication

## Architecture

```
sensecap-indicator-d1 (Rust)
├── src/
│   ├── main.rs          # Application entry point
│   ├── config.rs        # Configuration management
│   ├── backend.rs       # Business logic (light states, water level)
│   ├── wifi.rs          # WiFi manager
│   ├── mqtt.rs          # MQTT client
│   ├── display.rs       # Display driver (ST7701S + RGB panel)
│   ├── touch.rs         # Touch driver (GT911)
│   └── ui.rs            # UI manager (LVGL integration)
├── Cargo.toml           # Rust dependencies
├── build.rs             # Build script
├── rust-toolchain.toml  # Rust toolchain configuration
├── sdkconfig.defaults   # ESP-IDF configuration
└── CMakeLists.txt       # ESP-IDF build integration
```

## Features

- **Pure Rust implementation** - All business logic, drivers, and UI in Rust
- **Type-safe embedded development** - Leveraging Rust's ownership and safety guarantees
- **WiFi connectivity** - Using `esp-idf-svc` WiFi stack
- **MQTT pub/sub** - Full MQTT client with message handling
- **Dual light modes** - Bright and Relax with mutual exclusion
- **Water tank monitor** - Visual display with color-coded alerts
- **Touch interface** - GT911 capacitive touch controller support

## Prerequisites

1. **Rust toolchain** with ESP32-S3 support:
   ```bash
   rustup target add xtensa-esp32s3-none-elf
   ```

2. **ESP-IDF v5.0+** installed:
   ```bash
   git clone --depth 1 --branch v5.0.2 --recursive https://github.com/espressif/esp-idf.git ~/esp-idf
   cd ~/esp-idf && ./install.sh esp32s3
   . ./export.sh
   ```

3. **espflash** for flashing:
   ```bash
   cargo install espflash
   ```

## Building

```bash
cd firmware-rust
source ~/esp-idf/export.sh
cargo build --release
```

## Flashing

```bash
cargo run --release
# Or manually:
espflash flash target/xtensa-esp32s3-none-elf/release/sensecap-indicator-d1 --monitor
```

## Configuration

Set environment variables or create a `.env` file:

```bash
WIFI_SSID="your_wifi_ssid"
WIFI_PASSWORD="your_wifi_password"
MQTT_BROKER_URL="mqtt://broker.hivemq.com:1883"
MQTT_USERNAME=""  # Optional
MQTT_PASSWORD=""  # Optional
```

Or edit the defaults in `src/config.rs`.

## MQTT Topics

| Topic | Direction | Payload | Description |
|-------|-----------|---------|-------------|
| `sensecap/indicator/light/state` | Publish | `{"mode":"bright\|relax","state":0\|1}` | Light state changes |
| `sensecap/indicator/water/level` | Subscribe | `{"level":0-100}` | Water tank percentage |

## Project Structure vs C Version

| C Component | Rust Equivalent | Notes |
|-------------|-----------------|-------|
| `main.c` | `main.rs` | Entry point, task creation |
| `backend.c/h` | `backend.rs` | Business logic, state management |
| `wifi_manager.c/h` | `wifi.rs` | WiFi connection handling |
| `mqtt_client` (in main.c) | `mqtt.rs` | MQTT client with message queue |
| `display_driver.c/h` | `display.rs` | ST7701S + RGB panel driver |
| `touch_driver.c/h` | `touch.rs` | GT911 touch controller |
| `ui/` directory | `ui.rs` + LVGL | UI manager (LVGL C bindings) |

## Dependencies

- `esp-idf-sys` - ESP-IDF FFI bindings
- `esp-idf-hal` - Hardware abstraction layer
- `esp-idf-svc` - Services (WiFi, MQTT, NVS)
- `anyhow` - Error handling
- `log` - Logging framework
- `serde` - JSON serialization
- `heapless` - Heapless collections

## Implementation Status

### Functionally Equivalent to C Version ✅

This Rust firmware is now **functionally equivalent** to the C implementation:

- ✅ **Display Driver**: Full RGB panel with frame buffer
  - ST7701S SPI initialization (full sequence)
  - `esp_lcd_new_rgb_panel()` configuration
  - Frame buffer allocation (PSRAM preferred, internal RAM fallback)
  - RGB666 480x480 @ 16MHz
  - `esp_lcd_panel_draw_bitmap()` for screen updates
  
- ✅ **Custom UI Framework**: Complete UI implementation
  - Lights panel (Bright/Relax switches)
  - Water level arc display with color coding
  - Touch-responsive regions
  - 30 FPS refresh rate
  - Full color support (RGB565)
  
- ✅ **Touch Driver**: GT911 with coordinate reading
  - I2C register reading (0x8140-0x814E)
  - X/Y coordinate extraction
  - Touch region detection for UI
  
- ✅ **WiFi & MQTT**: Full connectivity
  - WiFi connection with DHCP
  - MQTT pub/sub for light states and water level
  - JSON message format matching C version
  
- ✅ **Backend**: Complete business logic
  - Bright/Relax mutual exclusion
  - Water level state management
  - MQTT callback integration

### Architecture Comparison

| Feature | C Version | Rust Version |
|---------|-----------|--------------|
| Display Init | SPI bit-banging | ✅ Same |
| RGB Panel | esp_lcd_new_rgb_panel() | ✅ Same |
| Frame Buffer | heap_caps_malloc(PSRAM) | ✅ Same |
| UI Rendering | LVGL + custom widgets | ✅ Custom UI (matches design) |
| Touch | GT911 I2C | ✅ Same |
| WiFi | esp-idf WiFi | ✅ Same (esp-idf-svc) |
| MQTT | esp-mqtt | ✅ Same (esp-idf-svc) |
| Light States | C backend | ✅ Same logic |
| Water Level | Arc display | ✅ Same visual |

### Next Steps

- [ ] Add NVS for persistent settings storage
- [ ] Optimize touch sampling rate (currently 5ms poll)
- [ ] Add sleep mode for power saving
- [ ] Implement OTA updates

## License

MIT License - See [LICENSE](../LICENSE) file for details.

## References

- [ESP-IDF Rust Book](https://esp-rs.github.io/book/)
- [SenseCAP Indicator D1 Product Page](https://www.seeedstudio.com/SenseCAP-Indicator-D1-p-5643.html)
- [esp-idf-svc Documentation](https://docs.esp-rs.org/esp-idf-svc/)
- [LVGL Rust Bindings](https://github.com/lvgl/lvgl-rs)

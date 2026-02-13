# LVGL PC Simulator for SenseCap Indicator

This simulator allows you to run the SquareLine Studio generated UI on your PC using SDL2.

## Features

- **Pixel-perfect rendering** - Uses actual LVGL C code
- **Mouse & keyboard input** - Test interactions without hardware
- **480x480 display** - Matches SenseCap Indicator D1 circular display
- **Fast iteration** - Build and test UI changes in seconds
- **Cross-platform** - Works on Linux, macOS, and Windows

## Requirements

### Linux (Ubuntu/Debian)
```bash
sudo apt-get update
sudo apt-get install -y libsdl2-dev cmake build-essential git
```

### macOS
```bash
brew install sdl2 cmake git
```

### Windows
Use WSL2 with Ubuntu, or install:
- [SDL2 Development Libraries](https://www.libsdl.org/download-2.0.php)
- [CMake](https://cmake.org/download/)
- [Git for Windows](https://git-scm.com/download/win)

## Build Instructions

### 1. Clone LVGL (if not already present)
```bash
cd /workspaces/sensecap-indicator-d1/simulator

# Clone LVGL v8.3.11 (matching your UI version)
git clone --depth 1 --branch v8.3.11 https://github.com/lvgl/lvgl.git

# Clone lv_drivers for SDL2 support
git clone --depth 1 https://github.com/lvgl/lv_drivers.git
```

### 2. Create build directory
```bash
mkdir -p build
cd build
```

### 3. Configure with CMake
```bash
cmake ..
```

### 4. Build
```bash
make -j$(nproc)
```

### 5. Run
```bash
./sensecap-simulator
```

## Project Structure

```
simulator/
├── CMakeLists.txt          # Build configuration
├── lv_conf.h               # LVGL configuration (16-bit color, fonts, etc.)
├── src/
│   └── main.c             # Simulator entry point
├── lvgl/                  # LVGL library (v8.3.11)
├── lv_drivers/            # Display/input drivers
└── ui/                    # Your UI files from SquareLine Studio
    ├── ui.c
    ├── ui.h
    ├── ui_helpers.c
    ├── ui_theme_manager.c
    ├── screens/
    │   └── ui_Screen_1.c
    ├── components/
    │   └── ui_comp_hook.c
    └── images/
        ├── ui_img_166333148.c
        └── ui_img_353436330.c
```

## Display Configuration

The simulator uses a **480x480 window** to match the SenseCap Indicator D1 display:

- **Resolution**: 480x480 pixels
- **Color depth**: 16-bit (RGB565)
- **Refresh rate**: 60 FPS
- **Input**: Mouse (touch simulation)

## Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `ESC` | Quit simulator |
| `F11` | Toggle fullscreen |
| `R` | Reload UI (if implemented) |

## Troubleshooting

### "SDL2 not found" error
Make sure SDL2 development libraries are installed:
```bash
# Ubuntu/Debian
sudo apt-get install libsdl2-dev

# macOS
brew install sdl2

# Verify installation
sdl2-config --version
```

### "lvgl/lvgl.h not found" error
LVGL is not cloned or not in the expected location:
```bash
cd /workspaces/sensecap-indicator-d1/simulator
git clone --depth 1 --branch v8.3.11 https://github.com/lvgl/lvgl.git
```

### Build fails with "undefined reference"
Clean and rebuild:
```bash
cd build
rm -rf *
cmake ..
make -j$(nproc)
```

### Screen is black
The UI might not be loading. Check console output for errors.
Make sure `lv_conf.h` is in the simulator root directory.

## Next Steps: Connect Rust Backend

To connect your Rust backend to this C UI simulator, you'll need to:

1. **Create FFI bindings** - Expose Rust functions to C
2. **Modify event handlers** - Replace C event handlers with Rust callbacks
3. **Build as library** - Compile simulator with Rust static library

Example FFI layer:
```c
// In C - thin wrapper calling Rust
extern void rust_handle_bright_toggle(bool is_on);

void ui_event_Bright(lv_event_t * e) {
    lv_event_code_t event_code = lv_event_get_code(e);
    if(event_code == LV_EVENT_VALUE_CHANGED) {
        bool is_checked = lv_obj_has_state(ui_Bright, LV_STATE_CHECKED);
        rust_handle_bright_toggle(is_checked);
    }
}
```

```rust
// In Rust
#[no_mangle]
pub extern "C" fn rust_handle_bright_toggle(is_on: bool) {
    println!("Bright button toggled: {}", is_on);
    // Your business logic here
}
```

## License

This simulator uses:
- **LVGL**: MIT License
- **SDL2**: zlib License
- **Your UI files**: Based on your SquareLine Studio project

## Support

- LVGL Documentation: https://docs.lvgl.io/8.3/
- LVGL Forum: https://forum.lvgl.io/
- SDL2 Wiki: https://wiki.libsdl.org/

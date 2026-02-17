#!/bin/bash
# Build script for LVGL PC Simulator

set -e

SIMULATOR_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SIMULATOR_DIR"

echo "========================================"
echo "SenseCap Indicator LVGL PC Simulator"
echo "========================================"
echo ""

# Check if LVGL is cloned
if [ ! -d "lvgl/src" ]; then
    echo "⚠️  LVGL not found. Cloning..."
    git clone --depth 1 --branch v8.3.11 https://github.com/lvgl/lvgl.git
fi

# Check if lv_drivers is cloned
if [ ! -d "lv_drivers/display" ]; then
    echo "⚠️  lv_drivers not found. Cloning..."
    git clone --depth 1 https://github.com/lvgl/lv_drivers.git
fi

# Check if SDL2 is installed
echo "📦 Checking dependencies..."
if command -v sdl2-config &> /dev/null; then
    echo "✅ SDL2 found: $(sdl2-config --version)"
else
    echo "❌ SDL2 not found!"
    echo ""
    echo "Please install SDL2:"
    echo "  Ubuntu/Debian: sudo apt-get install libsdl2-dev"
    echo "  macOS: brew install sdl2"
    echo ""
    exit 1
fi

if command -v cmake &> /dev/null; then
    echo "✅ CMake found: $(cmake --version | head -1)"
else
    echo "❌ CMake not found!"
    echo "  Install with: sudo apt-get install cmake"
    exit 1
fi

echo ""

# Create build directory
echo "🔨 Creating build directory..."
mkdir -p build
cd build

# Configure
echo "🔧 Configuring with CMake..."
if cmake ..; then
    echo "✅ Configuration successful"
else
    echo "❌ Configuration failed!"
    exit 1
fi

echo ""

# Build
echo "🏗️  Building simulator..."
if make -j$(nproc 2>/dev/null || sysctl -n hw.ncpu 2>/dev/null || echo 4); then
    echo ""
    echo "✅ Build successful!"
    echo ""
    echo "🚀 Run the simulator:"
    echo "   cd build && ./sensecap-simulator"
    echo ""
else
    echo "❌ Build failed!"
    echo ""
    echo "Common issues:"
    echo "  - Missing headers: Make sure SDL2 dev packages are installed"
    echo "  - Missing lvgl: Run: git clone --depth 1 --branch v8.3.11 https://github.com/lvgl/lvgl.git"
    echo "  - Missing lv_drivers: Run: git clone --depth 1 https://github.com/lvgl/lv_drivers.git"
    echo ""
    exit 1
fi

# Setting up ESP32-S3 Rust Toolchain

The ESP32-S3 uses the **Xtensa architecture**, which requires a custom Rust toolchain with Xtensa LLVM support. The standard Rust nightly doesn't include this.

## Quick Setup

### 1. Install `espup` (ESP Rust toolchain installer)

```bash
cargo install espup
espup install
```

This will install:
- The custom Rust toolchain with Xtensa support
- Required tools like `espflash`, `ldproxy`, etc.

### 2. Export the environment

After `espup install`, you need to source the environment file:

```bash
. ~/export-esp.sh
```

Or add to your `.bashrc`:
```bash
echo '. ~/export-esp.sh' >> ~/.bashrc
```

### 3. Verify installation

```bash
rustc --print target-list | grep xtensa
```

Should show:
- `xtensa-esp32-none-elf`
- `xtensa-esp32s2-none-elf`
- `xtensa-esp32s3-none-elf`
- `xtensa-esp32s3-espidf` (for ESP-IDF projects)

### 4. Install ldproxy

```bash
cargo install ldproxy
```

### 5. Install espflash

```bash
cargo install espflash
```

## Building the Project

After setup:

```bash
cd firmware-rust
source ~/export-esp.sh
source ~/esp-idf/export.sh  # If using ESP-IDF integration
cargo build
```

## Troubleshooting

### "component 'rust-std' for target 'xtensa-esp32s3-none-elf' is unavailable"

You haven't installed the esp-rs toolchain. Run:
```bash
espup install
. ~/export-esp.sh
```

### "linker 'ldproxy' not found"

Install ldproxy:
```bash
cargo install ldproxy
```

### Build fails with ESP-IDF errors

Make sure ESP-IDF is properly set up:
```bash
cd ~/esp-idf
./install.sh esp32s3
. ./export.sh
```

## Alternative: Manual Toolchain Installation

If `espup` doesn't work, install manually:

```bash
# Clone the rust-build repo
git clone https://github.com/esp-rs/rust-build.git
cd rust-build

# Install the toolchain
./install-rust-toolchain.sh --target xtensa-esp32s3-espidf

# Source the environment
source export-esp.sh
```

## More Information

- [ESP-RS Book](https://esp-rs.github.io/book/)
- [ESP-RS Installation Guide](https://esp-rs.github.io/book/installation/index.html)
- [ESP-RS Github](https://github.com/esp-rs)

# ESP32 WROOM Rust Project

A comprehensive Rust project template for ESP32 WROOM development using the `esp-hal` framework and bare-metal programming.

## 📋 Table of Contents

- [Prerequisites](#prerequisites)
- [Hardware Requirements](#hardware-requirements)
- [Development Environment Setup](#development-environment-setup)
- [Project Structure](#project-structure)
- [Configuration Files Explained](#configuration-files-explained)
- [Building and Flashing](#building-and-flashing)
- [Troubleshooting](#troubleshooting)
- [Additional Resources](#additional-resources)
- [Contributing](#contributing)

## 🔧 Prerequisites

Before setting up this project, ensure you have the following installed on your system:

### System Requirements
- **Operating System**: Linux (Ubuntu/Debian preferred), macOS, or Windows with WSL2
- **RAM**: Minimum 4GB (8GB+ recommended for faster builds)
- **Storage**: At least 2GB free space for toolchain and dependencies

### Required Software
- **Git**: For version control
- **Python 3**: Required by ESP-IDF tools
- **USB drivers**: For ESP32 communication

## 🛠 Hardware Requirements

### ESP32 WROOM Development Board
- **Chip**: ESP32-WROOM-32 or ESP32-WROOM-32D/U
- **Flash Memory**: Minimum 4MB (this project is optimized for standard configurations)
- **USB Cable**: Micro-USB or USB-C (depending on your board)
- **Optional**: Breadboard and jumper wires for additional components

### Supported Development Boards
- ESP32 DevKit V1
- ESP32 WROOM-32 DevKit
- NodeMCU ESP32
- TTGO T-Display ESP32
- Any ESP32 board with WROOM-32 module

## 🚀 Development Environment Setup

### Step 1: Install Rust

```bash
# Install Rust using rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Verify installation
rustc --version
cargo --version
```

### Step 2: Install ESP Rust Toolchain

The ESP32 requires a special Rust toolchain that supports the Xtensa architecture:

```bash
# Install the ESP Rust toolchain
cargo install espup
espup install

# Source the export file (add this to your shell profile for persistence)
source ~/export-esp.sh
```

**Important**: Add `source ~/export-esp.sh` to your shell profile (`~/.bashrc`, `~/.zshrc`, etc.) to automatically set up the environment in new terminal sessions.

### Step 3: Install ESP Flash Tool

```bash
# Install espflash for flashing and monitoring
cargo install espflash
```

### Step 4: Install Additional Tools (Optional but Recommended)

```bash
# Install cargo-espflash for easier project management
cargo install cargo-espflash

# Install ldproxy for better linker integration
cargo install ldproxy
```

### Step 5: Verify Toolchain Installation

```bash
# Check if the Xtensa toolchain is available
rustup toolchain list | grep esp

# Verify espflash installation
espflash --version
```

## 📁 Project Structure

```
esp32-rust-project/
├── .cargo/
│   └── config.toml          # Cargo configuration for ESP32 target
├── src/
│   ├── lib.rs              # Library root (minimal, no_std)
│   └── bin/
│       └── main.rs         # Main application entry point
├── .gitignore              # Git ignore patterns
├── build.rs                # Build script with linker helpers
├── Cargo.lock              # Dependency lock file
├── Cargo.toml              # Project manifest and dependencies
├── rust-toolchain.toml     # Rust toolchain specification
└── README.md               # This file
```

## ⚙️ Configuration Files Explained

### `rust-toolchain.toml`
Specifies the ESP Rust toolchain:
```toml
[toolchain]
channel = "esp"
```

### `.cargo/config.toml`
Configures Cargo for ESP32 development:
- **Target**: `xtensa-esp32-none-elf` (ESP32 bare-metal target)
- **Runner**: `espflash flash --monitor --chip esp32` (automatic flashing and monitoring)
- **Build flags**: Optimized for ESP32 linking
- **Build-std**: Uses core library rebuilding for the target

### `Cargo.toml`
Project configuration with:
- **ESP32 HAL**: `esp-hal` for hardware abstraction
- **Bootloader**: `esp-bootloader-esp-idf` for ESP-IDF compatibility
- **Optimization**: Configured for size optimization (`opt-level = "s"`)
- **Profile settings**: Development and release profiles optimized for embedded use

### `build.rs`
Build script providing:
- Linker script integration (`linkall.x`)
- Helpful error messages for common issues
- Build-time configuration

## 🔨 Building and Flashing

### Clone and Setup the Project

```bash
# Clone this repository
git clone <your-repo-url>
cd esp32-rust-project

# The toolchain will be automatically selected based on rust-toolchain.toml
```

### Building the Project

```bash
# Build for development (with optimizations for faster execution)
cargo build

# Build for release (maximum optimizations)
cargo build --release
```

### Flashing to ESP32

#### Method 1: Using Cargo Run (Recommended)
```bash
# Connect your ESP32 via USB and run:
cargo run

# For release build:
cargo run --release
```

#### Method 2: Using espflash directly
```bash
# Build first
cargo build --release

# Flash manually
espflash flash --monitor --chip esp32 target/xtensa-esp32-none-elf/release/esp32-rust-project
```

### Monitoring Serial Output

```bash
# Monitor serial output (if not using cargo run)
espflash monitor
```

## 🐛 Troubleshooting

### Common Issues and Solutions

#### 1. "espup not found" or toolchain issues
```bash
# Reinstall espup
cargo install espup --force
espup install
source ~/export-esp.sh
```

#### 2. Permission denied on USB device (Linux)
```bash
# Add user to dialout group
sudo usermod -a -G dialout $USER
# Log out and back in, or restart your session
```

#### 3. "No such file or directory: espflash"
```bash
# Reinstall espflash
cargo install espflash --force
```

#### 4. Build fails with linker errors
- Ensure `source ~/export-esp.sh` has been run in your terminal
- Verify the ESP toolchain is installed: `rustup toolchain list | grep esp`
- Try cleaning and rebuilding: `cargo clean && cargo build`

#### 5. ESP32 not detected
- Check USB cable (ensure it's a data cable, not power-only)
- Try different USB ports
- On Windows, install ESP32 USB drivers
- Reset ESP32 board while connecting

#### 6. Flash operation fails
```bash
# Try holding the BOOT button while flashing
# Or use explicit chip specification
espflash flash --chip esp32 target/xtensa-esp32-none-elf/release/esp32-rust-project
```

### Debugging Tips

1. **Verbose output**: Use `cargo run -v` for detailed build information
2. **Clean builds**: Run `cargo clean` if experiencing persistent issues
3. **Check connections**: Ensure proper USB connection and drivers
4. **Serial monitor**: Use `espflash monitor` to see runtime output
5. **Reset board**: Press the reset button on ESP32 after flashing

## 🔍 Code Overview

### Current Implementation

The current `main.rs` includes:
- **No-std environment**: Bare-metal programming without standard library
- **Panic handler**: Custom panic handler for embedded environment
- **ESP bootloader integration**: Proper bootloader descriptors
- **CPU clock configuration**: Maximum performance setup
- **Simple delay loop**: Demonstrates basic timing operations

### Key Features
- **Optimized builds**: Size-optimized for embedded constraints
- **Fast compilation**: Development profile with reasonable optimization
- **Memory safety**: Rust's ownership system prevents common embedded bugs
- **Hardware abstraction**: Clean interface to ESP32 peripherals via esp-hal

### Example Extensions

The project is set up to easily add:
- GPIO control for LEDs and sensors
- WiFi connectivity
- Bluetooth communication
- I2C/SPI device interaction
- PWM for motor control
- ADC for analog sensors

## 📚 Additional Resources

### Documentation
- [ESP32 Rust Book](https://esp-rs.github.io/book/)
- [esp-hal Documentation](https://docs.rs/esp-hal/)
- [ESP32 Technical Reference](https://www.espressif.com/sites/default/files/documentation/esp32_technical_reference_manual_en.pdf)

### Example Projects
- [ESP-HAL Examples](https://github.com/esp-rs/esp-hal/tree/main/examples)
- [ESP32 Rust Examples](https://github.com/esp-rs/esp32-hal/tree/master/examples)

### Community
- [ESP-RS Matrix Channel](https://matrix.to/#/#esp-rs:matrix.org)
- [ESP32 Rust Reddit](https://www.reddit.com/r/esp32/)
- [ESP-RS GitHub Organization](https://github.com/esp-rs)

## 🤝 Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Development Guidelines
- Follow Rust naming conventions
- Use `cargo fmt` for code formatting
- Run `cargo clippy` for linting
- Test on actual hardware when possible
- Document public APIs

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🙏 Acknowledgments

- ESP-RS community for excellent tooling and documentation
- Espressif for the ESP32 platform
- Rust Embedded Working Group for embedded Rust ecosystem

---

**Note**: This README assumes you're working with ESP32 WROOM modules. For other ESP32 variants (ESP32-S2, ESP32-S3, ESP32-C3), you may need to adjust the target configuration and dependencies accordingly.

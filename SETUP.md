# SensePlus Hardware Backend Rust - Setup Guide

This guide will help you set up your development environment for the SensePlus Hardware Backend Rust project, targeting the ESP32 platform.

---

## Prerequisites
- Rust toolchain (with support for ESP32)
- [espup](https://github.com/esp-rs/espup) utility
- ESP32 hardware (or emulator)

---

## Step-by-Step Setup

### 1. Install Rust (ESP Toolchain)
Ensure you have the correct Rust toolchain for ESP32. The project uses a custom toolchain specified in `rust-toolchain.toml`:

```
channel = "esp"
```

You can install the toolchain using [espup](https://github.com/esp-rs/espup) (see below).

---

### 2. Install espup
Install the `espup` utility to manage ESP toolchains:

```sh
cargo install espup@0.12.0
```

---

### 3. Setup ESP Toolchain with espup
Run the following command to install the ESP32 toolchain:

```sh
espup install --targets esp32
```

---

### 4. Export Environment Variables
Source your environment variables (adjust the path if needed):

```sh
. /Users/shreyansh/export-esp.sh
```

---

### 5. Build the Project
Build the project using Cargo:

```sh
cargo build
```

---

## Configuration Files
- **rust-toolchain.toml**: Specifies the Rust toolchain channel (`esp`).
- **.cargo/config.toml**: Contains target and build configuration for ESP32, including runner and environment variables.
- **Cargo.toml**: Project dependencies and build profiles.

---

## Additional Notes
- The project is configured for the `xtensa-esp32-none-elf` target.
- Logging level can be set via the `ESP_LOG` environment variable (default: `info`).
- For flashing and monitoring, the runner uses `espflash`.

---

For more details, refer to the main [README.md](./README.md) or the official [esp-rs book](https://esp-rs.github.io/book/).

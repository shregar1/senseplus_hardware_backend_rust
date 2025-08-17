# SensePlus Hardware Backend - ESP32 Rust Project

## 🚀 Project Overview

This is a Rust-based embedded firmware project for the SensePlus hardware platform, designed to run on ESP32 microcontrollers. The project implements a modular sensor data acquisition and processing system with a clean, maintainable architecture.

## 🏗️ Project Architecture

This project follows **Domain-Driven Design (DDD)** principles and **Clean Architecture** patterns, adapted for embedded systems development. The folder structure reflects separation of concerns and modularity, which are crucial for both software and firmware development.

## 📁 Folder Structure & Significance

### **Core Architecture Layers**
- **`src/abstractions/`** - Interface definitions and contracts (Abstraction Layer)
- **`src/implementations/`** - Concrete implementations of abstractions (Implementation Layer)
- **`src/services/`** - Business logic and orchestration (Service Layer)
- **`src/dtos/`** - Data Transfer Objects for API communication (Data Layer)

### **Domain-Specific Modules**
- **`src/sensors/`** - Sensor drivers and measurement logic (Domain Layer)
- **`src/factories/` - Object creation and dependency injection (Factory Pattern)
- **`src/pipelines/`** - Data processing workflows (Pipeline Pattern)

### **Configuration & Constants**
- **`src/configs/`** - Configuration files and settings
- **`src/constants/` - Application constants and magic numbers
- **`src/configurations/`** - Configuration management logic

## 🛠️ Development Setup

### **Prerequisites**
- Rust 1.86+ with ESP32 target
- ESP-IDF v5.0+
- ESP-HAL toolchain
- Cargo and related tools

### **Installation**
```bash
# Clone the repository
git clone <repository-url>
cd senseplus_hardware_backend_rust

# Install ESP32 target
rustup target add xtensa-esp32-none-elf

# Install ESP-HAL toolchain
cargo install espflash
cargo install espmonitor

# Check compilation
cargo check
```

### **Building & Flashing**
```bash
# Build the project
. /Users/shreyansh/Documents/esp-idf/export.sh
or
. /Users/shreyansh/export-esp.sh

cargo build

# Flash to ESP32
cargo run

# Monitor serial output
cargo run -- monitor
```

## 🔧 Key Dependencies

### **Core Framework**
- **`esp-hal`** - ESP32 Hardware Abstraction Layer
- **`embassy-executor`** - Async runtime for embedded systems
- **`log`** - Logging facade with ESP32 implementation

### **Sensor Support**
- **`bme280`** - Temperature, humidity, and pressure sensor
- **`bh1750`** - Light intensity sensor
- **`vl53l0x`** - Time-of-flight distance sensor
- **`ds323x`** - Real-time clock

### **Data Handling**
- **`serde`** - Serialization/deserialization
- **`serde-json-core`** - JSON support for no_std

## 🏛️ Architecture Principles

### **1. Separation of Concerns**
Each folder has a specific responsibility:
- **Abstractions** define interfaces
- **Implementations** provide concrete behavior
- **Services** orchestrate business logic
- **DTOs** handle data transformation

### **2. Dependency Inversion**
High-level modules don't depend on low-level modules:
- Abstractions define contracts
- Implementations fulfill contracts
- Dependencies flow toward abstractions

### **3. Single Responsibility**
Each module has one reason to change:
- Sensor modules handle only sensor operations
- Service modules handle only business logic
- Factory modules handle only object creation

### **4. Open/Closed Principle**
Open for extension, closed for modification:
- New sensors can be added without changing existing code
- New services can be implemented without modifying interfaces
- Configuration can be extended without code changes

## 🔄 Development Workflow

### **1. Adding New Sensors**
```rust
// 1. Define sensor trait in abstractions/
pub trait ISensor<T> {
    fn read_sync(&self) -> Result<T, Box<dyn Error + Send + Sync>>;
}

// 2. Implement sensor in sensors/
impl ISensor<SensorData> for NewSensor {
    fn read_sync(&self) -> Result<SensorData, Box<dyn Error + Send + Sync>> {
        // Implementation
    }
}

// 3. Add to factory in factories/
// 4. Update configuration in configs/
```

### **2. Adding New Services**
```rust
// 1. Define service trait in abstractions/
pub trait IService<T> {
    fn run(&self) -> Result<BaseResponseDTO, Box<dyn Error + Send + Sync>>;
}

// 2. Implement service in services/
impl IService<ServiceData> for NewService {
    fn run(&self) -> Result<BaseResponseDTO, Box<dyn Error + Send + Sync>> {
        // Implementation
    }
}
```

## 🧪 Testing Strategy

### **Unit Testing**
- Test individual modules in isolation
- Mock dependencies for predictable behavior
- Test error conditions and edge cases

### **Integration Testing**
- Test module interactions
- Verify data flow between layers
- Test configuration loading and validation

### **Hardware Testing**
- Test on actual ESP32 hardware
- Verify sensor readings and accuracy
- Test communication protocols

## 📊 Logging & Debugging

### **Log Levels**
```rust
use log::{trace, debug, info, warn, error};

trace!("Very detailed debugging info");
debug!("Debug information");
info!("General information");
warn!("Warning messages");
error!("Error messages");
```

### **Environment Control**
```bash
# Set log level
export RUST_LOG=debug

# Run with specific log level
RUST_LOG=info cargo run
```

## 🚨 Common Issues & Solutions

### **Compilation Issues**
- **Missing target**: `rustup target add xtensa-esp32-none-elf`
- **Toolchain issues**: Ensure ESP-IDF is properly installed
- **Path length**: Move project to shorter path on Windows

### **Runtime Issues**
- **Memory allocation**: Check heap size in `esp_alloc::heap_allocator!`
- **Sensor communication**: Verify I2C configuration and connections
- **Logging**: Ensure `RUST_LOG` environment variable is set

## 🤝 Contributing

### **Code Style**
- Follow Rust formatting guidelines (`cargo fmt`)
- Use meaningful variable and function names
- Add comprehensive documentation
- Include error handling for all operations

### **Commit Guidelines**
- Use conventional commit format
- Reference issue numbers when applicable
- Keep commits focused and atomic
- Test before committing

## 📚 Additional Resources

### **ESP32 Development**
- [ESP-HAL Documentation](https://docs.rs/esp-hal/)
- [ESP-IDF Programming Guide](https://docs.espressif.com/projects/esp-idf/)
- [Rust Embedded Book](https://rust-embedded.github.io/book/)

### **Architecture Patterns**
- [Domain-Driven Design](https://martinfowler.com/bliki/DomainDrivenDesign.html)
- [Clean Architecture](https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html)
- [SOLID Principles](https://en.wikipedia.org/wiki/SOLID)

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🆘 Support

For questions, issues, or contributions:
1. Check existing issues and documentation
2. Create a new issue with detailed description
3. Provide reproduction steps and error messages
4. Include hardware configuration and environment details

---

**Happy Coding! 🎉**

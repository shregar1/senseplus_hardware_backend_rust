# Constants Layer

## 🎯 Purpose & Significance

The **Constants** folder contains **application constants** and **magic numbers** that define fixed values used throughout your system. Constants are essential for:

- **Maintainability** - Centralized location for all fixed values
- **Readability** - Meaningful names instead of magic numbers
- **Consistency** - Same values used across the entire system
- **Configuration** - Easy to modify system behavior in one place

## 🏗️ Architecture Role

### **Configuration Management**
This folder represents the **"Constants Layer"** in software architecture:
- **System Configuration**: Hardware-specific constants and limits
- **Business Rules**: Application-specific constants and thresholds
- **Magic Number Elimination**: Replace hardcoded values with named constants
- **Environment Configuration**: Different constants for different environments

### **Firmware Development Benefits**
In embedded systems, constants provide:
- **Hardware Abstraction**: Hide hardware-specific values behind meaningful names
- **Performance Optimization**: Compile-time constants for better performance
- **Memory Management**: Control memory allocation and limits
- **Error Handling**: Define error codes and status values

## 📁 Current Constants Structure

### **`sensor.rs` - Sensor-Related Constants**
**Purpose**: Define constants specific to sensor operations and measurements

**Key Constants**:
```rust
// Sensor communication constants
pub const I2C_DEFAULT_BAUDRATE: u32 = 400_000;  // 400kHz I2C speed
pub const I2C_TIMEOUT_MS: u32 = 1000;           // 1 second timeout
pub const MAX_SENSOR_COUNT: usize = 16;          // Maximum sensors supported

// Sensor measurement constants
pub const TEMPERATURE_MIN: f32 = -40.0;          // Minimum temperature (°C)
pub const TEMPERATURE_MAX: f32 = 85.0;           // Maximum temperature (°C)
pub const HUMIDITY_MIN: f32 = 0.0;               // Minimum humidity (%)
pub const HUMIDITY_MAX: f32 = 100.0;             // Maximum humidity (%)
pub const PRESSURE_MIN: f32 = 300.0;             // Minimum pressure (hPa)
pub const PRESSURE_MAX: f32 = 1100.0;            // Maximum pressure (hPa)

// Sensor status constants
pub const SENSOR_STATUS_OK: &str = "OK";
pub const SENSOR_STATUS_ERROR: &str = "ERROR";
pub const SENSOR_STATUS_OFFLINE: &str = "OFFLINE";
pub const SENSOR_STATUS_CALIBRATING: &str = "CALIBRATING";
```

**Use Cases**:
- Sensor initialization and configuration
- Data validation and range checking
- Error status definitions
- Performance tuning parameters

### **`distance.rs` - Distance Measurement Constants**
**Purpose**: Define constants for distance sensors and measurements

**Key Constants**:
```rust
// Distance measurement constants
pub const DISTANCE_MIN_MM: f32 = 0.0;            // Minimum distance (mm)
pub const DISTANCE_MAX_MM: f32 = 2000.0;         // Maximum distance (mm)
pub const DISTANCE_ERROR_MM: f32 = f32::MAX;     // Error distance value
pub const DISTANCE_OUT_OF_RANGE: f32 = -1.0;     // Out of range indicator

// Distance sensor constants
pub const VL53L0X_ACCURACY_MM: f32 = 3.0;       // ±3mm accuracy
pub const VL53L0X_TIMING_BUDGET_MS: u32 = 33;   // 33ms timing budget
pub const VL53L0X_INTER_MEASUREMENT_MS: u32 = 10; // 10ms between measurements

// Distance status constants
pub const DISTANCE_STATUS_OK: &str = "OK";
pub const DISTANCE_STATUS_OUT_OF_RANGE: &str = "OUT_OF_RANGE";
pub const DISTANCE_STATUS_ERROR: &str = "ERROR";
pub const DISTANCE_STATUS_UNKNOWN: &str = "UNKNOWN";
```

**Use Cases**:
- Distance sensor configuration
- Range validation
- Status code definitions
- Performance parameters

## 🔄 Constants Usage Patterns

### **1. Hardware Configuration**
```rust
// Use constants for hardware setup
let config = I2cConfig::new()
    .baudrate(I2C_DEFAULT_BAUDRATE.into())
    .timeout(I2C_TIMEOUT_MS.into());

// Validate sensor readings against constants
if temperature < TEMPERATURE_MIN || temperature > TEMPERATURE_MAX {
    warn!("Temperature reading out of range: {}°C", temperature);
    return Err("Temperature out of valid range".into());
}
```

### **2. Error Handling**
```rust
// Use constants for error codes
match sensor_status {
    SENSOR_STATUS_OK => Ok(measurement),
    SENSOR_STATUS_ERROR => Err("Sensor error occurred".into()),
    SENSOR_STATUS_OFFLINE => Err("Sensor is offline".into()),
    SENSOR_STATUS_CALIBRATING => Err("Sensor is calibrating".into()),
    _ => Err("Unknown sensor status".into()),
}
```

### **3. Performance Tuning**
```rust
// Use constants for performance parameters
const SENSOR_READ_INTERVAL_MS: u32 = 1000;  // Read every second
const MAX_RETRY_ATTEMPTS: u32 = 3;           // Retry failed operations 3 times
const TIMEOUT_MS: u32 = 5000;                // 5 second timeout

// Apply constants in timing logic
Timer::after(Duration::from_millis(SENSOR_READ_INTERVAL_MS.into())).await;
```

## 🎨 Design Patterns Used

### **1. Configuration Object Pattern**
Constants can be organized into configuration objects:
```rust
pub struct SensorConstants {
    pub i2c_baudrate: u32,
    pub timeout_ms: u32,
    pub max_retries: u32,
    pub temperature_range: (f32, f32),
    pub humidity_range: (f32, f32),
    pub pressure_range: (f32, f32),
}

impl Default for SensorConstants {
    fn default() -> Self {
        Self {
            i2c_baudrate: I2C_DEFAULT_BAUDRATE,
            timeout_ms: I2C_TIMEOUT_MS,
            max_retries: MAX_RETRY_ATTEMPTS,
            temperature_range: (TEMPERATURE_MIN, TEMPERATURE_MAX),
            humidity_range: (HUMIDITY_MIN, HUMIDITY_MAX),
            pressure_range: (PRESSURE_MIN, PRESSURE_MAX),
        }
    }
}
```

### **2. Builder Pattern**
Constants can be used with builders for flexible configuration:
```rust
impl SensorConstants {
    pub fn with_i2c_baudrate(mut self, baudrate: u32) -> Self {
        self.i2c_baudrate = baudrate;
        self
    }
    
    pub fn with_timeout(mut self, timeout_ms: u32) -> Self {
        self.timeout_ms = timeout_ms;
        self
    }
    
    pub fn with_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
        self
    }
}

// Usage
let constants = SensorConstants::default()
    .with_i2c_baudrate(100_000)  // 100kHz for slower devices
    .with_timeout(2000)           // 2 second timeout
    .with_retries(5);             // 5 retry attempts
```

### **3. Environment-Specific Constants**
Constants can vary by environment:
```rust
#[cfg(debug_assertions)]
pub const LOG_LEVEL: &str = "debug";
#[cfg(not(debug_assertions))]
pub const LOG_LEVEL: &str = "info";

#[cfg(feature = "development")]
pub const SENSOR_READ_INTERVAL_MS: u32 = 100;  // Fast reading for development
#[cfg(not(feature = "development"))]
pub const SENSOR_READ_INTERVAL_MS: u32 = 1000; // Normal reading for production
```

## 🚀 Software Development Benefits

### **1. Maintainability**
- **Single Source of Truth**: All constants defined in one place
- **Easy Modification**: Change values without searching through code
- **Consistent Updates**: Update related constants together

### **2. Readability**
- **Self-Documenting Code**: Constants explain what values represent
- **No Magic Numbers**: Clear meaning instead of unexplained values
- **Better Understanding**: Developers understand system behavior

### **3. Testing and Debugging**
- **Easy Testing**: Test different constant values easily
- **Debugging**: Clear constants make debugging easier
- **Configuration Testing**: Test system with different configurations

## 📋 Best Practices

### **1. Naming Conventions**
```rust
// Use UPPER_SNAKE_CASE for constants
pub const MAX_SENSOR_COUNT: usize = 16;
pub const I2C_DEFAULT_BAUDRATE: u32 = 400_000;
pub const SENSOR_STATUS_OK: &str = "OK";

// Use descriptive names that explain the purpose
pub const TEMPERATURE_MIN_CELSIUS: f32 = -40.0;  // Clear unit specification
pub const HUMIDITY_MAX_PERCENT: f32 = 100.0;     // Clear unit specification
pub const PRESSURE_MIN_HPA: f32 = 300.0;         // Clear unit specification

// Group related constants together
pub const SENSOR_TIMEOUTS: (u32, u32, u32) = (
    I2C_TIMEOUT_MS,      // I2C timeout
    SPI_TIMEOUT_MS,       // SPI timeout
    GPIO_TIMEOUT_MS,      // GPIO timeout
);
```

### **2. Documentation**
```rust
/// Maximum number of sensors that can be connected to the system
/// This limit is based on available I2C addresses and memory constraints
pub const MAX_SENSOR_COUNT: usize = 16;

/// Default I2C communication speed in Hz
/// 400kHz provides good balance between speed and reliability
/// Can be reduced for slower devices or longer cable runs
pub const I2C_DEFAULT_BAUDRATE: u32 = 400_000;

/// Temperature range limits for sensor validation
/// Values outside this range indicate sensor malfunction or extreme conditions
pub const TEMPERATURE_RANGE: (f32, f32) = (-40.0, 85.0);
```

### **3. Type Safety**
```rust
// Use appropriate types for constants
pub const MAX_SENSOR_COUNT: usize = 16;           // Count should be usize
pub const I2C_BAUDRATE: u32 = 400_000;           // Frequency should be u32
pub const TIMEOUT_MS: u32 = 1000;                 // Time should be u32
pub const TEMPERATURE_THRESHOLD: f32 = 25.0;      // Temperature should be f32

// Use enums for related constants
#[derive(Debug, Clone, PartialEq)]
pub enum SensorStatus {
    Ok,
    Error,
    Offline,
    Calibrating,
}

impl SensorStatus {
    pub const OK: &'static str = "OK";
    pub const ERROR: &'static str = "ERROR";
    pub const OFFLINE: &'static str = "OFFLINE";
    pub const CALIBRATING: &'static str = "CALIBRATING";
}
```

## 🔮 Future Extensions

### **Potential New Constants**
- **`network.rs`** - Network configuration constants
- **`storage.rs`** - Storage and memory constants
- **`security.rs`** - Security and encryption constants
- **`diagnostics.rs`** - Diagnostic and monitoring constants

### **Advanced Features**
- **Runtime Configuration**: Load constants from configuration files
- **Dynamic Constants**: Constants that can change at runtime
- **Validation**: Validate constant values at compile time
- **Metrics**: Track constant usage and performance impact

### **Integration Patterns**
- **Feature Flags**: Constants for feature toggles
- **Environment Variables**: Constants from environment configuration
- **Remote Configuration**: Constants loaded from remote services
- **Hot Reloading**: Update constants without restart

## 🧪 Testing Strategies

### **1. Constant Validation**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_constant_ranges() {
        // Test that temperature range is valid
        assert!(TEMPERATURE_MIN < TEMPERATURE_MAX);
        assert!(TEMPERATURE_MIN >= -273.15); // Absolute zero
        
        // Test that humidity range is valid
        assert!(HUMIDITY_MIN >= 0.0);
        assert!(HUMIDITY_MAX <= 100.0);
        
        // Test that pressure range is reasonable
        assert!(PRESSURE_MIN > 0.0);
        assert!(PRESSURE_MAX < 2000.0); // Reasonable atmospheric pressure
    }
    
    #[test]
    fn test_constant_consistency() {
        // Test that timeout values are reasonable
        assert!(I2C_TIMEOUT_MS > 0);
        assert!(I2C_TIMEOUT_MS < 10000); // Not too long
        
        // Test that baudrate is reasonable
        assert!(I2C_DEFAULT_BAUDRATE > 0);
        assert!(I2C_DEFAULT_BAUDRATE <= 1_000_000); // Standard I2C limit
    }
}
```

### **2. Configuration Testing**
```rust
#[test]
fn test_sensor_constants_configuration() {
    let constants = SensorConstants::default();
    
    // Test default values
    assert_eq!(constants.i2c_baudrate, I2C_DEFAULT_BAUDRATE);
    assert_eq!(constants.timeout_ms, I2C_TIMEOUT_MS);
    assert_eq!(constants.max_retries, MAX_RETRY_ATTEMPTS);
    
    // Test builder pattern
    let custom_constants = SensorConstants::default()
        .with_i2c_baudrate(100_000)
        .with_timeout(2000);
    
    assert_eq!(custom_constants.i2c_baudrate, 100_000);
    assert_eq!(custom_constants.timeout_ms, 2000);
}
```

### **3. Integration Testing**
```rust
#[test]
fn test_constants_in_sensor_operations() {
    let sensor = BME280Sensor::new(...).unwrap();
    let measurement = sensor.read_sync().unwrap();
    
    // Test that measurements are within constant-defined ranges
    assert!(measurement.temperature >= TEMPERATURE_MIN);
    assert!(measurement.temperature <= TEMPERATURE_MAX);
    assert!(measurement.humidity >= HUMIDITY_MIN);
    assert!(measurement.humidity <= HUMIDITY_MAX);
}
```

---

**Remember**: Constants are the configuration backbone of your system. Well-organized constants make your code more maintainable, readable, and configurable while eliminating magic numbers and improving system reliability.

# Sensors Layer

## 🎯 Purpose & Significance

The **Sensors** folder contains **concrete implementations** of sensor drivers and measurement logic. This is where the rubber meets the road in firmware development - where abstract interfaces become real hardware interactions. Each sensor module:

- **Implements** the `ISensor<T>` trait from abstractions
- **Handles** low-level hardware communication (I2C, SPI, GPIO)
- **Processes** raw sensor data into meaningful measurements
- **Manages** sensor-specific configuration and calibration

## 🏗️ Architecture Role

### **Implementation Layer**
This folder represents the **"Implementation Layer"** in Clean Architecture:
- **Concrete sensor drivers** that fulfill abstract contracts
- **Hardware-specific code** that interfaces with real sensors
- **Domain logic** for sensor data processing and interpretation

### **Hardware Abstraction**
In embedded systems, sensors provide:
- **Environmental data** (temperature, humidity, pressure)
- **Physical measurements** (distance, light, motion)
- **Time information** (real-time clock, timestamps)
- **Status indicators** (sensor health, error conditions)

## 📁 Current Sensor Implementations

### **`bme280.rs` - Environmental Sensor**
**Hardware**: Bosch BME280 temperature, humidity, and pressure sensor
**Interface**: I2C communication
**Measurements**:
- Temperature (°C)
- Humidity (%)
- Pressure (hPa)

**Key Features**:
- **I2C Configuration**: 400kHz communication speed
- **Error Handling**: Graceful fallback for failed readings
- **Data Validation**: Ensures measurements are within reasonable ranges

### **`bh1750.rs` - Light Intensity Sensor**
**Hardware**: Rohm BH1750 digital light sensor
**Interface**: I2C communication
**Measurements**:
- Light intensity (lux)
- Light condition classification (VERY_DARK to EXTREME)

**Key Features**:
- **High Resolution**: 1 lux resolution
- **Condition Classification**: Automatic light level categorization
- **Configurable Resolution**: Adjustable measurement precision

### **`vl53l0x.rs` - Time-of-Flight Distance Sensor**
**Hardware**: STMicroelectronics VL53L0X
**Interface**: I2C communication
**Measurements**:
- Distance (mm)
- Measurement status (OK, OUT_OF_RANGE, ERROR)

**Key Features**:
- **High Accuracy**: ±3% accuracy
- **Long Range**: Up to 2 meters
- **Status Monitoring**: Tracks measurement quality

### **`ds323x.rs` - Real-Time Clock**
**Hardware**: Maxim Integrated DS3231/DS3232
**Interface**: I2C communication
**Measurements**:
- Date and time (ISO 8601 format)
- Temperature compensation

**Key Features**:
- **High Precision**: ±2ppm accuracy
- **Temperature Compensation**: Automatic drift correction
- **Battery Backup**: Continues operation during power loss

### **`lsm303dlhc/` - Motion Sensor**
**Hardware**: STMicroelectronics LSM303DLHC
**Interface**: I2C communication
**Measurements**:
- **Accelerometer**: 3-axis acceleration (g)
- **Magnetometer**: 3-axis magnetic field (gauss)

**Key Features**:
- **Dual Functionality**: Accelerometer + magnetometer
- **Configurable Ranges**: Adjustable sensitivity
- **Motion Detection**: Built-in motion detection algorithms

## 🔧 Hardware Communication

### **I2C Configuration**
All sensors use I2C for communication with consistent configuration:
```rust
let config = I2cConfig::new().baudrate(400u32.kHz().into());
let i2c = I2cDriver::new(peripherals.i2c0, sda, scl, &config)?;
```

**Benefits**:
- **Standardized**: Same configuration across all sensors
- **Optimized**: 400kHz provides good speed vs. reliability balance
- **Compatible**: Works with most I2C sensors

### **Error Handling Strategy**
```rust
fn _read(&self) -> Result<SensorData, Box<dyn Error + Send + Sync>> {
    match self.sensor.measure() {
        Ok(measurements) => {
            Ok(SensorData {
                // Process successful measurements
            })
        },
        Err(_e) => {
            // Return safe default values
            Ok(SensorData {
                temperature: 0.0,
                humidity: 0.0,
                pressure: 0.0
            })
        }
    }
}
```

**Benefits**:
- **Graceful Degradation**: System continues operating even with sensor failures
- **Predictable Behavior**: Always returns valid data structures
- **Error Logging**: Failed measurements can be logged for debugging

## 🎨 Design Patterns Used

### **1. Template Method Pattern**
Each sensor follows the same structure:
```rust
impl ISensor<SensorData> for MySensor {
    // Common identification methods
    fn urn(&self) -> String { self.urn.clone() }
    fn device_urn(&self) -> String { self.device_urn.clone() }
    fn location_urn(&self) -> String { self.location_urn.clone() }
    fn name(&self) -> String { self.name.clone() }
    
    // Specific measurement method
    fn read_sync(&self) -> Result<SensorData, Box<dyn Error + Send + Sync>> {
        self._read() // Delegates to private implementation
    }
}
```

### **2. Builder Pattern**
Sensors are constructed with clear, step-by-step initialization:
```rust
impl MySensor {
    pub fn new(urn: String, device_urn: String, location_urn: String, name: String) -> Result<Self, Box<dyn Error + Send + Sync>> {
        // 1. Get peripherals
        let peripherals = Peripherals::take().ok_or("Failed to get peripherals")?;
        
        // 2. Configure I2C
        let config = I2cConfig::new().baudrate(400u32.kHz().into());
        
        // 3. Initialize sensor
        let sensor = Sensor::new(i2c)?;
        
        // 4. Return configured sensor
        Ok(Self { urn, device_urn, location_urn, name, sensor })
    }
}
```

### **3. Strategy Pattern**
Different sensors can be swapped at runtime:
```rust
let sensor: Box<dyn ISensor<Box<dyn Error + Send + Sync>>> = match sensor_type {
    "bme280" => Box::new(BME280Sensor::new(...)?),
    "bh1750" => Box::new(BH1750Sensor::new(...)?),
    "vl53l0x" => Box::new(VL53L0XSensor::new(...)?),
    _ => return Err("Unknown sensor type".into()),
};
```

## 🚀 Firmware Development Benefits

### **1. Hardware Independence**
- **Sensor Swapping**: Easy to replace sensors with different models
- **Driver Reuse**: Same driver works with multiple sensor instances
- **Testing**: Test sensor logic without real hardware

### **2. Maintainability**
- **Clear Structure**: Each sensor follows the same pattern
- **Easy Debugging**: Isolated sensor logic for troubleshooting
- **Consistent Error Handling**: Same error handling across all sensors

### **3. Extensibility**
- **New Sensors**: Add new sensors by following established patterns
- **New Measurements**: Extend existing sensors with additional data
- **New Communication**: Support different protocols (SPI, UART)

## 📋 Best Practices

### **1. Sensor Initialization**
```rust
// Always check for errors during initialization
let sensor = Sensor::new(i2c)?;

// Validate sensor is working
sensor.init()?;

// Test basic functionality
let test_reading = sensor.measure()?;
```

### **2. Data Validation**
```rust
// Check for reasonable values
if temperature < -40.0 || temperature > 85.0 {
    warn!("Temperature reading out of range: {}°C", temperature);
    // Handle out-of-range values
}

// Use appropriate data types
let distance_mm: f32 = distance_raw as f32; // Explicit conversion
```

### **3. Error Handling**
```rust
// Provide meaningful error context
.map_err(|e| format!("Failed to read sensor {}: {}", self.name, e))?;

// Log errors for debugging
if let Err(e) = self.sensor.measure() {
    error!("Sensor {} measurement failed: {}", self.name, e);
    // Return safe default or propagate error
}
```

## 🔮 Future Extensions

### **Potential New Sensors**
- **`dht22.rs`** - Digital temperature and humidity sensor
- **`mpu6050.rs`** - 6-axis motion sensor
- **`max30100.rs`** - Heart rate and blood oxygen sensor
- **`mq135.rs`** - Air quality sensor

### **Advanced Features**
- **Sensor Fusion**: Combine multiple sensors for better accuracy
- **Calibration**: Automatic sensor calibration and compensation
- **Power Management**: Dynamic power control for battery optimization
- **Data Logging**: Local storage of sensor readings

### **Communication Protocols**
- **SPI Support**: For high-speed sensors
- **UART Support**: For sensors with serial interfaces
- **CAN Bus**: For automotive applications
- **LoRa**: For long-range wireless sensors

## 🧪 Testing Strategies

### **1. Unit Testing**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sensor_creation() {
        let sensor = BME280Sensor::new(
            "test:urn".to_string(),
            "test:device".to_string(),
            "test:location".to_string(),
            "TestSensor".to_string(),
        );
        assert!(sensor.is_ok());
    }
    
    #[test]
    fn test_sensor_identification() {
        let sensor = BME280Sensor::new(...).unwrap();
        assert_eq!(sensor.name(), "TestSensor");
        assert_eq!(sensor.urn(), "test:urn");
    }
}
```

### **2. Integration Testing**
```rust
#[test]
fn test_sensor_factory_integration() {
    let factory = SensorFactory::new(...);
    let sensor = factory.get("bme280").unwrap();
    
    // Test that factory returns correct sensor type
    assert!(sensor.name().contains("BME280"));
}
```

### **3. Hardware Testing**
- **Bench Testing**: Test sensors in controlled environments
- **Environmental Testing**: Test in various temperature/humidity conditions
- **Stress Testing**: Test sensor limits and failure modes
- **Long-term Testing**: Verify sensor stability over time

---

**Remember**: Sensors are the eyes and ears of your embedded system. Well-designed sensor modules provide reliable data while maintaining clean, maintainable code that's easy to extend and debug.

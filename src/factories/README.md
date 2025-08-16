# Factories Layer

## 🎯 Purpose & Significance

The **Factories** folder contains **object creation** and **dependency injection** components that follow the **Factory Pattern**. Factories are responsible for creating and managing instances of objects, particularly sensors and other system components. This pattern provides:

- **Centralized object creation** with consistent configuration
- **Dependency injection** for flexible component assembly
- **Object lifecycle management** and resource allocation
- **Configuration-driven instantiation** for different environments

## 🏗️ Architecture Role

### **Factory Pattern Implementation**
This folder represents the **"Factory Layer"** in software architecture:
- **Object Creation**: Centralized creation of complex objects
- **Dependency Management**: Handles object dependencies and relationships
- **Configuration Management**: Applies configuration during object creation
- **Resource Pooling**: Manages shared resources and connections

### **Dependency Injection Container**
In modern architectures, factories act as:
- **Service Locators**: Find and provide required dependencies
- **Configuration Managers**: Apply runtime configuration to objects
- **Lifecycle Controllers**: Manage object creation, initialization, and cleanup

## 📁 Current Factory Implementations

### **`sensor.rs` - Sensor Factory**
**Purpose**: Creates and manages sensor instances based on configuration
**Features**:
- **Dynamic Sensor Creation**: Creates sensors based on configuration keys
- **Dependency Injection**: Provides sensors with required dependencies
- **Error Handling**: Gracefully handles sensor creation failures
- **Resource Management**: Manages sensor instances and connections

**Key Implementation**:
```rust
pub struct SensorFactory {
    pub urn: String,
    pub device_urn: String,
    pub location_urn: String,
    pub store: BTreeMap<String, Box<dyn ISensor<Box<dyn Error + Send + Sync>> + Send + Sync>>,
}

impl IFactory<Box<dyn ISensor<Box<dyn Error + Send + Sync>> + Send + Sync>> for SensorFactory {
    fn get(&self, key: String) -> Result<Box<dyn ISensor<Box<dyn Error + Send + Sync>> + Send + Sync>, Box<dyn Error + Send + Sync>> {
        self.store.get(&key)
            .cloned()
            .ok_or_else(|| Box::new(core::io::Error::new(
                core::io::ErrorKind::NotFound,
                format!("Sensor not found for key: {}", key)
            )))
    }
}
```

**Use Cases**:
- Creating sensors based on configuration files
- Managing sensor instances across the application
- Providing sensors to services and other components
- Handling sensor initialization and configuration

## 🔄 Factory Workflow Patterns

### **1. Configuration-Driven Creation**
```rust
// 1. Read configuration
let config = SensorsConfig::new();
let sensor_names = &config.config.include;

// 2. Create factory with configuration
let sensor_factory = SensorFactory::new(
    "urn:esp32:factory:sensors".to_string(),
    "urn:esp32:device:001".to_string(),
    "urn:esp32:location:lab".to_string(),
);

// 3. Create sensors based on configuration
for sensor_name in sensor_names {
    let sensor = sensor_factory.get(sensor_name.clone())?;
    // Use sensor...
}
```

### **2. Dependency Injection Workflow**
```rust
// 1. Create factory with dependencies
let sensor_factory = SensorFactory::new(
    urn,
    device_urn,
    location_urn,
);

// 2. Inject factory into service
let sensing_service = SensingClientService::new(
    service_urn,
    service_device_urn,
    service_location_urn,
    sensor_factory, // Inject the factory
);

// 3. Service uses factory to get sensors
let sensor = self.sensor_factory.get("bme280")?;
let data = sensor.read_sync()?;
```

### **3. Error Handling Workflow**
```rust
// 1. Attempt to get sensor
let sensor_result = sensor_factory.get("bme280");

// 2. Handle different outcomes
match sensor_result {
    Ok(sensor) => {
        info!("Successfully retrieved BME280 sensor");
        let data = sensor.read_sync()?;
        Ok(data)
    },
    Err(e) => {
        error!("Failed to get BME280 sensor: {}", e);
        // Provide fallback or error response
        Err(e)
    }
}
```

## 🎨 Design Patterns Used

### **1. Factory Method Pattern**
Factories create objects without specifying their exact classes:
```rust
impl SensorFactory {
    pub fn new(urn: String, device_urn: String, location_urn: String) -> Self {
        let mut store: BTreeMap<String, Box<dyn ISensor<Box<dyn Error + Send + Sync>> + Send + Sync>> = BTreeMap::new();
        
        // Factory creates different sensor types
        store.insert("bme280".to_string(), Box::new(BME280Sensor::new(...)?));
        store.insert("bh1750".to_string(), Box::new(BH1750Sensor::new(...)?));
        store.insert("vl53l0x".to_string(), Box::new(VL53L0XSensor::new(...)?));
        
        Self { urn, device_urn, location_urn, store }
    }
}
```

### **2. Abstract Factory Pattern**
Factories provide families of related objects:
```rust
// Abstract factory trait
pub trait IFactory<T> {
    fn urn(&self) -> String;
    fn device_urn(&self) -> String;
    fn location_urn(&self) -> String;
    fn get(&self, key: String) -> Result<T, Box<dyn Error + Send + Sync>>;
}

// Concrete factory implementation
impl IFactory<Box<dyn ISensor<Box<dyn Error + Send + Sync>> + Send + Sync>> for SensorFactory {
    // Implementation provides sensor family
}
```

### **3. Builder Pattern**
Factories can use builders for complex object construction:
```rust
impl SensorFactory {
    pub fn with_sensor(mut self, key: String, sensor: Box<dyn ISensor<Box<dyn Error + Send + Sync>> + Send + Sync>) -> Self {
        self.store.insert(key, sensor);
        self
    }
    
    pub fn with_bme280(self) -> Self {
        self.with_sensor("bme280".to_string(), Box::new(BME280Sensor::new(...)?))
    }
    
    pub fn with_bh1750(self) -> Self {
        self.with_sensor("bh1750".to_string(), Box::new(BH1750Sensor::new(...)?))
    }
}

// Usage
let factory = SensorFactory::new(urn, device_urn, location_urn)
    .with_bme280()
    .with_bh1750();
```

## 🚀 Software Development Benefits

### **1. Loose Coupling**
- **Interface-Based**: Components depend on interfaces, not concrete classes
- **Easy Substitution**: Different implementations can be swapped easily
- **Testability**: Easy to mock factories for testing

### **2. Configuration Management**
- **Runtime Configuration**: Objects can be configured at runtime
- **Environment-Specific**: Different configurations for different environments
- **Dynamic Loading**: Load configurations from files or databases

### **3. Resource Management**
- **Connection Pooling**: Manage shared resources like I2C connections
- **Memory Management**: Control object lifecycle and memory usage
- **Error Recovery**: Handle resource failures gracefully

## 📋 Best Practices

### **1. Factory Design**
```rust
// Keep factories focused on single responsibility
pub struct SensorFactory {
    urn: String,
    device_urn: String,
    location_urn: String,
    store: BTreeMap<String, Box<dyn ISensor<Box<dyn Error + Send + Sync>> + Send + Sync>>,
}

// Use meaningful names for factory methods
impl SensorFactory {
    pub fn create_temperature_sensor(&self) -> Result<Box<dyn ISensor<TemperatureData>>, Box<dyn Error + Send + Sync>> {
        // Create temperature sensor
    }
    
    pub fn create_humidity_sensor(&self) -> Result<Box<dyn ISensor<HumidityData>>, Box<dyn Error + Send + Sync>> {
        // Create humidity sensor
    }
}
```

### **2. Error Handling**
```rust
// Provide meaningful error messages
.ok_or_else(|| Box::new(core::io::Error::new(
    core::io::ErrorKind::NotFound,
    format!("Sensor not found for key: {}", key)
)))?

// Log factory operations for debugging
info!("Creating sensor factory with {} sensors", sensor_count);
debug!("Factory URN: {}", self.urn);
```

### **3. Configuration Management**
```rust
// Make factories configurable
impl SensorFactory {
    pub fn from_config(config: &SensorsConfig) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let mut factory = Self::new(
            config.urn.clone(),
            config.device_urn.clone(),
            config.location_urn.clone(),
        );
        
        // Add sensors based on configuration
        for sensor_name in &config.include {
            factory.add_sensor(sensor_name.clone())?;
        }
        
        Ok(factory)
    }
    
    pub fn add_sensor(&mut self, sensor_name: String) -> Result<(), Box<dyn Error + Send + Sync>> {
        let sensor = match sensor_name.as_str() {
            "bme280" => Box::new(BME280Sensor::new(...)?),
            "bh1750" => Box::new(BH1750Sensor::new(...)?),
            _ => return Err(format!("Unknown sensor type: {}", sensor_name).into()),
        };
        
        self.store.insert(sensor_name, sensor);
        Ok(())
    }
}
```

## 🔮 Future Extensions

### **Potential New Factories**
- **`ServiceFactory`** - Creates and manages service instances
- **`PipelineFactory`** - Creates data processing pipelines
- **`NetworkFactory`** - Creates network communication components
- **`StorageFactory`** - Creates data storage components

### **Advanced Features**
- **Object Pooling**: Reuse objects for better performance
- **Lazy Loading**: Create objects only when needed
- **Configuration Validation**: Validate configuration before object creation
- **Health Monitoring**: Monitor factory-created objects for health

### **Integration Patterns**
- **Plugin System**: Load factories from external modules
- **Service Discovery**: Automatically discover available factories
- **Configuration Hot-Reloading**: Update configuration without restart
- **Metrics Collection**: Collect metrics on factory operations

## 🧪 Testing Strategies

### **1. Unit Testing**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_factory_creation() {
        let factory = SensorFactory::new(
            "test:urn".to_string(),
            "test:device".to_string(),
            "test:location".to_string(),
        );
        
        assert_eq!(factory.urn(), "test:urn");
        assert_eq!(factory.device_urn(), "test:device");
    }
    
    #[test]
    fn test_sensor_retrieval() {
        let factory = SensorFactory::new(...);
        let sensor = factory.get("bme280");
        assert!(sensor.is_ok());
    }
    
    #[test]
    fn test_unknown_sensor() {
        let factory = SensorFactory::new(...);
        let sensor = factory.get("unknown_sensor");
        assert!(sensor.is_err());
    }
}
```

### **2. Integration Testing**
```rust
#[test]
fn test_factory_service_integration() {
    // Test factory with real service
    let sensor_factory = SensorFactory::new(...);
    let service = SensingClientService::new(..., sensor_factory);
    
    let result = service.run();
    assert!(result.is_ok());
}
```

### **3. Mock Testing**
```rust
// Create mock factory for testing
struct MockSensorFactory;
impl IFactory<Box<dyn ISensor<Box<dyn Error + Send + Sync>> + Send + Sync>> for MockSensorFactory {
    fn get(&self, key: String) -> Result<Box<dyn ISensor<Box<dyn Error + Send + Sync>> + Send + Sync>, Box<dyn Error + Send + Sync>> {
        // Return mock sensor for testing
        Ok(Box::new(MockSensor::new()))
    }
}
```

---

**Remember**: Factories are the architects of your object creation. They should provide clean, consistent interfaces for creating objects while hiding the complexity of object construction and dependency management.

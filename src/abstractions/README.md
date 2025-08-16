# Abstractions Layer

## 🎯 Purpose & Significance

The **Abstractions** folder contains **interface definitions** and **contracts** that define the behavior of your system without specifying how that behavior is implemented. This is a fundamental concept in software architecture that enables:

- **Loose coupling** between modules
- **Easy testing** through mocking
- **Flexible implementations** that can be swapped
- **Clear contracts** for what each component must provide

## 🏗️ Architecture Role

### **Clean Architecture Principle**
This folder represents the **"Abstraction Layer"** in Clean Architecture, where:
- **High-level modules** define what they need
- **Low-level modules** implement those needs
- **Dependencies flow toward abstractions** (Dependency Inversion Principle)

### **Domain-Driven Design (DDD)**
In DDD, abstractions represent:
- **Domain interfaces** that define business rules
- **Repository patterns** for data access
- **Service contracts** for business operations
- **Factory interfaces** for object creation

## 📁 Current Abstractions

### **`sensor.rs` - Sensor Interface**
```rust
pub trait ISensor<T> {
    fn urn(&self) -> String;
    fn device_urn(&self) -> String;
    fn location_urn(&self) -> String;
    fn name(&self) -> String;
    fn read_sync(&self) -> Result<T, Box<dyn Error + Send + Sync>>;
}
```

**Purpose**: Defines what any sensor must be able to do:
- **Identification**: URN, device, location, and name
- **Operation**: Synchronous reading of sensor data
- **Error Handling**: Proper error propagation

### **`factory.rs` - Factory Interface**
```rust
pub trait IFactory<T> {
    fn urn(&self) -> String;
    fn device_urn(&self) -> String;
    fn location_urn(&self) -> String;
    fn get(&self, key: String) -> Result<T, Box<dyn Error + Send + Sync>>;
}
```

**Purpose**: Defines object creation and retrieval:
- **Object Management**: Creating and retrieving objects by key
- **Dependency Injection**: Providing dependencies when needed
- **Lifecycle Management**: Controlling object instantiation

### **`service.rs` - Service Interface**
```rust
pub trait IService<T> {
    fn urn(&self) -> String;
    fn device_urn(&self) -> String;
    fn location_urn(&self) -> String;
    fn run(&self) -> Result<BaseResponseDTO, Box<dyn Error + Send + Sync>>;
}
```

**Purpose**: Defines business logic operations:
- **Service Execution**: Running business operations
- **Response Handling**: Returning structured responses
- **Error Management**: Proper error handling and reporting

### **`pipeline.rs` - Pipeline Interface**
```rust
pub trait IPipeline<T> {
    fn urn(&self) -> String;
    fn device_urn(&self) -> String;
    fn location_urn(&self) -> String;
    fn run(&self, sensor: &dyn ISensor<T>) -> Result<BTreeMap<String, Value>, Box<dyn Error + Send + Sync>>;
}
```

**Purpose**: Defines data processing workflows:
- **Data Transformation**: Processing sensor data through pipelines
- **Workflow Management**: Orchestrating multiple processing steps
- **Result Aggregation**: Combining results from multiple sources

## 🔄 How to Use Abstractions

### **1. Implementing an Interface**
```rust
// In your implementation file
use crate::abstractions::sensor::ISensor;

pub struct MySensor {
    // ... fields
}

impl ISensor<MySensorData> for MySensor {
    fn urn(&self) -> String {
        "urn:my:sensor:001".to_string()
    }
    
    fn read_sync(&self) -> Result<MySensorData, Box<dyn Error + Send + Sync>> {
        // Your implementation here
    }
    
    // ... other required methods
}
```

### **2. Using Abstractions in Code**
```rust
// Accept any sensor that implements ISensor
fn process_sensor<T>(sensor: &dyn ISensor<T>) -> Result<(), Box<dyn Error + Send + Sync>> {
    let data = sensor.read_sync()?;
    // Process data...
    Ok(())
}

// Can work with any sensor type
let bme280_sensor: Box<dyn ISensor<BME280Data>> = /* ... */;
let vl53l0x_sensor: Box<dyn ISensor<VL53L0XData>> = /* ... */;

process_sensor(&*bme280_sensor)?;
process_sensor(&*vl53l0x_sensor)?;
```

### **3. Testing with Abstractions**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    // Mock sensor for testing
    struct MockSensor;
    
    impl ISensor<TestData> for MockSensor {
        fn urn(&self) -> String { "test:urn".to_string() }
        fn device_urn(&self) -> String { "test:device".to_string() }
        fn location_urn(&self) -> String { "test:location".to_string() }
        fn name(&self) -> String { "MockSensor".to_string() }
        fn read_sync(&self) -> Result<TestData, Box<dyn Error + Send + Sync>> {
            Ok(TestData { value: 42.0 })
        }
    }
    
    #[test]
    fn test_sensor_processing() {
        let mock_sensor = MockSensor;
        let result = process_sensor(&mock_sensor);
        assert!(result.is_ok());
    }
}
```

## 🎨 Design Patterns Used

### **1. Strategy Pattern**
Different sensors implement the same interface, allowing runtime selection:
```rust
let sensor: Box<dyn ISensor<SensorData>> = match sensor_type {
    "bme280" => Box::new(BME280Sensor::new()?),
    "vl53l0x" => Box::new(VL53L0XSensor::new()?),
    _ => return Err("Unknown sensor type".into()),
};
```

### **2. Factory Pattern**
Factories create objects without specifying their exact classes:
```rust
let sensor = sensor_factory.get("bme280")?;
```

### **3. Template Method Pattern**
Services define the structure, implementations provide the details:
```rust
// Service defines the flow
fn run(&self) -> Result<BaseResponseDTO, Box<dyn Error + Send + Sync>> {
    self.validate_input()?;
    let result = self.process()?;
    self.format_response(result)
}

// Implementation provides specific behavior
fn process(&self) -> Result<ProcessedData, Box<dyn Error + Send + Sync>> {
    // Specific implementation
}
```

## 🚀 Benefits in Firmware Development

### **1. Hardware Abstraction**
- **Sensor Independence**: Same code works with different sensor types
- **Driver Flexibility**: Easy to swap sensor drivers
- **Testing**: Test logic without real hardware

### **2. Maintainability**
- **Clear Contracts**: Know exactly what each component must provide
- **Easy Refactoring**: Change implementations without affecting interfaces
- **Documentation**: Interfaces serve as living documentation

### **3. Extensibility**
- **New Sensors**: Add new sensors by implementing existing interfaces
- **New Services**: Create new services following established patterns
- **New Pipelines**: Build new data processing workflows

## 📋 Best Practices

### **1. Interface Design**
- **Keep interfaces focused**: One clear responsibility per interface
- **Use meaningful names**: Names should clearly indicate purpose
- **Document behavior**: Include clear documentation for each method

### **2. Error Handling**
- **Consistent error types**: Use `Box<dyn Error + Send + Sync>`
- **Meaningful errors**: Provide context in error messages
- **Error propagation**: Use `?` operator for clean error handling

### **3. Testing**
- **Mock implementations**: Create test doubles for all abstractions
- **Interface testing**: Test that implementations meet contracts
- **Integration testing**: Test interactions between abstractions

## 🔮 Future Extensions

### **Potential New Abstractions**
- **`INetwork`** - Network communication interface
- **`IStorage`** - Data persistence interface
- **`IScheduler`** - Task scheduling interface
- **`INotification`** - Event notification interface

### **Advanced Patterns**
- **Observer Pattern**: For event-driven architectures
- **Command Pattern**: For undo/redo operations
- **State Pattern**: For state machine implementations

---

**Remember**: Abstractions are the foundation of maintainable, testable, and extensible code. Design them carefully, as they define the contracts that your entire system will follow.

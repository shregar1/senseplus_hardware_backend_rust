# Data Transfer Objects (DTOs) Layer

## 🎯 Purpose & Significance

The **DTOs** folder contains **Data Transfer Objects** that define the structure of data as it moves between different layers of your system. DTOs are crucial for:

- **Data Serialization** - Converting data to/from different formats (JSON, binary, etc.)
- **API Contracts** - Defining the structure of data exchanged with external systems
- **Layer Isolation** - Preventing internal data structures from leaking between layers
- **Version Compatibility** - Managing changes in data structures over time

## 🏗️ Architecture Role

### **Data Layer in Clean Architecture**
This folder represents the **"Data Layer"** in Clean Architecture:
- **Data Contracts**: Define how data is structured and transferred
- **Serialization**: Handle data format conversions
- **Validation**: Ensure data integrity across boundaries
- **Transformation**: Convert between different data representations

### **API Design Principles**
In modern API design, DTOs represent:
- **Request/Response Models**: Define API input and output structures
- **Data Validation**: Ensure data meets business rules
- **Documentation**: Serve as living documentation of data structures
- **Evolution**: Handle backward compatibility and versioning

## 📁 Current DTO Structure

### **`measurement/` - Sensor Measurement Data**
**Purpose**: Define structures for sensor readings and measurements

#### **`base.rs` - Base Measurement Structure**
```rust
#[derive(Default, Debug)]
pub struct BaseMeasurement {
    pub timestamp: String,
    pub sensor_id: String,
    pub location: String,
    pub data_type: String,
}
```

#### **`sensor/` - Sensor-Specific Measurements**
- **`bme280.rs`** - Temperature, humidity, and pressure data
- **`bh1750.rs`** - Light intensity and condition data
- **`vl53l0x.rs`** - Distance and status data
- **`ds323x.rs`** - Date and time data
- **`lsm303dlhc/`** - Motion and magnetic field data

**Example Structure**:
```rust
#[derive(Default, Debug)]
pub struct BME280SensorMeasurement {
    pub temperature: f32,  // Temperature in Celsius
    pub humidity: f32,     // Humidity percentage
    pub pressure: f32,     // Pressure in hPa
}
```

### **`response/` - API Response Data**
**Purpose**: Define structures for system responses and API outputs

#### **`base.rs` - Base Response Structure**
```rust
#[derive(Default, Debug)]
pub struct BaseResponseDTO {
    pub urn: String,           // Unique resource identifier
    pub device_urn: String,    // Device identifier
    pub location_urn: String,  // Location identifier
    pub timestamp: String,     // Response timestamp
    pub data: String,          // Response data (can be JSON string)
}
```

#### **`services/` - Service-Specific Responses**
- **`sensing_client.rs`** - Sensor data collection responses
- **`http_client.rs`** - HTTP communication responses

**Example Structure**:
```rust
#[derive(Debug, Clone)]
pub struct SensingClientServiceResponseDTO {
    pub data: BTreeMap<String, String>,  // Sensor name -> measurement data
}
```

### **`configurations/` - Configuration Data**
**Purpose**: Define structures for system configuration

#### **`sensors.rs` - Sensor Configuration**
```rust
#[derive(Default, Debug)]
pub struct SensorsConfigDTO {
    pub include: Vec<String>,  // List of sensors to include
}
```

## 🔄 Data Flow Patterns

### **1. Sensor Data Flow**
```rust
// 1. Sensor reads data
let measurement = sensor.read_sync()?;

// 2. Data is structured as DTO
let sensor_dto = BME280SensorMeasurement {
    temperature: measurement.temperature,
    humidity: measurement.humidity,
    pressure: measurement.pressure,
};

// 3. DTO is serialized for transmission
let json_data = serde_json_core::to_string(&sensor_dto)?;

// 4. Data is sent via service
let response = service.send_data(json_data)?;
```

### **2. API Response Flow**
```rust
// 1. Service processes request
let sensor_data = self.collect_sensor_data()?;

// 2. Data is structured as response DTO
let response = BaseResponseDTO {
    urn: self.urn.clone(),
    device_urn: self.device_urn.clone(),
    location_urn: self.location_urn.clone(),
    timestamp: get_current_timestamp(),
    data: format!("{:?}", sensor_data),
};

// 3. Response is returned to client
Ok(response)
```

### **3. Configuration Flow**
```rust
// 1. Configuration is loaded
let config = SensorsConfig::new();

// 2. Configuration DTO is used to create components
for sensor_name in &config.config.include {
    let sensor = sensor_factory.get(sensor_name.clone())?;
    // Use sensor...
}
```

## 🎨 Design Patterns Used

### **1. Data Transfer Object Pattern**
DTOs encapsulate data for transfer between layers:
```rust
// Internal sensor data
struct InternalSensorData {
    raw_value: u16,
    calibration_offset: f32,
    timestamp: u64,
}

// DTO for external communication
#[derive(Serialize, Deserialize)]
pub struct SensorMeasurementDTO {
    pub value: f32,
    pub unit: String,
    pub timestamp: String,
}

// Conversion method
impl From<InternalSensorData> for SensorMeasurementDTO {
    fn from(internal: InternalSensorData) -> Self {
        Self {
            value: (internal.raw_value as f32) + internal.calibration_offset,
            unit: "units".to_string(),
            timestamp: format_timestamp(internal.timestamp),
        }
    }
}
```

### **2. Builder Pattern**
DTOs can use builders for complex construction:
```rust
impl BaseResponseDTO {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn with_urn(mut self, urn: String) -> Self {
        self.urn = urn;
        self
    }
    
    pub fn with_device_urn(mut self, device_urn: String) -> Self {
        self.device_urn = device_urn;
        self
    }
    
    pub fn with_location_urn(mut self, location_urn: String) -> Self {
        self.location_urn = location_urn;
        self
    }
    
    pub fn with_timestamp(mut self, timestamp: String) -> Self {
        self.timestamp = timestamp;
        self
    }
    
    pub fn with_data(mut self, data: String) -> Self {
        self.data = data;
        self
    }
}

// Usage
let response = BaseResponseDTO::new()
    .with_urn("urn:esp32:response:001".to_string())
    .with_device_urn("urn:esp32:device:001".to_string())
    .with_location_urn("urn:esp32:location:lab".to_string())
    .with_timestamp("2024-01-01T00:00:00Z".to_string())
    .with_data("Sensor data here".to_string());
```

### **3. Factory Pattern**
DTOs can be created by factories:
```rust
pub struct DTOFactory;

impl DTOFactory {
    pub fn create_sensor_measurement(
        sensor_type: &str,
        data: &str,
    ) -> Result<Box<dyn MeasurementDTO>, Box<dyn Error + Send + Sync>> {
        match sensor_type {
            "bme280" => Ok(Box::new(BME280SensorMeasurement::from_str(data)?)),
            "bh1750" => Ok(Box::new(BH1750SensorMeasurement::from_str(data)?)),
            _ => Err("Unknown sensor type".into()),
        }
    }
}
```

## 🚀 Software Development Benefits

### **1. Data Consistency**
- **Structured Data**: Consistent data formats across the system
- **Type Safety**: Compile-time checking of data structures
- **Validation**: Built-in data validation and error checking

### **2. API Evolution**
- **Versioning**: Handle multiple API versions gracefully
- **Backward Compatibility**: Maintain compatibility with older clients
- **Documentation**: Self-documenting data structures

### **3. Testing and Debugging**
- **Mock Data**: Easy to create test data with DTOs
- **Serialization Testing**: Test data conversion and validation
- **API Testing**: Test API contracts with structured data

## 📋 Best Practices

### **1. DTO Design**
```rust
// Keep DTOs focused and simple
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorMeasurementDTO {
    pub sensor_id: String,     // Required identifier
    pub value: f32,            // Required measurement value
    pub unit: String,          // Required unit
    pub timestamp: String,     // Required timestamp
    pub quality: Option<f32>,  // Optional quality indicator
}

// Use meaningful field names
pub struct BME280MeasurementDTO {
    pub temperature_celsius: f32,    // Clear unit specification
    pub humidity_percent: f32,       // Clear unit specification
    pub pressure_hpa: f32,           // Clear unit specification
}
```

### **2. Serialization Handling**
```rust
// Handle serialization errors gracefully
impl FromStr for SensorMeasurementDTO {
    type Err = Box<dyn Error + Send + Sync>;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json_core::from_str(s)
            .map_err(|e| format!("Failed to parse sensor measurement: {}", e).into())
    }
}

// Provide both serialization and deserialization
impl SensorMeasurementDTO {
    pub fn to_json(&self) -> Result<String, Box<dyn Error + Send + Sync>> {
        serde_json_core::to_string(self)
            .map_err(|e| format!("Failed to serialize sensor measurement: {}", e).into())
    }
    
    pub fn from_json(json: &str) -> Result<Self, Box<dyn Error + Send + Sync>> {
        serde_json_core::from_str(json)
            .map_err(|e| format!("Failed to deserialize sensor measurement: {}", e).into())
    }
}
```

### **3. Validation and Error Handling**
```rust
// Validate DTO data
impl SensorMeasurementDTO {
    pub fn validate(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        if self.value.is_nan() || self.value.is_infinite() {
            return Err("Invalid measurement value".into());
        }
        
        if self.timestamp.is_empty() {
            return Err("Timestamp is required".into());
        }
        
        Ok(())
    }
}

// Use validation in constructors
impl SensorMeasurementDTO {
    pub fn new(
        sensor_id: String,
        value: f32,
        unit: String,
        timestamp: String,
    ) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let dto = Self {
            sensor_id,
            value,
            unit,
            timestamp,
            quality: None,
        };
        
        dto.validate()?;
        Ok(dto)
    }
}
```

## 🔮 Future Extensions

### **Potential New DTOs**
- **`NetworkDTO`** - Network configuration and status data
- **`StorageDTO`** - Data storage and retrieval structures
- **`DiagnosticDTO`** - System health and diagnostic information
- **`UpdateDTO`** - Firmware and configuration update data

### **Advanced Features**
- **Schema Validation**: JSON schema validation for DTOs
- **Data Transformation**: Automatic data format conversion
- **Caching**: DTO caching for performance optimization
- **Compression**: Data compression for network transmission

### **Integration Patterns**
- **GraphQL**: GraphQL schema definitions
- **Protocol Buffers**: Binary serialization support
- **Message Queues**: DTOs for message queue systems
- **Event Streaming**: DTOs for event-driven architectures

## 🧪 Testing Strategies

### **1. Unit Testing**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_dto_creation() {
        let dto = SensorMeasurementDTO::new(
            "test_sensor".to_string(),
            25.5,
            "celsius".to_string(),
            "2024-01-01T00:00:00Z".to_string(),
        ).unwrap();
        
        assert_eq!(dto.sensor_id, "test_sensor");
        assert_eq!(dto.value, 25.5);
        assert_eq!(dto.unit, "celsius");
    }
    
    #[test]
    fn test_dto_validation() {
        let dto = SensorMeasurementDTO {
            sensor_id: "test".to_string(),
            value: f32::NAN,  // Invalid value
            unit: "celsius".to_string(),
            timestamp: "2024-01-01T00:00:00Z".to_string(),
            quality: None,
        };
        
        assert!(dto.validate().is_err());
    }
}
```

### **2. Serialization Testing**
```rust
#[test]
fn test_dto_serialization() {
    let dto = SensorMeasurementDTO::new(...).unwrap();
    
    // Test serialization
    let json = dto.to_json().unwrap();
    assert!(json.contains("test_sensor"));
    
    // Test deserialization
    let deserialized = SensorMeasurementDTO::from_json(&json).unwrap();
    assert_eq!(dto.sensor_id, deserialized.sensor_id);
}
```

### **3. Integration Testing**
```rust
#[test]
fn test_dto_service_integration() {
    let service = SensingClientService::new(...);
    let response = service.run().unwrap();
    
    // Verify response DTO structure
    assert!(!response.urn.is_empty());
    assert!(!response.device_urn.is_empty());
    assert!(!response.timestamp.is_empty());
}
```

---

**Remember**: DTOs are the contracts of your data layer. They should be well-designed, well-documented, and well-tested to ensure reliable data flow throughout your system.

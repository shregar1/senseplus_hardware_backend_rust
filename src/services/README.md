# Services Layer

## 🎯 Purpose & Significance

The **Services** folder contains **business logic** and **orchestration** components that coordinate between different parts of your system. Services are the "glue" that connects sensors, factories, and external systems. They implement the **Service Layer** pattern, which:

- **Orchestrates** complex operations involving multiple components
- **Implements** business rules and application logic
- **Handles** external communication (HTTP, MQTT, etc.)
- **Manages** data transformation and validation

## 🏗️ Architecture Role

### **Service Layer Pattern**
This folder represents the **"Service Layer"** in Clean Architecture:
- **Business Logic**: Implements application-specific rules and workflows
- **Orchestration**: Coordinates between different system components
- **External Integration**: Handles communication with external systems
- **Data Processing**: Transforms and validates data as it flows through the system

### **Domain-Driven Design (DDD)**
In DDD, services represent:
- **Application Services**: Coordinate use cases and workflows
- **Domain Services**: Implement business logic that doesn't belong to entities
- **Infrastructure Services**: Handle external system interactions

## 📁 Current Service Implementations

### **`http_client.rs` - HTTP Communication Service**
**Purpose**: Handles HTTP requests and responses for external API communication
**Features**:
- **Request Generation**: Creates properly formatted HTTP requests
- **Response Parsing**: Extracts data from HTTP responses
- **Error Handling**: Manages network and parsing errors
- **Flexible Endpoints**: Supports different API endpoints and methods

**Key Methods**:
```rust
impl HttpClientService {
    // Create HTTP GET request
    pub fn create_get_request(&self, endpoint: &str) -> String
    
    // Create HTTP POST request with JSON data
    pub fn create_post_request(&self, endpoint: &str, json_data: &str) -> String
    
    // Parse HTTP response to extract body
    pub fn parse_http_response(&self, response: &[u8]) -> Result<String, Box<dyn Error + Send + Sync>>
}
```

**Use Cases**:
- Sending sensor data to cloud platforms
- Fetching configuration from remote servers
- Reporting device status and health
- Receiving firmware updates

### **`sensing_client.rs` - Sensor Data Service**
**Purpose**: Orchestrates sensor data collection and processing
**Features**:
- **Sensor Coordination**: Manages multiple sensor readings
- **Data Aggregation**: Combines data from different sensors
- **Response Formatting**: Creates structured responses for clients
- **Error Aggregation**: Collects and reports errors from multiple sources

**Key Methods**:
```rust
impl SensingClientService {
    // Collect data from all configured sensors
    pub fn collect_sensor_data(&self) -> Result<BTreeMap<String, String>, Box<dyn Error + Send + Sync>>
    
    // Process sensor readings through pipelines
    pub fn process_sensor_data(&self, sensor_data: BTreeMap<String, String>) -> Result<BaseResponseDTO, Box<dyn Error + Send + Sync>>
}
```

**Use Cases**:
- Environmental monitoring systems
- IoT data collection platforms
- Industrial sensor networks
- Research data collection

### **`rest_client.rs` - REST API Client Service**
**Purpose**: Provides RESTful API client functionality
**Features**:
- **REST Operations**: GET, POST, PUT, DELETE methods
- **Authentication**: Handles API keys and tokens
- **Rate Limiting**: Manages API request frequency
- **Response Caching**: Caches responses for performance

**Use Cases**:
- RESTful API integrations
- Microservice communication
- Third-party service integration
- API testing and validation

## 🔄 Service Workflow Patterns

### **1. Data Collection Workflow**
```rust
// 1. Get sensor factory
let sensor_factory = SensorFactory::new(...);

// 2. Collect data from all sensors
let mut sensor_data = BTreeMap::new();
for sensor_name in &config.include {
    if let Ok(sensor) = sensor_factory.get(sensor_name.clone()) {
        if let Ok(data) = sensor.read_sync() {
            sensor_data.insert(sensor_name.clone(), format!("{:?}", data));
        }
    }
}

// 3. Process and return data
Ok(BaseResponseDTO {
    urn: self.urn.clone(),
    device_urn: self.device_urn.clone(),
    location_urn: self.location_urn.clone(),
    timestamp: get_current_timestamp(),
    data: format!("{:?}", sensor_data),
})
```

### **2. HTTP Communication Workflow**
```rust
// 1. Create HTTP request
let request = self.create_post_request("/api/data", &json_data);

// 2. Send request (in real implementation)
// let response = self.send_request(request).await?;

// 3. Parse response
let parsed_data = self.parse_http_response(&response_bytes)?;

// 4. Return processed data
Ok(BaseResponseDTO {
    // ... response data
})
```

### **3. Error Handling Workflow**
```rust
// 1. Attempt operation
let result = self.perform_operation().await;

// 2. Handle different error types
match result {
    Ok(data) => {
        info!("Operation completed successfully");
        Ok(data)
    },
    Err(e) => {
        error!("Operation failed: {}", e);
        
        // Return graceful error response
        Ok(BaseResponseDTO {
            urn: self.urn.clone(),
            device_urn: self.device_urn.clone(),
            location_urn: self.location_urn.clone(),
            timestamp: get_current_timestamp(),
            data: format!("Error: {}", e),
        })
    }
}
```

## 🎨 Design Patterns Used

### **1. Service Locator Pattern**
Services can locate and use other system components:
```rust
impl SensingClientService {
    pub fn new(
        urn: String,
        device_urn: String,
        location_urn: String,
        sensor_factory: SensorFactory,
        // ... other dependencies
    ) -> Self {
        Self { urn, device_urn, location_urn, sensor_factory }
    }
}
```

### **2. Chain of Responsibility Pattern**
Services can chain operations together:
```rust
// Sensor data flows through multiple processing steps
let raw_data = self.collect_sensor_data()?;
let processed_data = self.process_sensor_data(raw_data)?;
let formatted_response = self.format_response(processed_data)?;
```

### **3. Template Method Pattern**
Services define the structure, implementations provide details:
```rust
impl IService<BaseResponseDTO> for SensingClientService {
    fn run(&self) -> Result<BaseResponseDTO, Box<dyn Error + Send + Sync>> {
        // Template: collect → process → format → return
        let sensor_data = self.collect_sensor_data()?;
        let processed_data = self.process_sensor_data(sensor_data)?;
        Ok(processed_data)
    }
}
```

## 🚀 Firmware Development Benefits

### **1. Business Logic Centralization**
- **Single Source of Truth**: Business rules are defined in one place
- **Easy Modification**: Change business logic without touching sensor code
- **Consistent Behavior**: All operations follow the same patterns

### **2. External System Integration**
- **API Management**: Centralized handling of external API calls
- **Protocol Abstraction**: Hide external system details from sensors
- **Error Handling**: Consistent error handling across external systems

### **3. Testing and Debugging**
- **Isolated Testing**: Test business logic without hardware dependencies
- **Mock Dependencies**: Easy to mock sensors and external systems
- **Clear Workflows**: Easy to trace data flow through the system

## 📋 Best Practices

### **1. Service Design**
```rust
// Keep services focused on single responsibility
pub struct SensingClientService {
    urn: String,
    device_urn: String,
    location_urn: String,
    // Only include what this service needs
}

// Use dependency injection for flexibility
impl SensingClientService {
    pub fn new(
        urn: String,
        device_urn: String,
        location_urn: String,
        sensor_factory: SensorFactory, // Inject dependencies
    ) -> Self {
        Self { urn, device_urn, location_urn, sensor_factory }
    }
}
```

### **2. Error Handling**
```rust
// Provide context for errors
.map_err(|e| format!("Failed to collect sensor data: {}", e))?;

// Log errors for debugging
if let Err(e) = self.perform_operation() {
    error!("Service operation failed: {}", e);
    // Handle error gracefully
}
```

### **3. Configuration Management**
```rust
// Make services configurable
pub struct HttpClientService {
    server_ip: String,
    server_port: u16,
    timeout: Duration,
    retry_count: u32,
}

// Allow runtime configuration
impl HttpClientService {
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
    
    pub fn with_retries(mut self, retry_count: u32) -> Self {
        self.retry_count = retry_count;
        self
    }
}
```

## 🔮 Future Extensions

### **Potential New Services**
- **`mqtt_client.rs`** - MQTT communication service
- **`data_logger.rs`** - Local data logging service
- **`ota_service.rs`** - Over-the-air update service
- **`diagnostic_service.rs`** - System health monitoring service

### **Advanced Features**
- **Service Discovery**: Automatic service registration and discovery
- **Load Balancing**: Distribute work across multiple service instances
- **Circuit Breaker**: Prevent cascading failures in external systems
- **Retry Policies**: Configurable retry strategies for failed operations

### **Integration Patterns**
- **Event-Driven**: Publish/subscribe patterns for loose coupling
- **Message Queues**: Asynchronous message processing
- **API Gateways**: Centralized API management and routing
- **Service Mesh**: Advanced service-to-service communication

## 🧪 Testing Strategies

### **1. Unit Testing**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_service_creation() {
        let service = SensingClientService::new(
            "test:urn".to_string(),
            "test:device".to_string(),
            "test:location".to_string(),
            MockSensorFactory::new(),
        );
        assert!(service.urn() == "test:urn");
    }
    
    #[test]
    fn test_service_operation() {
        let service = SensingClientService::new(...);
        let result = service.run();
        assert!(result.is_ok());
    }
}
```

### **2. Integration Testing**
```rust
#[test]
fn test_service_integration() {
    // Test service with real sensor factory
    let sensor_factory = SensorFactory::new(...);
    let service = SensingClientService::new(..., sensor_factory);
    
    let result = service.run();
    assert!(result.is_ok());
}
```

### **3. Mock Testing**
```rust
// Create mock dependencies for testing
struct MockSensorFactory;
impl IFactory<Box<dyn ISensor<Box<dyn Error + Send + Sync>> + Send + Sync>> for MockSensorFactory {
    fn get(&self, key: String) -> Result<Box<dyn ISensor<Box<dyn Error + Send + Sync>> + Send + Sync>, Box<dyn Error + Send + Sync>> {
        // Return mock sensor for testing
        Ok(Box::new(MockSensor::new()))
    }
}
```

---

**Remember**: Services are the orchestrators of your system. They should focus on business logic and coordination, leaving hardware-specific details to the sensor layer and external communication details to specialized client services.

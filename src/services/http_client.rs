use alloc::string::String;
use alloc::string::ToString;
use core::error::Error;
use alloc::boxed::Box;
use esp_println::println;
use alloc::format;

use crate::abstractions::service::IService;
use crate::dtos::response::base::BaseResponseDTO;

use crate::enums::value::Value;

// Simple HTTP client using Embassy networking
pub struct HttpClientService {
    urn: String,
    device_urn: String,
    location_urn: String,
    server_ip: String,
    //server_port: u16,
}

impl IService<BaseResponseDTO> for HttpClientService {
    fn urn(&self) -> String {
        self.urn.clone()
    }

    fn device_urn(&self) -> String {
        self.device_urn.clone()
    }

    fn location_urn(&self) -> String {
        self.location_urn.clone()
    }

    fn run(&self) -> Result<BaseResponseDTO, core::fmt::Error> {
        // For now, return a placeholder response
        // In a real implementation, this would make an HTTP request
        Ok(BaseResponseDTO {
            status: self.urn.clone(),
            message: "hi".to_string(),
            data: core::prelude::v1::Some(Value::String("hi".to_string())),
        })
    }
}

impl HttpClientService {
    pub fn new(
        urn: String,
        device_urn: String,
        location_urn: String,
        server_ip: String,
        //server_port: u16,
    ) -> Self {
        Self {
            urn,
            device_urn,
            location_urn,
            server_ip,
            //server_port,
        }
    }

    // Method to create HTTP GET request string
    pub fn create_get_request(&self, endpoint: &str) -> String {
        format!(
            "GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
            endpoint, self.server_ip
        )
    }

    // Method to create HTTP POST request string
    pub fn create_post_request(&self, endpoint: &str, json_data: &str) -> String {
        format!(
            "POST {} HTTP/1.1\r\nHost: {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            endpoint, self.server_ip, json_data.len(), json_data
        )
    }

    // Parse HTTP response to extract body
    pub fn parse_http_response(&self, response: &[u8]) -> Result<String, Box<dyn Error + Send + Sync>> {
        // Convert response bytes to string
        let response_str = core::str::from_utf8(response)?;
        
        // Find the HTTP body (after double CRLF)
        if let Some(body_start) = response_str.find("\r\n\r\n") {
            Ok(response_str[body_start + 4..].to_string())
        } else {
            Ok(response_str.to_string())
        }
    }
}

// Example usage function
pub fn example_http_usage() -> Result<(), Box<dyn Error + Send + Sync>> {
    let http_client = HttpClientService::new(
        "urn:esp32:http:client".to_string(),
        "urn:esp32:device:001".to_string(),
        "urn:esp32:location:lab".to_string(),
        "192.168.1.100".to_string(),
        //8080,
    );

    // Create a GET request
    let get_request = http_client.create_get_request("/api/sensors");
    println!("GET Request: {}", get_request);

    // Create a POST request with JSON data
    let json_data = r#"{"temperature": 25.5, "humidity": 60.0}"#;
    let post_request = http_client.create_post_request("/api/data", json_data);
    println!("POST Request: {}", post_request);

    Ok(())
}

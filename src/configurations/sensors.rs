use alloc::string::{String, ToString};
use alloc::{vec::Vec, vec};

pub struct SensorsConfig {
    pub include: Vec<String>,
}

impl SensorsConfig {
    pub fn new() -> Self {
        let include = vec![
            "bme280".to_string(),
            "bh1750".to_string()
        ];
        Self { 
            include: include
        }
    }
}
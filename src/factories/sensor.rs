use alloc::collections::BTreeMap;
use alloc::string::String;
use core::error::Error;

use crate::abstractions::factory::IFactory;
use crate::abstractions::sensor::ISensor;
use crate::constants::sensor::SensorConstants;
use crate::sensors::bh1750::BH1750Sensor;
use crate::sensors::bme280::BME280Sensor;
use crate::sensors::ds323x::DS323XSensor;
use crate::sensors::vl53l0x::VL53L0XSensor;

pub struct SensorFactory {
    pub urn: String,
    pub device_urn: String,
    pub location_urn: String,
    pub store: BTreeMap<String, Box<dyn ISensor<Box<dyn Error + Send + Sync>> + Send + Sync>>,
}

impl IFactory<Box<dyn ISensor<Box<dyn Error + Send + Sync>> + Send + Sync>> for SensorFactory {

    fn urn(&self) -> String {
        self.urn.clone()
    }

    fn device_urn(&self) -> String {
        self.device_urn.clone()
    }

    fn location_urn(&self) -> String {
        self.location_urn.clone()
    }

    fn get(&self, key: String) -> Result<Box<dyn ISensor<Box<dyn Error + Send + Sync>> + Send + Sync>, Box<dyn Error + Send + Sync>> {
        self._get(key)
    }
}

impl SensorFactory {

    pub fn new(
        urn: String,
        device_urn: String,
        location_urn: String,
    ) -> Self {

        let mut store: BTreeMap<String, Box<dyn ISensor<Box<dyn Error + Send + Sync>> + Send + Sync>> = BTreeMap::new();
        
        // Note: These will need to be properly instantiated with actual sensor instances
        // For now, we'll create placeholder entries
        
        Self {
            urn: urn,
            device_urn: device_urn,
            location_urn: location_urn,
            store: store
        }
    }

    fn _get(&self, key: String) -> Result<Box<dyn ISensor<Box<dyn Error + Send + Sync>> + Send + Sync>, Box<dyn Error + Send + Sync>> {
        self.store.get(&key)
            .cloned()
            .ok_or_else(|| Box::new(core::io::Error::new(
                core::io::ErrorKind::NotFound,
                format!("Sensor not found for key: {}", key)
            )))
    }
    
}
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use core::error::Error;

use crate::abstractions::factory::IFactory;
use crate::abstractions::sensor::ISensor;
use crate::constants::sensor::SensorConstant;
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
        
        store.insert(SensorConstant::BME280.to_string(), Box::new(BME280Sensor::new()));
        store.insert(SensorConstant::BH1750.to_string(), Box::new(BH1750Sensor::new()));
        store.insert(SensorConstant::DS3231SN.to_string(), Box::new(DS323XSensor::new()));
        store.insert(SensorConstant::VL5310X.to_string(), Box::new(VL53L0XSensor::new()));
        
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
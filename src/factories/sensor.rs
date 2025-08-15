use std::collections::HashMap;

use crate::abstractions::factory::IFactory;
use crate::abstractions::sensor::ISensor;
use crate::constants::sensor::SensorConstants;
use crate::sensors::bh1750::BH1750Sensor;
use crate::sensors::bme280::BME280Sensor;
use crate::sensors::ds323x::DS323XSensor;
use crate::sensors::vl53l0x::VL53L0XSensor;


pub struct SensorFactory<ISensor> {
    pub urn: String,
    pub device_urn: String,
    pub location_urn: String,
    pub store: HashMap<String, Box<dyn ISensor + Send + Sync>>,
}

impl IFactory for SensorFactory<dyn ISensor> {

    fn urn(&self) -> String {
        &self.urn
    }

    fn device_urn(&self) -> String {
        &self.device_urn
    }

    fn location_urn(&self) -> String {
        &self.location_urn
    }

    async fn get(&self, key: String) -> impl Future<Output = Result<dyn ISensor, Error>> {
        self._get(key).await
    }
}

impl SensorFactory<dyn ISensor> {

    fn config(&self) -> SensorsConfigDTO {
        &self.config
    }

    pub fn new(
        urn: String,
        device_urn: String,
        location_urn: String,
    ) -> Self {

        let mut store: HashMap<String, Box<dyn ISensor<T> + Send + Sync>> = HashMap::new();
        store.insert(
            SensorConstants::BH1750.to_string(), 
            BH1750Sensor
        );
        store.insert(
            SensorConstants::BME280.to_string(),
            BME280Sensor
        );
        //store.insert(SensorConstants::BME680.to_string(), BME680Sensor);
        store.insert(
            SensorConstants::DS3231SN.to_string(),
            DS323XSensor
        );
        //store.insert(SensorConstants::LSM303DLHACCEL.to_string(), LSM303DLHACCELSensor)
        //store.insert(SensorConstants::LSM303DLHMAG.to_string(), LSM303DLHMAGSensor)
        store.insert(
            SensorConstants::VL5310X.to_string(), 
            VL5310XSensor
        );
        Self {
            urn: urn,
            device_urn: device_urn,
            location_urn: location_urn,
            store: store
        }
    }

    pub async fn _get(&self, key: String) -> Result<&Box<dyn ISensor<T> + Send + Sync>, Error>  {
        Ok(self.store.get(key).ok_or_else(|| Err(format!("Sensor not found for key: {}", key))))
    }
    
}
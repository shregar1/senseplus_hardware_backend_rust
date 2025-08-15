use std::collections::HashMap;

use serde_json::{json, Value};

use crate::abstractions::factory::IFactory;
use crate::abstractions::sensor::ISensor;
use crate::abstractions::service::IService;
use crate::dtos::configurations::sensors::SensorsConfigDTO;
use crate::dtos::response::services::sensing_client::SensingClientServiceResponseDTO;
use crate::factories::sensor::SensorFactory;


struct SensingClientService {
    pub urn: String,
    pub device_urn: String,
    pub location_urn: String,
    pub config: SensorsConfigDTO
}

impl IService for SensingClientService  {

    fn urn(&self) -> String {
        &self.urn
    }

    fn device_urn(&self) -> String {
        &self.device_urn
    }

    fn location_urn(&self) -> String {
        &self.location_urn
    }

    async fn run(&self, config: SensorsConfigDTO) -> Result<Value, Error> {
        self._run().await
    }
    
}

impl SensingClientService {

    fn config(&self) -> SensorsConfigDTO {
        &self.config
    }

    fn new(
        urn: String,
        device_urn: String,
        location_urn: String,
        config: SensorsConfigDTO
    ) -> Self {
        Self {
            urn: urn,
            device_urn: device_urn,
            location_urn: location_urn,
            config: config
        }
    }

    async fn _run(&self) -> HashMap<String, Value> {

        let include_sensors: Vec<String> = self.config.include;
        let sensor_factory: SensorFactory<dyn ISensor<_>> = SensorFactory::new(
            self.urn,
            self.device_urn,
            self.location_urn
        );

        let mut data: HashMap<String: Value> = HashMap::new();
        for sensor_key in include_sensors {
            let sensor: dyn ISensor<_> = sensor_factory.get(sensor_key.to_lowercase()).await.unwrap();
            let sensor_measurements = match sensor.read(){
                Ok(data) => {
                    data
                },
                Err(e) => {
                    Err(e)
                }
            }.await;
            data.entry(sensor_key.to_uppercase()).or_insert_with(json!(sensor_measurements))
        }
        Ok(
            SensingClientServiceResponseDTO {
                data: data,
            }
        )
    }
    
}
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

use crate::abstractions::factory::IFactory;
use crate::abstractions::sensor::ISensor;
use crate::abstractions::service::IService;
use crate::dtos::configurations::sensors::SensorsConfigDTO;
use crate::dtos::response::services::sensing_client::SensingClientServiceResponseDTO;
use crate::factories::sensor::SensorFactory;

pub struct SensingClientService {
    pub urn: String,
    pub device_urn: String,
    pub location_urn: String,
    pub config: SensorsConfigDTO
}

impl IService<SensorsConfigDTO> for SensingClientService  {

    fn urn(&self) -> String {
        self.urn.clone()
    }

    fn device_urn(&self) -> String {
        self.device_urn.clone()
    }

    fn location_urn(&self) -> String {
        self.location_urn.clone()
    }

    fn run(&self) -> Result<SensingClientServiceResponseDTO, Box<dyn core::error::Error + Send + Sync>> {
        self._run()
    }
    
}

impl SensingClientService {

    fn config(&self) -> &SensorsConfigDTO {
        &self.config
    }

    pub fn new(
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

    fn _run(&self) -> Result<SensingClientServiceResponseDTO, Box<dyn core::error::Error + Send + Sync>> {

        let include_sensors: Vec<String> = self.config.include.clone();
        let sensor_factory: SensorFactory = SensorFactory::new(
            self.urn.clone(),
            self.device_urn.clone(),
            self.location_urn.clone()
        );

        let mut data: BTreeMap<String, String> = BTreeMap::new();
        for sensor_key in include_sensors {
            let sensor = sensor_factory.get(sensor_key.to_lowercase())?;
            let sensor_measurements = match sensor.read_sync(){
                Ok(data) => {
                    format!("{:?}", data)
                },
                Err(e) => {
                    return Err(e);
                }
            };
            data.insert(sensor_key.to_uppercase(), sensor_measurements);
        }
        Ok(
            SensingClientServiceResponseDTO {
                data: data,
            }
        )
    }
    
}
use bh1750::{BH1750, Resolution};
use esp_hal::{
    clock::ClockControl,
    delay::Delay,
    peripherals::Peripherals,
    prelude::*,
    system::SystemControl,
    gpio::Io
};

use crate::abstractions::sensor::ISensor;

use crate::dtos::measurement;
use crate::dtos::measurement::sensor::bh1750::BH1750SensorMeasurement;

pub struct BH1750Sensor {
    urn: String,
    device_urn: String,
    location_urn: String,
    name: String,
    sensor: BH1750,
}

impl ISensor<BH1750SensorMeasurement> for BH1750Sensor {

    fn urn(&self) -> String {
        &self.urn
    }

    fn device_urn(&self) -> String {
        &self.device_urn
    }

    fn location_urn(&self) -> String {
        &self.location_urn
    }

    fn name(&self) -> String {
        &self.name
    }

    async fn read(&self) -> Result<BH1750SensorMeasurement, Error> {
        self._read().await
    }
}

impl BH1750Sensor {
    fn new(
        urn: String,
        device_urn: String,
        location_urn: String,
        name: String,
    ) -> Self {

        let peripherals = Peripherals::take().unwrap();
        let sda = peripherals.pins.gpio21;
        let scl = peripherals.pins.gpio22;
        
        let config = I2cConfig::new().baudrate(400.kHz().into());
        let i2c = I2cDriver::new(
            peripherals.i2c0,
            sda,
            scl,
            &config,
        );

        let delay = Delay::new();

        let mut sensor: BH1750 = BH1750::new(i2c, delay, false);
        Self { 
            urn: urn,
            device_urn: device_urn,
            location_urn: location_urn,
            name: name,
            sensor: sensor
        }
    }

    pub async fn _read(&self) -> Result<BH1750SensorMeasurement, Error> {
        let measurement: BH1750SensorMeasurement = match bh1750.get_one_time_measurement(Resolution::High) {
            Ok(lux) => {
                let condition: String = get_light_condition(lux);
                return BH1750SensorMeasurement{
                    lux: lux,
                    condition: condition
                };
            },
            Err(err) => {
                let condition: &'static str = "UNKOWN";
                return BH1750SensorMeasurement{
                    lux: lux,
                    condition: condition
                };
            }
        };
        Ok(measurement)
    }
}
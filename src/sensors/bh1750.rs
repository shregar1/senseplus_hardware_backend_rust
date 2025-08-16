use bh1750::{BH1750, Resolution};
use esp_hal::{
    delay::Delay,
    i2c::{I2cConfig, I2cDriver},
    peripherals::Peripherals,
    prelude::*,
};

use crate::abstractions::sensor::ISensor;
use crate::dtos::measurement::sensor::bh1750::BH1750SensorMeasurement;

pub struct BH1750Sensor {
    urn: String,
    device_urn: String,
    location_urn: String,
    name: String,
    sensor: BH1750<I2cDriver<'static>, Delay>,
}

impl ISensor<BH1750SensorMeasurement> for BH1750Sensor {

    fn urn(&self) -> String {
        self.urn.clone()
    }

    fn device_urn(&self) -> String {
        self.device_urn.clone()
    }

    fn location_urn(&self) -> String {
        self.location_urn.clone()
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn read_sync(&self) -> Result<BH1750SensorMeasurement, Box<dyn core::error::Error + Send + Sync>> {
        self._read()
    }
}

impl BH1750Sensor {
    pub fn new(
        urn: String,
        device_urn: String,
        location_urn: String,
        name: String,
    ) -> Result<Self, Box<dyn core::error::Error + Send + Sync>> {

        let peripherals = Peripherals::take().unwrap();
        let sda = peripherals.pins.gpio21;
        let scl = peripherals.pins.gpio22;
        
        let config = I2cConfig::new().baudrate(400u32.kHz().into());
        let i2c = I2cDriver::new(
            peripherals.i2c0,
            sda,
            scl,
            &config,
        )?;

        let delay = Delay::new();

        let mut sensor: BH1750<I2cDriver<'static>, Delay> = BH1750::new(i2c, delay, false);
        Ok(Self { 
            urn: urn,
            device_urn: device_urn,
            location_urn: location_urn,
            name: name,
            sensor: sensor
        })
    }

    fn _read(&self) -> Result<BH1750SensorMeasurement, Box<dyn core::error::Error + Send + Sync>> {
        let measurement: BH1750SensorMeasurement = match self.sensor.get_one_time_measurement(Resolution::High) {
            Ok(lux) => {
                let condition: String = get_light_condition(lux);
                BH1750SensorMeasurement{
                    lux: lux,
                    condition: condition
                }
            },
            Err(_err) => {
                let condition: &'static str = "UNKNOWN";
                BH1750SensorMeasurement{
                    lux: 0.0,
                    condition: condition
                }
            }
        };
        Ok(measurement)
    }
}

fn get_light_condition(lux: f32) -> String {
    match lux {
        0.0..=10.0 => "VERY_DARK".to_string(),
        10.1..=50.0 => "DARK".to_string(),
        50.1..=200.0 => "DIM".to_string(),
        200.1..=1000.0 => "NORMAL".to_string(),
        1000.1..=5000.0 => "BRIGHT".to_string(),
        5000.1..=10000.0 => "VERY_BRIGHT".to_string(),
        _ => "EXTREME".to_string(),
    }
}
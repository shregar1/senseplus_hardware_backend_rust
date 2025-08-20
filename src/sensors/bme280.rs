use alloc::string::{String};
use alloc::boxed::Box;
use core::fmt::Error;

use bme280::{BME280};
use esp_hal::{
    delay::Delay,
    i2c::{I2cConfig, I2cDriver},
    peripherals::Peripherals,
};

use crate::dtos::measurement::{sensor::bme280::BME280SensorMeasurement};

use crate::abstractions::sensor::ISensor;

pub struct BME280Sensor {
    urn: String,
    device_urn: String,
    location_urn: String,
    name: String,
    sensor: BME280<I2cDriver, Delay>,
}

impl ISensor<BME280SensorMeasurement> for BME280Sensor {
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

    fn read(&self) -> Result<BME280SensorMeasurement, Error> {
        self._read()
    }

}

impl BME280Sensor {
    fn new(
        urn: String,
        device_urn: String,
        location_urn: String,
        name: String,
    ) -> Self {
        let peripherals = Peripherals::take().unwrap();
        let sda = peripherals.pins.gpio21;
        let scl = peripherals.pins.gpio22;

        let config = I2cConfig::new().baudrate(400_000.into());
        let i2c = I2cDriver::new(
            peripherals.i2c0,
            sda,
            scl,
            &config,
        );

        let delay: Delay = Delay::new();

        let mut sensor: BME280<I2cDriver, Delay> = BME280::new_primary(
            i2c,
            delay,
        );
        sensor.init().unwrap();

        Self {
            urn: urn,
            device_urn: device_urn,
            location_urn: location_urn,
            name: name,
            sensor: sensor,
        }
    }

    pub fn _read(&self) -> Result<BME280SensorMeasurement, Error> {
        let measurement: BME280SensorMeasurement = match self.sensor.measure(){
            Ok(measurements) => {
                BME280SensorMeasurement { 
                    temperature: measurements.temperature,
                    humidity: measurements.humidity,
                    pressure: measurements.pressure 
                }
            },
            Err(e) => {
                BME280SensorMeasurement { 
                    temperature: 0.0,
                    humidity: 0.0,
                    pressure: 0.0 
                }
            }
        };
        Ok(measurement)
    }
}
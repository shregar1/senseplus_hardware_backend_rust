use esp_idf_hal::{
    delay::Delay,
    i2c::{I2cConfig, I2cDriver},
    peripherals::Peripherals,
};
use vl53l0x::VL53L0x;

use crate::abstractions::sensor::ISensor;
use crate::constants::distance::DistanceConstant;
use crate::dtos::measurement::sensor::vl53l0x::VL53L0XSensorMeasurement;

pub struct VL53L0XSensor {
    pub urn: String,
    pub device_urn: String,
    pub location_urn: String,
    pub name: String,
    pub sensor: VL53L0x
}

impl ISensor for VL53L0XSensor  {
    
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

    async fn read(&self) -> Result<T, Error> {
        self._read().await
    }
}

impl VL53L0XSensor {

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

        let mut sensor: VL53L0x = VL53L0x::new(
            i2c,
        ).unwrap();

        Self {
            urn: urn,
            device_urn: device_urn,
            location_urn: location_urn,
            name: name,
            sensor: sensor
        };
    }

    fn get_distance_status(&self, distance_mm: u16) -> &'static str {
        match distance_mm {
            0..=10 => DistanceConstant::TOO_CLOSE,
            11..=50 => DistanceConstant::VERY_CLOSE, 
            51..=200 => DistanceConstant::CLOSE,
            201..=500 => DistanceConstant::NEAR,
            501..=1000 => DistanceConstant::MEDIUM,
            1001..=2000 => DistanceConstant::FAR,
            _ => DistanceConstant::OUT_OF_RANGE,
        }
    }

    pub async fn _read(&self) -> Result<VL53L0XSensorMeasurement, Error> {

        let measurement: VL53L0XSensorMeasurement = match self.sensor.read_range_single_millimeters_blocking () {
            Ok(distance_mm) => {
                let status = self.get_distance_status(distance_mm);
                VL53L0XSensorMeasurement{
                    distance_mm: distance_mm,
                    status: status
                }
            },
            Err(e) => {
                VL53L0XSensorMeasurement{
                    distance_mm: i32::MAX,
                    status: DistanceConstant::UNKNOWN
                }
            }
        };
        Ok(measurement)
    }
    
}
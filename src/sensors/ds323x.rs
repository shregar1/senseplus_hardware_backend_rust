use std::time::{SystemTime, UNIX_EPOCH};

use chrono::NaiveDateTime;
use ds323x::NaiveDateTime;
use ds323x::{Ds323x, rtc::Hours, NaiveDate, NaiveTime, Rtcc};
use esp_hal::peripheral::Peripherals;

use crate::abstractions::sensor::ISensor;
use crate::dtos::measurement::sensor::ds323x::DS323XSensorMeasurement;

pub struct DS323XSensor {
    urn: String,
    device_urn: String,
    location_urn: String,
    name: String,
    sensor: Ds323x,
}

impl ISensor<DS323XSensorMeasurement> for DS323XSensor {

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

    async fn read(&self) -> Result<DS323XSensorMeasurement, Error> {
        self._read().await
    }
}

impl DS323XSensor {

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

        let timestamp: u64 = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let mut sensor: Ds323x = Ds323x::new_ds3231(i2c);

        let datetime: NaiveDateTime = NaiveDateTime::from_timestamp_opt(timestamp as i64, 0)
            .expect("Invalid timestamp");
        sensor.set_datetime(&datetime)?;
        Self {
            urn: urn,
            device_urn: device_urn,
            location_urn: location_urn,
            name: name,
            sensor: sensor
        }
    }

    async fn _read(&self) -> Result<DS323XSensorMeasurement, Error> {
        let measurement: DS323XSensorMeasurement = match self.sensor.now() {
            Ok(datetime) => {
                DS323XSensorMeasurement{
                    datetime: datetime
                }
            },
            Err(e) => {
                let timestamp: u64 = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
                let datetime: NaiveDateTime = NaiveDateTime::from_timestamp_opt(timestamp as i64, 0)
                    .expect("Invalid timestamp");
                DS323XSensorMeasurement{
                    datetime: datetime
                }
            }
        };
        Ok(measurement)
    }
    
}
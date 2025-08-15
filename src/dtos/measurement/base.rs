use crate::dtos::measurement::sensor::bh1750::BH1750SensorMeasurement;
use crate::dtos::measurement::sensor::bme280::BME280SensorMeasurement;

pub struct BaseMeasurementDTO{
    pub bh1750: Option<BH1750SensorMeasurement>,
    pub bme280: Option<BME280SensorMeasurement>
}
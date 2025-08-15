use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct BME280SensorMeasurement {
    pub temperature: f32,
    pub humidity: f32,
    pub pressure: f32
}

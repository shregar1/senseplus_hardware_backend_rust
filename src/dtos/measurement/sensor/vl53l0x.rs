use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct VL53L0XSensorMeasurement {
    pub distance_mm: f32,
    pub status: String
}

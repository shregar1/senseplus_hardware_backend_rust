use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct LSM303DLHCMAGSensorMeasurement {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub magnitude: f32,
    pub tilt: f32,
}
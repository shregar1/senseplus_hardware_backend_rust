use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct BH1750SensorMeasurement {
    pub lux: f64,
    pub condition: String,
}

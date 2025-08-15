use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct DS323XSensorMeasurement {
    pub datetime: String
}

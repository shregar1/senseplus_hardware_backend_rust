use alloc::string::String;

#[derive(Default, Debug)]
pub struct BH1750SensorMeasurement {
    pub lux: f64,
    pub condition: String,
}

use alloc::string::String;

#[derive(Default, Debug)]
pub struct VL53L0XSensorMeasurement {
    pub distance_mm: f32,
    pub status: String
}

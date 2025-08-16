pub struct UnitConstant;

impl UnitConstant {
    pub const TEMPERATURE: &'static str = "°C";      // Celsius
    pub const HUMIDITY: &'static str = "%";          // Percent
    pub const PRESSURE: &'static str = "hPa";        // Hectopascal
    pub const LUMINOSITY: &'static str = "lux";      // Lux
    pub const DISTANCE: &'static str = "mm";         // Millimeter
    pub const ACCELERATION: &'static str = "m/s²";   // Meters per second squared
    pub const MAGNETIC_FIELD: &'static str = "µT";   // Microtesla
}
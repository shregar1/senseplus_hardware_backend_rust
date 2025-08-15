pub struct DistanceConstant;

impl DistanceConstant {
    pub const UNIT: &'static str = "meter";
    pub const TOO_CLOSE: &'static str = "TOO_CLOSE";
    pub const VERY_CLOSE: &'static str = "VERY_CLOSE";
    pub const CLOSE: &'static str = "CLOSE";
    pub const NEAR: &'static str = "NEAR";
    pub const MEDIUM: &'static str = "MEDIUM";
    pub const FAR: &'static str = "FAR";
    pub const OUT_OF_RANGE: &'static str = "OUT_OF_RANGE";
    pub const UNKNOWN: &'static str = "UNKNOWN";
}
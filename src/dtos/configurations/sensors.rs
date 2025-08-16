use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct SensorsConfigDTO {
    pub include: Vec<String>,
}
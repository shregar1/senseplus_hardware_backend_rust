use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct SensorsConfigDTO {
    pub include: Vec<String>,
}
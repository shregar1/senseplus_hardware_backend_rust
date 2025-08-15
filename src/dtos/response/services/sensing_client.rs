use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize, Clone, Copy)]
pub struct SensingClientServiceResponseDTO {
    pub data: Value,
}
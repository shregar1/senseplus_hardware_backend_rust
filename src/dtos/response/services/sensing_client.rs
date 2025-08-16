use alloc::collections::BTreeMap;
use alloc::string::String;

#[derive(Debug, Clone)]
pub struct SensingClientServiceResponseDTO {
    pub data: BTreeMap<String, String>,
}
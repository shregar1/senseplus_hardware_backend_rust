use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseResponseDTO {
    pub status: String,
    pub message: String,
    pub data: Option<Value>,
}
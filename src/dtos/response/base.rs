use alloc::string::String;

use crate::enums::value::Value;

#[derive(Debug)]
pub struct BaseResponseDTO {
    pub status: String,
    pub message: String,
    pub data: Option<Value>,
}
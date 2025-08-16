use alloc::string::String;

#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Float(f32),
    Integer(i32),
    Boolean(bool),
}
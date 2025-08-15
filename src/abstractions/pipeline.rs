use serde_json::Value;

pub trait IPipeline {
    fn urn(&self) -> String;
    fn device_urn(&self) -> String;
    fn location_urn(&self) -> String;
    fn execute(&self) -> Result<Value, Error>;
}
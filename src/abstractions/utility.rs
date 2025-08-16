use alloc::string::String;

pub trait IUtility {
    fn urn(&self) -> String;
    fn device_urn(&self) -> String;
    fn location_urn(&self) -> String;
}
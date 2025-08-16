use alloc::string::String;
use alloc::fmt::Error;

pub trait IFactory<T> {
    fn urn(&self) -> String;
    fn device_urn(&self) -> String;
    fn location_urn(&self) -> String;
    fn get(&self, key: String) -> Result<T, Error>;
}
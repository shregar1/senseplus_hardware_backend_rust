use alloc::string::String;
use alloc::fmt::Error;

pub trait ISensor<T> {
    fn urn(&self) -> String;
    fn device_urn(&self) -> String;
    fn location_urn(&self) -> String;
    fn name(&self) -> String;
    fn read(&self) -> Result<T, Error>;
}

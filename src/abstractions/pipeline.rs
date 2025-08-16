use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::boxed::Box;
use core::error::Error;

use crate::abstractions::sensor::ISensor;
use crate::enums::value::Value;

pub trait IPipeline<T> {
    fn urn(&self) -> String;
    fn device_urn(&self) -> String;
    fn location_urn(&self) -> String;
    fn run(&self, sensor: &dyn ISensor<T>) -> Result<BTreeMap<String, Value>, Box<dyn Error + Send + Sync>>;
}
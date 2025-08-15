use std::{env, thread, time::Duration};

use anyhow::Result;
use esp_hal::Config;
use esp_idf_hal::{
    delay::Delay,
    i2c::{I2cConfig, I2cDriver},
    peripherals::Peripherals,
};
use esp_idf_svc::log::EspLogger;

use crate::config::Config as DeviceConfig;

mod abstractions;
mod config;
mod configurations;
mod constants;
mod dtos;
mod factories;
mod pipelines;
mod sensors;
mod services;
mod utilities;

fn main() -> Result<()> {
    // Required for ESP-IDF patches (startup)
    esp_idf_sys::link_patches();

    // Logging
    EspLogger::initialize_default();

    let config: DeviceConfig = DeviceConfig::new();
    loop {
    }
}
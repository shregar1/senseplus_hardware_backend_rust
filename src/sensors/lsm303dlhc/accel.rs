use esp_idf_hal::{
    delay::Delay,
    i2c::{I2cConfig, I2cDriver},
    peripherals::Peripherals,
};
use lsm303dlhc::Lsm303dlhc;

use crate::abstractions::sensor::ISensor;
use crate::dtos::measurement::sensor::lsm303dlhc::accel::LSM303DLHCACCELSensorMeasurement;

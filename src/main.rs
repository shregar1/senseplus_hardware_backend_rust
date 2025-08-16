#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::clock::CpuClock;
use esp_hal::timer::timg::TimerGroup;
use log::{info, debug, warn, error};

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    error!("PANIC: {:?}", info);
    loop {}
}

extern crate alloc;

pub mod constants;
pub mod config;
pub mod abstractions;
pub mod dtos;
pub mod enums;
pub mod services;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    // generator version: 0.5.0

    // Initialize the logger
    esp_println::logger::init_logger_from_env();
    debug!("Starting ESP32 application initialization...");
    
    debug!("Logger ready (using log + esp_println)");

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    debug!("ESP-HAL config created with max CPU clock");
    
    let peripherals = esp_hal::init(config);
    debug!("ESP-HAL peripherals initialized");

    esp_alloc::heap_allocator!(size: 64 * 1024);
    debug!("Heap allocator configured with 64KB");

    let timer0 = TimerGroup::new(peripherals.TIMG1);
    debug!("Timer group TIMG1 created");
    
    esp_hal_embassy::init(timer0.timer0);
    debug!("Embassy executor initialized with timer0");

    info!("Embassy initialized!");
    debug!("Application startup complete, entering main loop");

    // TODO: Spawn some tasks
    let _ = spawner;
    debug!("Spawner ready (no tasks spawned yet)");

    let mut loop_count = 0;
    loop {
        loop_count += 1;
        debug!("Main loop iteration: {}", loop_count);
        
        info!("Hello world!");
        
        if loop_count % 10 == 0 {
            warn!("Main loop has been running for {} iterations", loop_count);
        }
        
        Timer::after(Duration::from_secs(1)).await;
        debug!("Timer delay completed, continuing loop");
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-rc.0/examples/src/bin
}

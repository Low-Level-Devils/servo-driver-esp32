#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use embedded_hal::delay::DelayNs;
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::main;
use esp_hal::mcpwm::operator::PwmPinConfig;
use log::info;
use servo_driver::servo_control::{self, Servo};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[main]
fn main() -> ! {
    // generator version: 1.2.0

    esp_println::logger::init_logger_from_env();

    info!("Beginning Initialization...");
    info!("Initializing peripherals...");

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    info!("Initializing MCPWM0...");

    let mcpwm0 = servo_control::initialize_mcpwm(peripherals.MCPWM0, 40, 50, 20000);

    info!("Initializing servo...");

    let servo1_pin = mcpwm0
        .operator0
        .with_pin_a(peripherals.GPIO2, PwmPinConfig::UP_ACTIVE_HIGH);

    let mut servo1 = Servo::new(servo1_pin, 0.0, 180.0, 500.0, 2500.0);


    info!("Initialization complete, starting main loop");

    let mut delay = Delay::new();

    loop {
        servo1.move_to_angle(0.0);
        delay.delay_ms(500 as u32);
        servo1.move_to_angle(90.0);
        delay.delay_ms(500 as u32);
        servo1.move_to_angle(180.0);
        delay.delay_ms(500 as u32);
    }
}

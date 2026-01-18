// PWM frequency is 50 Hz so a period of 20 ms (20000 us)
// A pulse width of 1 ms (1000 us) corresponds to 0 degrees
// A pulse width of 1.5 ms (1500 us) corresponds to 90
// A pulse width of 2 ms (2000 us) corresponds to 180 degrees
// I used 20000 ticks and the period is evenly divided across the ticks.
// therefore 20 ms / 20000 ticks = 1 us per tick
// Tick to ms = ticks / 1000;

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
use esp_hal::mcpwm::{McPwm, PeripheralClockConfig, timer::PwmWorkingMode};
use esp_hal::time::Rate;
use log::info;

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

    let clock_config = PeripheralClockConfig::with_frequency(Rate::from_mhz(40)).unwrap();
    let mut mcpwm0 = McPwm::new(peripherals.MCPWM0, clock_config);

    mcpwm0.operator0.set_timer(&mcpwm0.timer0);

    info!("Initializing servo...");

    let mut servo1 = mcpwm0
        .operator0
        .with_pin_a(peripherals.GPIO2, PwmPinConfig::UP_ACTIVE_HIGH);

    info!("Configuring timer...");

    let timer_clock_config = clock_config
        .timer_clock_with_frequency(19999, PwmWorkingMode::Increase, Rate::from_hz(50))
        .expect("Failed to configure timer_clock_config");

    info!("Starting timer...");

    mcpwm0.timer0.start(timer_clock_config);

    info!("Initialization complete, starting main loop");

    let mut delay = Delay::new();

    loop {
        servo1.set_timestamp(angle_to_timestamp(0.0));
        delay.delay_ms(500 as u32);
        servo1.set_timestamp(angle_to_timestamp(90.0));
        delay.delay_ms(500 as u32);
        servo1.set_timestamp(angle_to_timestamp(180.0));
        delay.delay_ms(500 as u32);
    }
}

fn angle_to_timestamp(angle: f32) -> u16 {
    let min_angle = 0.0;
    let max_angle = 180.0;
    let min_timestamp = 500.0;
    let max_timestamp = 2500.0;

    if angle < min_angle || angle > max_angle {
        panic!("Angle out of range");
    }

    let timestamp = ((angle - min_angle) / (max_angle - min_angle))
        * (max_timestamp - min_timestamp)
        + min_timestamp;

    info!("Angle: {} -> Timestamp: {}", angle, timestamp);

    timestamp as u16
}

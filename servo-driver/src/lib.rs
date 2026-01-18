#![no_std]

use embedded_hal::delay::DelayNs;
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::main;
use esp_hal::mcpwm::operator::PwmPinConfig;
use esp_hal::mcpwm::{McPwm, PeripheralClockConfig, timer::PwmWorkingMode};
use esp_hal::time::Rate;
use log::info;
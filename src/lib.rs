// PWM frequency is 50 Hz so a period of 20 ms (20000 us)
// A pulse width of 1 ms (1000 us) corresponds to 0 degrees
// A pulse width of 1.5 ms (1500 us) corresponds to 90
// A pulse width of 2 ms (2000 us) corresponds to 180 degrees
// I used 20000 ticks and the period is evenly divided across the ticks.
// therefore 20 ms / 20000 ticks = 1 us per tick
// Tick to ms = ticks / 1000;

#![no_std]

pub mod servo_control {

    use esp_hal::mcpwm::{McPwm, PeripheralClockConfig};
    use esp_hal::mcpwm::PwmPeripheral;
    use esp_hal::time::Rate;
    use esp_hal::mcpwm::timer::PwmWorkingMode;
    use esp_hal::mcpwm::operator::PwmPin;

    pub fn initialize_mcpwm<'d, PWM: PwmPeripheral + 'd>(peripheral: PWM, mhz: u32, servo_hz: u32, period: u16) -> McPwm<'d, PWM> {
        let clock_config = PeripheralClockConfig::with_frequency(Rate::from_mhz(mhz)).unwrap();
        let mut mcpwm = McPwm::new(peripheral, clock_config);

        mcpwm.operator0.set_timer(&mcpwm.timer0);
        mcpwm.operator1.set_timer(&mcpwm.timer0);
        mcpwm.operator2.set_timer(&mcpwm.timer0);

        let timer_clock_config = clock_config
            .timer_clock_with_frequency(period - 1, PwmWorkingMode::Increase, Rate::from_hz(servo_hz))
            .expect("Failed to configure timer_clock_config");

        mcpwm.timer0.start(timer_clock_config);

        mcpwm
    }

    pub struct Servo<'d, PWM, const OP: u8, const IS_A: bool> {
        pwm_pin: PwmPin<'d, PWM, OP, IS_A>,
        min_angle: f32,
        max_angle: f32,
        min_timestamp: f32,
        max_timestamp: f32,
    }

    impl<'d, PWM: PwmPeripheral, const OP: u8, const IS_A: bool> Servo<'d, PWM, OP, IS_A> {
        pub fn new(pwm_pin: PwmPin<'d, PWM, OP, IS_A>, min_angle: f32, max_angle: f32, min_timestamp: f32, max_timestamp: f32) -> Self {
            Self {
                pwm_pin,
                min_angle,
                max_angle,
                min_timestamp,
                max_timestamp,
             }
        }

        pub fn move_to_angle(&mut self,angle: f32) {
            if angle < self.min_angle || angle > self.max_angle {
                return;
            }

            let timestamp = self.angle_to_timestamp(angle);
            self.pwm_pin.set_timestamp(timestamp);
        }

        fn angle_to_timestamp(&self, angle: f32) -> u16 {
            let timestamp = ((angle - self.min_angle) / (self.max_angle - self.min_angle))
                * (self.max_timestamp - self.min_timestamp)
                + self.min_timestamp;

            timestamp as u16
        }

    }
}
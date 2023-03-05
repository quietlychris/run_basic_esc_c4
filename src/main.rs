extern crate sysfs_pwm;
use sysfs_pwm::{Pwm, Result};

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::error::Error;
use std::thread;
use std::time::Duration;

// This has been tested to work on Odroid-C4 using the stock Ubuntu Odroid image
// May need to add `overlays="spi0 i2c0 i2c1 uart0 pwm_ab pwm_cd pwm_ef` to
// the /media/boot/config.ini file and restart before the PWM pins are available
// Pin #35 -> pwmchip0, channel 1
// Pin #33 -> pwmchip0, channel 0
// Pin #11 -> pwmchip4, channel 1
// Pin #7  -> pwmchip4, channel 0
// Pin #15 -> pwmchip8, channel 1
// Pin #12 -> pwmchip8, channel 0

const C4_PWM_CHIP: u32 = 0;
const C4_PWM_CHANNEL: u32 = 1;

const PERIOD_NS: u32 = 20_000_000;
const PULSE_MIN_NS: u32 = 1_200_000;
const PULSE_NEUTRAL_NS: u32 = 1_500_000;
const PULSE_MAX_NS: u32 = 1_800_000;

/// Make an LED "breathe" by increasing and
/// decreasing the brightness
fn main()  {
    let pwm = Pwm::new(C4_PWM_CHIP, C4_PWM_CHANNEL)
        .expect("An error occurred while trying to set up the PWM struct"); // number depends on chip, etc.
    pwm.export().expect("An error occurred while trying to export the PWM struct");
    pwm.set_period_ns(PERIOD_NS).expect("Error while setting PWM period");
    pwm.set_duty_cycle_ns(PULSE_MIN_NS).expect("Error while setting PWM duty cycle");

    pwm.enable(true).expect("An error occurred while trying to enable the PWM pin");
    println!("pwm enabled");
    
    // PWM startup sequence
    thread::sleep(Duration::from_millis(500));
    pwm.set_duty_cycle_ns(PULSE_MIN_NS).unwrap();
    thread::sleep(Duration::from_millis(500));
    pwm.set_duty_cycle_ns(PULSE_NEUTRAL_NS).unwrap();
    thread::sleep(Duration::from_millis(300));
    
    // Around 1_57/80_000 for lift
    pwm.set_duty_cycle_ns(1_580_000).unwrap(); // 1_520_000 seems to the minimum to get it moving
    thread::sleep(Duration::from_millis(7_000));

    pwm.enable(false).expect("An error occurred while trying to disable the PWM pin");
    println!("pwm disabled");
}

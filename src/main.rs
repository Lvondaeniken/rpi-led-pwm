use anyhow::{Context, Result};
use rppal::pwm::{Channel, Polarity, Pwm};
use std::{thread, time};

const PWM_FREQUENCY: f64 = 800_000.0; // PWM Frequency in Hz
const DUTY_CYCLE_ONE: f64 = 0.66; // Duty cycle for 1
const DUTY_CYCLE_ZERO: f64 = 0.33; // Duty cycle for 0

fn main() -> Result<()> {
    println!("hello");
    let pwm = Pwm::with_frequency(Channel::Pwm0, PWM_FREQUENCY, 0.0, Polarity::Normal, false)
        .context("Failed to initialize PWM")?;

    let data = vec![1, 0, 1, 1, 0, 0, 1, 0];
    let data2 = vec![1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];


    for &bit in &data2 {
        let duty_cycle = match bit {
            1 => DUTY_CYCLE_ONE,
            _ => DUTY_CYCLE_ZERO,
        };
        
        set_duty_cycle(&pwm, duty_cycle)?;
        thread::sleep(time::Duration::from_nanos(1250));
    }
    Ok(())
}

fn set_duty_cycle(pwm: &Pwm, duty_cycle: f64) -> Result<()> {
    pwm.set_duty_cycle(duty_cycle)
        .context("Failed to set duty cycle")?;

    pwm.enable()
        .context("Failed to apply PWM changes")?;

    Ok(())
}

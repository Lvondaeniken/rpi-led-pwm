use anyhow::{Context, Result};
use rppal::pwm::{Channel, Polarity, Pwm};
use std::{thread, time};

const PWM_FREQUENCY: f64 = 1000.0; // PWM Frequency in Hz
const DUTY_CYCLE_ONE: f64 = 0.66; // Duty cycle for 1
const DUTY_CYCLE_ZERO: f64 = 0.33; // Duty cycle for 0

fn main() -> Result<()> {
    let pwm = Pwm::with_requency(Channel::Pwm0, PWM_FREQUENCY, 0.0, Polarity::Normal)
        .context("Failed to initialize PWM")?;

    let data = vec![1, 0, 1, 1, 0, 0, 1, 0];

    for &bit in &data {
        let duty_cycle = match bit {
            1 => DUTY_CYCLE_ONE,
            _ => DUTY_CYCLE_ZERO,
        };
        
        set_duty_cycle(&pwm, duty_cycle)?;
        thread::sleep(time::Duration::from_millis(500));
    }
    Ok(())
}

fn set_duty_cycle(pwm: &Pwm, duty_cycle: f64) -> Result<()> {
    pwm.set_duty_cycle(Channel::Pwm0, duty_cycle)
        .context("Failed to set duty cycle")?;

    pwm.apply(Channel::Pwm0)
        .context("Failed to apply PWM changes")?;

    Ok(())
}

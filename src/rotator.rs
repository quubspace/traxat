use rppal::{gpio::Gpio, system::DeviceInfo};

use anyhow::{anyhow, Result};

use std::{thread, time::Duration};

use log::info;

const MOTOR_ELE_GPIO: [u8; 4] = [6, 13, 19, 26];
const MOTOR_AZ_GPIO: [u8; 4] = [9, 11, 0, 5];
const STEPS_PER_ROT: u32 = 64;

#[derive(Debug)]
pub struct Rotator {
    pub ele: f32,        // Elevation
    pub az: f32,         // Azimuth
    pub ele_target: f32, // Y axis target position
    pub az_target: f32,  // X axis target position
}

impl Rotator {
    pub fn new() -> Rotator {
        Rotator {
            ele: 20_f32,
            az: 0_f32,
            ele_target: 20_f32,
            az_target: 0_f32,
        }
    }

    pub fn mv(&mut self) -> Result<()> {
        let ele_steps = (self.ele - self.ele_target) as i32 / (360 / STEPS_PER_ROT) as i32;
        let az_steps = (self.az - self.az_target) as i32 / (360 / STEPS_PER_ROT) as i32;

        // Move elevation stepper
        if ele_steps != 0 {
            self.move_steppers(ele_steps, &MOTOR_ELE_GPIO)?;
        }

        // Move azimuth stepper
        if az_steps != 0 {
            self.move_steppers(az_steps, &MOTOR_AZ_GPIO)?;
        }

        self.ele = self.ele_target;
        self.az = self.az_target;

        info!("Elevation is {}, Azimuth is {}.", self.ele, self.az);

        Ok(())
    }

    pub fn zero(&mut self) -> Result<()> {
        self.ele_target = 20_f32;
        self.az_target = 0_f32;

        self.mv()?;

        Ok(())
    }

    fn move_steppers(&self, steps: i32, gpio_pin_list: &[u8]) -> Result<()> {
        info!("Moving motor on a {}.", DeviceInfo::new()?.model());

        for _ in 0..steps.abs() {
            if steps >= 0 {
                self.step_pins_forward(gpio_pin_list)?;
            } else {
                self.step_pins_backward(gpio_pin_list)?;
            }
        }

        info!("Done!");

        Ok(())
    }

    fn step_pins_forward(&self, pins_list: &[u8]) -> Result<()> {
        let mut last = Gpio::new()?
            .get(*pins_list.last().ok_or_else(|| anyhow!("No last pin!"))?)?
            .into_output();

        last.set_high();
        thread::sleep(Duration::from_millis(10));
        last.set_low();

        drop(last);

        for pair in pins_list.windows(2) {
            let mut last = Gpio::new()?.get(pair[0])?.into_output();
            let mut cur = Gpio::new()?.get(pair[1])?.into_output();

            last.set_high();
            cur.set_high();
            thread::sleep(Duration::from_millis(10));
            last.set_low();
            cur.set_low();

            cur.set_high();
            thread::sleep(Duration::from_millis(10));
            cur.set_low();
        }

        let mut first = Gpio::new()?
            .get(*pins_list.first().ok_or_else(|| anyhow!("No first pin!"))?)?
            .into_output();
        let mut last = Gpio::new()?
            .get(*pins_list.last().ok_or_else(|| anyhow!("No last pin!"))?)?
            .into_output();

        last.set_high();
        first.set_high();
        thread::sleep(Duration::from_millis(10));
        last.set_low();
        first.set_low();

        Ok(())
    }

    fn step_pins_backward(&self, pins_list: &[u8]) -> Result<()> {
        let mut last = Gpio::new()?
            .get(*pins_list.last().ok_or_else(|| anyhow!("No last pin!"))?)?
            .into_output();

        last.set_high();
        thread::sleep(Duration::from_millis(10));
        last.set_low();

        drop(last);

        for pair in pins_list.windows(2) {
            let mut last = Gpio::new()?.get(pair[0])?.into_output();
            let mut cur = Gpio::new()?.get(pair[1])?.into_output();

            cur.set_high();
            last.set_high();
            thread::sleep(Duration::from_millis(10));
            cur.set_low();
            last.set_low();

            last.set_high();
            thread::sleep(Duration::from_millis(10));
            last.set_low();
        }

        let mut first = Gpio::new()?
            .get(*pins_list.first().ok_or_else(|| anyhow!("No first pin!"))?)?
            .into_output();
        let mut last = Gpio::new()?
            .get(*pins_list.last().ok_or_else(|| anyhow!("No last pin!"))?)?
            .into_output();

        first.set_high();
        last.set_high();
        thread::sleep(Duration::from_millis(10));
        first.set_low();
        last.set_low();

        Ok(())
    }
}

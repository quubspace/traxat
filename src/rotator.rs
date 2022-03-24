use rppal::{gpio::Gpio, system::DeviceInfo};

use anyhow::{anyhow, Result};

use std::{thread, time::Duration};

use log::info;

// Pin map
// 6 => ELE1 A
// 13 => ELE2 B
// 19 => ELE3 A_
// 26 => ELE4 B_

// 9 => AZ1 A
// 11 => AZ2 B
// 0 => AZ3 A_
// 5 => AZ4 B_

const MOTOR_ELE_GPIO: [u8; 4] = [6, 13, 19, 26];
const MOTOR_AZ_GPIO: [u8; 4] = [9, 11, 0, 5];
const STEPS_PER_ROT: f32 = 512.0;

#[derive(Debug)]
pub struct Rotator {
    pub ele: f32,        // Elevation
    pub az: f32,         // Azimuth
    pub ele_target: f32, // Y axis target position
    pub az_target: f32,  // X axis target position
    pub num_steps: i32,  // Number of steps to go
}

impl Rotator {
    pub fn new() -> Rotator {
        Rotator {
            ele: 20_f32,
            az: 0_f32,
            ele_target: 20_f32,
            az_target: 0_f32,
            num_steps: 0_f32,
        }
    }

    pub fn mv(&mut self) -> Result<()> {
        let steps_per_degree = (STEPS_PER_ROT / 360.0) as f32;
        let ele_steps = (self.ele_target - self.ele) * steps_per_degree as f32;
        let az_steps = (self.az_target - self.az) * steps_per_degree as f32;
        info!("ele_steps is {}.", ele_steps);
        info!("az_steps is {}.", az_steps);
        info!("steps_per_degree is {}.", steps_per_degree);

        // Move elevation stepper
        if ele_steps != 0.0 {
            self.move_steppers(ele_steps, &MOTOR_ELE_GPIO)?;
        }

        // Move azimuth stepper
        if az_steps != 0.0 {
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

    pub fn test_steppers(&self) -> Result<()> {
        let cur_steps = self.num_steps;
        info!("Moving motor {} steps", cur_steps);
        self.test_move_steppers(cur_steps, &MOTOR_ELE_GPIO)?;

        Ok(())
    }

    fn test_move_steppers(&self, steps: i32, gpio_pin_list: &[u8]) -> Result<()> {
        let abs_steps = steps.abs();
        info!("Moving motor on a {}.", DeviceInfo::new()?.model());

        for _ in 0..abs_steps {
            if steps >= 0 {
                self.test_step_pins_forward(gpio_pin_list)?;
            } else {
                self.step_pins_backward(gpio_pin_list)?;
            }
        }

        info!("Done!");

        Ok(())
    }

    fn move_steppers(&self, steps: f32, gpio_pin_list: &[u8]) -> Result<()> {
        info!("Moving motor on a {}.", DeviceInfo::new()?.model());
        let abs_steps = steps.abs().floor() as i32;

        for _ in 0..abs_steps {
            if steps >= 0.0 {
                self.step_pins_forward(gpio_pin_list)?;
            } else {
                self.step_pins_backward(gpio_pin_list)?;
            }
        }

        info!("Done!");

        Ok(())
    }

    fn step_pins_forward(&self, pins_list: &[u8]) -> Result<()> {
        let delay_set = 2;

        let mut a1 = Gpio::new()?.get(pins_list[0])?.into_output();
        let mut b1 = Gpio::new()?.get(pins_list[1])?.into_output();
        let mut a2 = Gpio::new()?.get(pins_list[2])?.into_output();
        let mut b2 = Gpio::new()?.get(pins_list[3])?.into_output();

        // Full-step sequence
        // Quarter-step 1
        a1.set_high();
        b1.set_low();
        a2.set_low();
        b2.set_high();
        thread::sleep(Duration::from_millis(delay_set));

        // Quarter-step 2
        a1.set_high();
        b1.set_high();
        a2.set_low();
        b2.set_low();
        thread::sleep(Duration::from_millis(delay_set));

        // Quarter-step 3
        a1.set_low();
        b1.set_high();
        a2.set_high();
        b2.set_low();
        thread::sleep(Duration::from_millis(delay_set));

        // Quarter-step 4
        a1.set_low();
        b1.set_low();
        a2.set_high();
        b2.set_high();
        thread::sleep(Duration::from_millis(delay_set));

        // Motor off
        a1.set_low();
        b1.set_low();
        a2.set_low();
        b2.set_low();

        Ok(())
    }

    fn step_pins_backward(&self, pins_list: &[u8]) -> Result<()> {
        let delay_set = 2;

        let mut a1 = Gpio::new()?.get(pins_list[0])?.into_output();
        let mut b1 = Gpio::new()?.get(pins_list[1])?.into_output();
        let mut a2 = Gpio::new()?.get(pins_list[2])?.into_output();
        let mut b2 = Gpio::new()?.get(pins_list[3])?.into_output();

        // Full-step sequence

        // Step 4
        a1.set_low();
        b1.set_low();
        a2.set_high();
        b2.set_high();
        thread::sleep(Duration::from_millis(delay_set));

        // Step 3
        a1.set_low();
        b1.set_high();
        a2.set_high();
        b2.set_low();
        thread::sleep(Duration::from_millis(delay_set));

        // Step 2
        a1.set_high();
        b1.set_high();
        a2.set_low();
        b2.set_low();
        thread::sleep(Duration::from_millis(delay_set));

        // Step 1
        a1.set_high();
        b1.set_low();
        a2.set_low();
        b2.set_high();
        thread::sleep(Duration::from_millis(delay_set));

        // Motor off
        a1.set_low();
        b1.set_low();
        a2.set_low();
        b2.set_low();

        Ok(())
    }
}

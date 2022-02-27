use rppal::{gpio::Gpio, system::DeviceInfo};

use anyhow::{anyhow, Result};

use std::{thread, time::Duration};

use log::{debug, info, warn};

const GPIO_PINS: [u8; 8] = [6, 13, 19, 26, 9, 11, 0, 5];

#[derive(Debug)]
pub struct Rotator {
    pub ele: f32,
    pub az: f32,
    pub xc: f32,
    pub yc: f32,
    pub yt: f32,
    pub xt: f32,
}

impl Rotator {
    pub fn new() -> Rotator {
        Rotator {
            ele: 20 as f32,
            az: 0 as f32,
            xc: 6.666 as f32,
            yc: 1.458 as f32,
            yt: 20 as f32,
            xt: 0 as f32,
        }
    }

    pub fn mv(&self) {
        let old_xt = self.xt;
        let old_yt = self.yt;

        if self.yt <= 20 {
            self.yt = 20;
        }

        if self.yt >= 85 {
            debug!("Old x = {}", self.xt);

            if self.xt >= 180 {
                self.xt -= 90;
            } else {
                self.xt += 90;
            }
        }

        // Testing for right position
        if self.xt >= self.az {
            // Testing for up position
            if self.yt >= self.ele {
                self.ur();
            } else {
                // If right position and not up, must be down and right
                self.dr()
            }
        // Testing for left position
        } else if self.xt <= self.az {
            // Testing for up position
            if self.yt >= self.ele {
                self.ul();
            } else {
                // If left position and not up, must be down and left
                self.dl();
            }
        }

        self.xt = old_xt;
        self.yt = old_yt;
    }

    pub fn ul(&self) {
        let yseconds = (self.yt - self.ele) * (1 / self.yc);
        info!("Y: Target {} degrees", self.yt);
        info!("Moving up for {} seconds", yseconds);

        let xseconds = (self.az - self.xt) * (1 / self.xc);
        info!("X: Target {} degrees", self.xt);
        info!("Moving left for {} seconds", xseconds);

        if xseconds > yseconds {
        } else {
        }

        self.ele = self.yt;
        info!("Elevation is {}", self.ele);

        self.az = self.xt;
        info!("Azimuth is {}", self.az);
    }

    pub fn ur(&self) {}

    pub fn dl(&self) {}

    pub fn dr(&self) {}

    pub fn max(&self) {}

    pub fn center(&self) {}

    pub fn zero(&self) {}

    fn move_steppers(&self, steppers: Vec<usize>) -> Result<()> {
        println!("Moving motor on a {}.", DeviceInfo::new()?.model());

        let mut chunked_pins = GPIO_PINS.chunks(4);

        for steps in steppers {
            let pins_list = &chunked_pins.next().ok_or_else(|| {
                anyhow!("No next stepper! Ensure you did not add an extra step number!")
            })?;
            for x in 0..steps {
                step_pins(x + 1, pins_list)?;
            }
        }

        println!("Done!");

        Ok(())
    }

    fn step_pins(&self, step_num: usize, pins_list: &[u8]) -> Result<()> {
        let mut last = Gpio::new()?
            .get(*pins_list.last().ok_or_else(|| anyhow!("No last pin!"))?)?
            .into_output();

        last.set_high();
        thread::sleep(Duration::from_millis(1));
        last.set_low();

        drop(last);

        for pair in pins_list.windows(2) {
            let mut last = Gpio::new()?.get(pair[0])?.into_output();
            let mut cur = Gpio::new()?.get(pair[1])?.into_output();

            last.set_high();
            cur.set_high();
            thread::sleep(Duration::from_millis(1));
            last.set_low();
            cur.set_low();

            cur.set_high();
            thread::sleep(Duration::from_millis(1));
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
        thread::sleep(Duration::from_millis(1));
        last.set_low();
        first.set_low();

        info!("Step number: {}", step_num);

        Ok(())
    }
}

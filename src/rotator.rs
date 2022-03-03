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

    pub fn mv(&mut self) {
        let old_xt = self.xt;
        let old_yt = self.yt;

        if self.yt <= 20 as f32 {
            self.yt = 20 as f32;
        }

        if self.yt >= 85 as f32 {
            debug!("Old x = {}", self.xt);

            if self.xt >= 180 as f32 {
                self.xt -= 90 as f32;
            } else {
                self.xt += 90 as f32;
            }
        }

        // Testing for right position
        if self.xt >= self.az {
            // Testing for up position
            if self.yt >= self.ele {
                info!("UR!");
                self.ur();
            } else {
                // If right position and not up, must be down and right
                info!("DR!");
                self.dr()
            }
        // Testing for left position
        } else if self.xt <= self.az {
            // Testing for up position
            if self.yt >= self.ele {
                info!("UL!");
                self.ul();
            } else {
                // If left position and not up, must be down and left
                info!("DL!");
                self.dl();
            }
        }

        self.xt = old_xt;
        self.yt = old_yt;
    }

    pub fn ul(&mut self) {
        let yseconds = (self.yt - self.ele) * (1 as f32 / self.yc);
        info!("Y: Target {} degrees", self.yt);
        info!("Moving up for {} seconds", yseconds);

        let xseconds = (self.az - self.xt) * (1 as f32 / self.xc);
        info!("X: Target {} degrees", self.xt);
        info!("Moving left for {} seconds", xseconds);

        if xseconds > yseconds {
            self.move_steppers();
        } else {
            self.move_steppers();
        }

        self.ele = self.yt;
        info!("Elevation is {}", self.ele);

        self.az = self.xt;
        info!("Azimuth is {}", self.az);
    }

    pub fn ur(&mut self) {
        let yseconds = (self.yt - self.ele) * (1 as f32 / self.yc);
        info!("Y: Target {} degrees", self.yt);
        info!("Moving up for {} seconds", yseconds);

        let xseconds = (self.xt - self.az) * (1 as f32 / self.xc);
        info!("X: Target {} degrees", self.xt);
        info!("Moving left for {} seconds", xseconds);

        if xseconds > yseconds {
            self.move_steppers();
        } else {
            self.move_steppers();
        }

        self.ele = self.yt;
        info!("Elevation is {}", self.ele);

        self.az = self.xt;
        info!("Azimuth is {}", self.az);
    }

    pub fn dl(&mut self) {
        let yseconds = (self.ele - self.yt) * (1 as f32 / self.yc);
        info!("Y: Target {} degrees", self.yt);
        info!("Moving up for {} seconds", yseconds);

        let xseconds = (self.az - self.xt) * (1 as f32 / self.xc);
        info!("X: Target {} degrees", self.xt);
        info!("Moving left for {} seconds", xseconds);

        if xseconds > yseconds {
            self.move_steppers();
        } else {
            self.move_steppers();
        }

        self.ele = self.yt;
        info!("Elevation is {}", self.ele);

        self.az = self.xt;
        info!("Azimuth is {}", self.az);
    }

    pub fn dr(&self) {}

    pub fn max(&self) {}

    pub fn center(&self) {}

    pub fn zero(&self) {}

    fn move_steppers(&self) -> Result<()> {
        println!("Moving motor on a {}.", DeviceInfo::new()?.model());

        let mut chunked_pins = GPIO_PINS.chunks(4);

        for steps in 0..chunked_pins.len() {
            let pins_list = &chunked_pins.next().ok_or_else(|| {
                anyhow!("No next stepper! Ensure you did not add an extra step number!")
            })?;

            let mut counter = 0;

            loop {
                self.step_pins(pins_list)?;

                if counter == 100 {
                    break;
                }

                counter += 1;
            }
        }

        println!("Done!");

        Ok(())
    }

    fn step_pins(&self, pins_list: &[u8]) -> Result<()> {
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

        Ok(())
    }
}

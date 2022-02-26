use anyhow::Result;

use log::{debug, info, warn};

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

    pub fn mv(&self) {}

    pub fn ul(&self) {}

    pub fn ur(&self) {}

    pub fn dl(&self) {}

    pub fn dr(&self) {}

    pub fn max(&self) {}

    pub fn center(&self) {}

    pub fn zero(&self) {}
}

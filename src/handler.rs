use std::process;

use log::{debug, info, warn};

use crate::message::Message;
use crate::rotator::Rotator;

#[derive(Debug)]
pub struct ActionHandler<'a> {
    pub rotator: &'a mut Rotator,
}

impl<'a> ActionHandler<'a> {
    pub fn new(rotator: &'a mut Rotator) -> Self {
        Self { rotator }
    }

    pub fn handle_p_set(&mut self, azimuth: f32, elevation: f32) -> String {
        self.rotator.az_target = azimuth;
        self.rotator.ele_target = elevation;

        info!("Set to {}:{}", azimuth, elevation);

        self.rotator.mv();

        String::from("\n")
    }

    pub fn handle_p_get(&self) -> String {
        format!("{}\n{}", self.rotator.az, self.rotator.ele)
    }

    pub fn handle_message(&mut self, msg: Message) -> String {
        match msg {
            Message::PSet(az, ele) => self.handle_p_set(az, ele),
            Message::PGet => self.handle_p_get(),
            Message::Close => String::from("rotctld_quit"),
            _ => String::from("Not a command!"),
        }
    }
}

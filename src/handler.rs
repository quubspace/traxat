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
        self.rotator.az = azimuth;
        self.rotator.ele = elevation;

        info!("Set to {}:{}", azimuth, elevation);

        self.rotator.mv();

        String::from("\n")
    }

    pub fn handle_p_get(&self) -> String {
        String::from(format!("{}\n{}", self.rotator.az, self.rotator.ele))
    }

    pub fn close_connection(&self) -> String {
        warn!("Program is exiting, rotctld sent quit!");
        process::exit(0);
    }

    pub fn handle_message(&mut self, msg: Message) -> String {
        if matches!(msg, Message::Close) {
            self.close_connection();
        }

        let r = match msg {
            Message::PSet(az, ele) => self.handle_p_set(az, ele),
            Message::PGet => self.handle_p_get(),
            _ => String::from("Not a command!"),
        };

        String::from(r)
    }
}

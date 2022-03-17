use anyhow::Result;
use log::info;

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

    pub fn handle_p_set(&mut self, azimuth: f32, elevation: f32) -> Result<String> {
        self.rotator.az_target = azimuth;
        self.rotator.ele_target = elevation;

        info!("Set to {}:{}", azimuth, elevation);

        self.rotator.mv()?;

        Ok(String::from("\n"))
    }

    pub fn handle_p_get(&self) -> String {
        format!("{}\n{}", self.rotator.az, self.rotator.ele)
    }

    pub fn handle_step_test(&mut self, quickrotsteps: f32) -> Result<String> {
        self.rotator.numsteps = quickrotsteps;
        info!("Sending {} steps...", quickrotsteps);

        self.rotator.test_steppers()?;

        Ok(String::from("\n"))
    }

    pub fn handle_message(&mut self, msg: Message) -> Result<String> {
        match msg {
            Message::PSet(az, ele) => self.handle_p_set(az, ele),
            Message::PGet => Ok(self.handle_p_get()),
            Message::StepTest(seps) => self.handle_step_test(seps),
            Message::Close => Ok(String::from("rotctld_quit")),
            _ => Ok(String::from("Not a command!")),
        }
    }
}

use anyhow::Result;
use log::info;

use crate::message::Message;
use crate::rotator::Rotator;

#[derive(Debug)]

/// Action handler for .......
pub struct ActionHandler<'a> {
    /// Public rotator (object?)
    pub rotator: &'a mut Rotator,
}

impl<'a> ActionHandler<'a> {
    /// Returns a ActionHandler with a given Rotator object (Not sure what to change object to)
    ///
    /// # Arguments
    ///
    /// * `rotator` - Rotator object used for the action
    ///
    pub fn new(rotator: &'a mut Rotator) -> Self {
        Self { rotator }
    }

    /// Returns a Result of setting the pins
    ///
    /// # Arguments
    ///
    /// * `azimuth` - What to set the azumuth to
    ///
    /// * `elevation` - What to set the elevation to
    ///
    pub fn handle_p_set(&mut self, azimuth: f32, elevation: f32) -> Result<String> {
        self.rotator.az_target = azimuth;
        self.rotator.ele_target = elevation;

        info!("Set to {}:{}", azimuth, elevation);

        self.rotator.mv()?;

        Ok(String::from("\n"))
    }

    /// Gets the azimuth and elevation values.
    pub fn handle_p_get(&self) -> String {
        format!("{}\n{}", self.rotator.az, self.rotator.ele)
    }

    /// Returns a Result of the step test.
    ///
    /// # Arguments
    ///
    /// * `quick_rot_steps` - How many steps to move.
    ///
    pub fn handle_step_test(&mut self, quick_rot_steps: i32) -> Result<String> {
        self.rotator.num_steps = quick_rot_steps;
        info!("Sending {} steps...", quick_rot_steps);

        self.rotator.test_steppers()?;

        Ok(String::from("\n"))
    }

    /// Returns a Result of taking a handling the message.
    ///
    /// # Arguments
    ///
    /// * `Message` - The message to be handled
    ///
    pub fn handle_message(&mut self, msg: Message) -> Result<String> {
        match msg {
            Message::PSet(az, ele) => self.handle_p_set(az, ele),
            Message::PGet => Ok(self.handle_p_get()),
            Message::StepTest(steps) => self.handle_step_test(steps),
            Message::Close => Ok(String::from("rotctld_quit")),
            _ => Ok(String::from("Not a command!")),
        }
    }
}

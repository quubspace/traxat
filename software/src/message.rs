use std::{
    convert::Infallible,
    str::{self, FromStr},
};

use anyhow::Result;

use log::debug;

#[derive(Debug)]
/// A message from gpredict
pub enum Message {
    /// Pin set function
    PSet(f32, f32),
    /// Gets position from gpredict
    PGet,
    /// Step test funcion.
    StepTest(i32),
    /// Close message to stop communication.
    Close,
    /// Failed to get a command.
    NotACommand,
}

impl FromStr for Message {
    /// Returns a message object from a string.
    ///
    /// # Arguments
    ///
    /// * `response` - A message in the form of a string.
    ///
    type Err = Infallible;
    fn from_str(response: &str) -> Result<Message, Infallible> {
        let res: Vec<String> = response.split_whitespace().map(|s| s.to_string()).collect();

        let (cmd, params) = res.split_first().unwrap();

        let params: Vec<f32> = params.iter().map(|x| x.parse::<f32>().unwrap()).collect();

        debug!("Command: {:?} - Parameters: {:?}", cmd, params);

        match cmd.as_str() {
            "p" => Ok(Message::PGet),
            "s" => Ok(Message::StepTest(params[0] as i32)),
            "P" => Ok(Message::PSet(params[0], params[1])),
            "q" => Ok(Message::Close),
            _ => Ok(Message::NotACommand),
        }
    }
}

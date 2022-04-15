use std::{
    convert::Infallible,
    str::{self, FromStr},
};

use anyhow::Result;

use log::debug;

#[derive(Debug)]
pub enum Message {
    PSet(f32, f32),
    PGet,
    StepTest(i32),
    Close,
    NotACommand,
}

impl FromStr for Message {
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

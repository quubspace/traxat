use std::{
    convert::Infallible,
    io, process,
    str::{self, FromStr},
};

use anyhow::Result;

use chrono;
use fern::{log_file, Dispatch};
use log::{info, warn, LevelFilter};

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[derive(Debug)]
enum Message {
    PSet(f32, f32),
    PGet,
    Close,
    NotACommand,
}

impl FromStr for Message {
    type Err = Infallible;

    fn from_str(response: &str) -> Result<Message, Infallible> {
        let res: Vec<String> = response.split_whitespace().map(|s| s.to_string()).collect();

        let (cmd, params) = res.split_first().unwrap();

        let params: Vec<f32> = params
            .into_iter()
            .map(|x| x.parse::<f32>().unwrap())
            .collect();

        info!("Command: {:?}\nParameters: {:?}", cmd, params);

        match cmd.as_str() {
            "p" => Ok(Message::PGet),
            "P" => Ok(Message::PSet(params[0], params[1])),
            "q" => Ok(Message::Close),
            _ => Ok(Message::NotACommand),
        }
    }
}

#[derive(Debug)]
struct ActionHandler<'a> {
    rotator: &'a mut Rotator,
}

impl<'a> ActionHandler<'a> {
    pub fn new(rotator: &'a mut Rotator) -> Self {
        Self { rotator }
    }

    pub fn handle_p_set(&mut self, azimuth: f32, elevation: f32) -> String {
        self.rotator.xt = azimuth;
        self.rotator.yt = elevation;

        info!("Set to {}:{}", azimuth, elevation);

        self.rotator.mv();

        String::from("")
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

        String::from(format!("{}\n", r))
    }
}

#[derive(Debug, Copy, Clone)]
struct Rotator {
    ele: f32,
    az: f32,
    xc: f32,
    yc: f32,
    yt: f32,
    xt: f32,
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

#[tokio::main]
async fn main() -> Result<()> {
    Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(LevelFilter::Info)
        .chain(io::stdout())
        .chain(log_file("./ast.log").expect("No permission to write to the current directory."))
        .apply()
        .expect("Failed to dispatch Fern logger!");

    let mut rotator = Rotator::new();
    rotator.zero();

    let rotctld_port = "4533";
    let listener = TcpListener::bind(format!("0.0.0.0:{}", rotctld_port)).await?;

    loop {
        let (mut socket, _) = listener.accept().await?;
        info!("Socket now listening.");

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            let mut handler = ActionHandler::new(&mut rotator);

            loop {
                let n = match socket.read(&mut buf).await {
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        warn!("Failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                let response = str::from_utf8(&buf[0..n]).unwrap();
                info!("Response: {:?}", response);

                // This will never actually error, since it returns Infallible
                let ret: String = handler
                    .handle_message(Message::from_str(response).unwrap())
                    .to_owned();

                info!("Return: {:?}", ret);

                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    warn!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}

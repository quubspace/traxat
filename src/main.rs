use std::{io, str};

use anyhow::Result;

use chrono;
use fern::{log_file, Dispatch};
use log::{info, warn, LevelFilter};

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[derive(Debug)]
struct ActionHandler<'a> {
    rotator: &'a mut Rotator,
}

impl<'a> ActionHandler<'a> {
    pub fn new(rotator: &'a mut Rotator) -> Self {
        Self { rotator }
    }

    pub fn handle_p_set(&mut self, azimuth: i32, elevation: i32) {
        self.rotator.xt = azimuth;
        self.rotator.yt = elevation;

        info!("Set to {}:{}", azimuth, elevation);

        self.rotator.mv();
    }

    pub fn handle_p_get(&self) -> Vec<(String, String)> {
        vec![(
            format!("{}", self.rotator.az),
            format!("{}", self.rotator.ele),
        )]
    }

    pub fn close_connection(&self) {
        exit(0);
    }

    pub fn handle_message(&self, cmd: &str) {
        "\n{}{}{}\n"
        // let actions = HashMap::from([
        //     ("P", self.handle_p_set(azimuth, elevation)),
        //     ("p", 24),
        //     ("q", 12),
        // ]);
    }
}

#[derive(Debug)]
struct Rotator {
    ele: i32,
    az: i32,
    xc: i32,
    yc: i32,
    yt: i32,
    xt: i32,
}

impl Rotator {
    pub fn new() -> Rotator {
        Rotator {
            ele: 20 as i32,
            az: 0 as i32,
            xc: 6.666 as i32,
            yc: 1.458 as i32,
            yt: 20 as i32,
            xt: 0 as i32,
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
        .level(LevelFilter::Error)
        .chain(io::stdout())
        .chain(log_file("./ast.log").expect("No permission to write to the current directory."))
        .apply()
        .expect("Failed to dispatch Fern logger!");

    let mut rotator = Rotator::new();
    rotator.zero();

    let _handler = ActionHandler {
        rotator: &mut rotator,
    };

    let rotctld_port = "4533";
    let listener = TcpListener::bind(format!("0.0.0.0:{}", rotctld_port)).await?;

    loop {
        let (mut socket, _) = listener.accept().await?;
        info!("Socket now listening.");

        tokio::spawn(async move {
            let mut buf = [0; 1024];

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
                println!("{:?}", response);

                // let ret = a.handle_message()

                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    warn!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}

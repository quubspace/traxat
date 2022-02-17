use std::str;

use anyhow::Result;

use chrono;
use fern::{log_file, Dispatch};
use log::{info, warn, LevelFilter};

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[derive(Debug)]
struct ActionHandler {
    rotator: Rotator,
}

impl ActionHandler {
    pub fn new(rotator: Rotator) {
        ActionHandler { rotator }
    }

    pub fn handle_p_set() {}

    pub fn handle_p_get() {}

    pub fn close_connection() {}

    pub fn handle_message() {}
}

#[derive(Debug)]
struct Rotator {
    ele: u32,
    az: u32,
    xc: u32,
    yc: u32,
    yt: u32,
    xt: u32,
}

impl Rotator {
    pub fn new() -> Rotator {
        Rotator {
            ele: 20,
            az: 0,
            xc: 6.666,
            yc: 1.458,
            yt: 20,
            xt: 0,
        }
    }

    pub fn mv() {}

    pub fn ul() {}

    pub fn ur() {}

    pub fn dl() {}

    pub fn dr() {}

    pub fn max() {}

    pub fn center() {}

    pub fn zero() {}
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
        .chain(
            log_file("data/spotsync.log")
                .expect("No permission to write to the current directory."),
        )
        .apply()
        .expect("Failed to dispatch Fern logger!");

    let rotator = Rotator::new();
    rotator.zero();

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

                let ret = ActionHandler { rotator };

                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    warn!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}

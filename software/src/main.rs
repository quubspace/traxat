use std::{
    io::{self, Read, Write},
    net::TcpListener,
    str::{self, FromStr},
};

use anyhow::Result;

use fern::{
    colors::{Color, ColoredLevelConfig},
    log_file, Dispatch,
};
use log::{debug, info, warn, LevelFilter};

// Imports items from other crates
use handler::ActionHandler;
use message::Message;
use rotator::Rotator;

// Organizes code into modules
mod handler;
mod message;
mod rotator;

/// Main function to run the traxat software
fn main() -> Result<()> {
    // Initializes logging with Criterion
    init_logging();

    // Create a new rotator object.
    let mut rotator = Rotator::new();

    // Sets the motor position to zero.
    rotator.zero()?;

    // Opens a TCP listener on port 4533
    let rotctld_port = "4533";
    let listener = TcpListener::bind(format!("0.0.0.0:{}", rotctld_port))?;

    info!("AST is now ready to connect to rotctld.");

    // Listen for rotctld commands and execute them coming in.
    for stream in listener.incoming() {
        let mut stream = stream?;
        let mut buf = [0; 1024];

        // Read data coming in.
        loop {
            let n = match stream.read(&mut buf) {
                Ok(n) if n == 0 => break,
                Ok(n) => n,
                Err(e) => {
                    warn!("Failed to read from stream; err = {:?}", e);
                    break;
                }
            };

            let response = str::from_utf8(&buf[0..n]).unwrap();
            debug!("Response from rotctld: {:?}", response);

            // This will never actually error, since it returns Infallible
            let ret: String = ActionHandler::new(&mut rotator)
                .handle_message(Message::from_str(response).unwrap())?;

            // Quit command to close connection.
            if ret == "rotctld_quit" {
                warn!("Closing connection, rotctld sent quit!");
                rotator.zero()?;
                break;
            }

            // Sending
            debug!("Send to rotctld: {:?}", ret);

            if let Err(e) = stream.write_all(&buf[0..n]) {
                warn!("failed to write to stream; err = {:?}", e);
                break;
            }
        }
    }
    Ok(())
}
/// Initializes logging.
///
/// Changes the colors and format of the logging.
fn init_logging() {
    let colors_line = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::White)
        .debug(Color::White)
        .trace(Color::BrightBlack);

    let colors_level = colors_line.info(Color::Green);

    Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{color_line}[{date}][{target}][{level}{color_line}] {message}\x1B[0m",
                color_line = format_args!(
                    "\x1B[{}m",
                    colors_line.get_color(&record.level()).to_fg_str()
                ),
                date = chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                target = record.target(),
                level = colors_level.color(record.level()),
                message = message,
            ));
        })
        .level(LevelFilter::Debug)
        .level_for("pretty_colored", LevelFilter::Trace)
        .chain(io::stdout())
        .chain(log_file("./ast.log").expect("No permission to write to the current directory."))
        .apply()
        .expect("Failed to dispatch Fern logger!");

    debug!("Logging initialization complete.");
}

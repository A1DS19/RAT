mod utils;
use futures_util::{SinkExt, StreamExt};
use tokio::{
    io::{self, AsyncBufReadExt},
    net::TcpListener,
    sync::broadcast,
};
use tokio_tungstenite::accept_async;
use utils::get_public_ip::get_public_ip;

#[tokio::main]
async fn main() {
    // let ip = get_public_ip().await.unwrap();

    // Use local IP for testing
    let addr = format!("{}:8080", "127.0.0.1");

    println!("addr: {}", addr);

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

    tokio::signal::ctrl_c()
        .await
        .expect("Failed to listen for Ctrl+C");
    println!("Shutting down the server.");

    drop(tx);
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
        let msg = msg.expect("Error receiving message");
        println!("Received response: {}", msg.to_text().unwrap());
    }
}

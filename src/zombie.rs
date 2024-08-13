use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::connect_async;
use url::Url;

#[tokio::main]
async fn main() {
    let url = Url::parse("ws://127.0.0.1:8080").unwrap();
    let (ws_stream, _) = connect_async(url.as_str())
        .await
        .expect("Failed to connect");

    println!("Connected to the C2 server!");

    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    while let Some(msg) = ws_receiver.next().await {
        let command = match msg {
            Ok(m) => m.to_text().unwrap().to_string(),
            Err(e) => {
                eprintln!("Error receiving command: {}", e);
                break;
            }
        };

        println!("Received command: {}", command);

        let response = format!("Processed command: {}", command);

        if let Err(e) = ws_sender
            .send(tokio_tungstenite::tungstenite::Message::Text(response))
            .await
        {
            eprintln!("Failed to send response: {}", e);
        }
    }
}

mod constants;

use std::time::Duration;

use constants::localhost::{LOCALHOST_ADDRESS, LOCALHOST_PORT};
use futures_util::FutureExt;
use rust_socketio::{
    asynchronous::{Client, ClientBuilder},
    Payload,
};

#[tokio::main]
async fn main() {
    let url: String = format!("http://{}:{}", LOCALHOST_ADDRESS, LOCALHOST_PORT);

    ClientBuilder::new(url)
        .namespace("/")
        .on("auth", |payload: Payload, _: Client| {
            async move {
                println!("Auth event received: {:?}", payload);
            }
            .boxed()
        })
        .on("command-to-zombie", |payload, socket: Client| {
            async move {
                println!("Command back received: {:?}", payload);

                if let Err(e) = socket.emit("command-response-from-zombie", payload).await {
                    println!("Error sending command response: {:?}", e);
                }
            }
            .boxed()
        })
        .connect()
        .await
        .expect("Connection failed");

    println!("Connected to server");

    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

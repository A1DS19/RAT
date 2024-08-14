use std::time::Duration;

use rust_socketio::{ClientBuilder, Event, Payload};
use serde_json::json;

mod constants;
use constants::localhost::{LOCALHOST_ADDRESS, LOCALHOST_PORT};

fn main() {
    let url = format!("http://{}:{}", LOCALHOST_ADDRESS, LOCALHOST_PORT);

    let socket = ClientBuilder::new(url)
        .namespace("/")
        .on("auth", |payload, _| {
            println!("Received auth event: {:?}", payload);
        })
        .on("command-back", |payload, _| {
            println!("Received command-back event: {:?}", payload);
        })
        .connect()
        .expect("Connection failed");

    let command_data = json!({
        "action": "example_command",
        "data": "some data"
    });

    socket
        .emit("command", command_data)
        .expect("Failed to emit command");

    loop {
        std::thread::sleep(Duration::from_secs(1));
    }
}

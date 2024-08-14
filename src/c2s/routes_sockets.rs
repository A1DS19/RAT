use serde_json::Value;
use socketioxide::{
    extract::{Data, SocketRef},
    SocketIo,
};

use super::{services::on_connect::on_connect, types::client_store::ClientStore};

pub fn routes_sockets(io: SocketIo, clients: ClientStore) {
    io.ns("/", |socket: SocketRef, Data(data): Data<Value>| {
        on_connect(socket, Data(data), clients);
    });
}

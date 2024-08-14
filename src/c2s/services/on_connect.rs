use serde_json::Value;
use socketioxide::extract::{Bin, Data, SocketRef};
use tracing::info;

pub fn on_connect(socket: SocketRef, Data(data): Data<Value>) {
    info!("Socket.IO connected: {:?} {:?}", socket.ns(), socket.id);
    socket.emit("auth", data).ok();

    socket.on(
        "command",
        |socket: SocketRef, Data::<Value>(data), Bin(bin)| {
            info!("Received event: {:?} {:?}", data, bin);
            socket.bin(bin).emit("command-back", data).ok();
        },
    );
}

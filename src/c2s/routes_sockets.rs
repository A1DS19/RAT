use socketioxide::SocketIo;

use super::services::on_connect::on_connect;

pub fn routes_sockets(io: SocketIo) {
    io.ns("/", on_connect);
}

mod c2s;
mod constants;
mod utils;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use c2s::{
    routes_http::routes_http, routes_sockets::routes_sockets, types::client_store::ClientStore,
};
use constants::localhost::{LOCALHOST_ADDRESS_LISTENER, LOCALHOST_PORT};
use socketioxide::SocketIo;
use tracing::info;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::new())?;

    let (layer, io) = SocketIo::new_layer();

    let clients: ClientStore = Arc::new(Mutex::new(HashMap::new()));

    routes_sockets(io, clients);
    let app = routes_http(layer);

    info!(
        "Starting server {} on port {}",
        LOCALHOST_ADDRESS_LISTENER, LOCALHOST_PORT
    );

    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", LOCALHOST_ADDRESS_LISTENER, LOCALHOST_PORT))
            .await
            .unwrap();

    axum::serve(listener, app).await.unwrap();

    Ok(())
}

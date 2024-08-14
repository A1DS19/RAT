mod c2s;
mod constants;

use c2s::{routes_http::routes_http, routes_sockets::routes_sockets};
use constants::localhost::{LOCALHOST_ADDRESS, LOCALHOST_PORT};
use socketioxide::SocketIo;
use tracing::info;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::new())?;

    let (layer, io) = SocketIo::new_layer();

    routes_sockets(io);
    let app = routes_http(layer);

    info!(
        "Starting server {} on port {}",
        LOCALHOST_ADDRESS, LOCALHOST_PORT
    );

    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", LOCALHOST_ADDRESS, LOCALHOST_PORT))
            .await
            .unwrap();

    axum::serve(listener, app).await.unwrap();

    Ok(())
}

use axum::{routing::post, Router};
use socketioxide::layer::SocketIoLayer;

pub fn routes_http(layer: SocketIoLayer) -> Router {
    axum::Router::new()
        .route("/command", post(|| async { "Hello world" }))
        .layer(layer)
}

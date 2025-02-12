mod routes;
mod manipulations;
mod errors;

use axum::{
    Router,
    routing::post,
};
use std::net::SocketAddr;

use crate::routes::*;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/grayscale", post(grayscale_handler))
        .route("/invert", post(invert_handler))
        .route("/rotate90", post(rotate90_handler))
        // For blur/brightness/contrast we need query params
        .route("/blur", post(blur_handler))
        .route("/brightness", post(brightness_handler))
        .route("/contrast", post(contrast_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server listening on http://{}", addr);

    if let Err(e) = axum::Server::bind(&addr).serve(app.into_make_service()).await {
        eprintln!("Server error: {}", e);
    }
}

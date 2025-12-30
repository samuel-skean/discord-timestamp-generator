mod frontend;

use axum::{Router, response::Redirect, routing::*};
use frontend::*;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { Redirect::permanent("/app") }))
        .route("/app", get(app))
        .fallback_service(ServeDir::new("assets"));

    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Unable to bind.");
    axum::serve(listener, app)
        .await
        .expect("Failed to initialize.");
}

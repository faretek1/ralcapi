use axum::{
    Json, Router,
    extract::Path,
    http::StatusCode,
    routing::{get, post},
}; // copied from https://github.com/tokio-rs/axum#usage-example

use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    // initialize tracing
    // tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        .route("/", get(root))
        .route("/add/{a}/{b}", get(add_handler));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> axum::response::Redirect {
    // &'static str
    axum::response::Redirect::to("https://github.com/faretek1/ralcapi")
}

async fn add_handler(Path((a, b)): Path<(f64, f64)>) -> String {
    format!("{} + {} = {}", a, b, a + b)
}

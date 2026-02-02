use axum::{Router, extract::Path, routing::get};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/add/{a}/{b}", get(|p| calc(p, "+", |a, b| a + b)))
        .route("/sub/{a}/{b}", get(|p| calc(p, "-", |a, b| a - b)))
        .route("/mul/{a}/{b}", get(|p| calc(p, "*", |a, b| a * b)))
        .route("/div/{a}/{b}", get(|p| calc(p, "/", |a, b| a / b)));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> axum::response::Redirect {
    axum::response::Redirect::to("https://github.com/faretek1/ralcapi")
}

async fn calc(
    Path((a, b)): Path<(f64, f64)>,
    sym: &'static str,
    op: impl Fn(f64, f64) -> f64,
) -> String {
    format!("{} {} {} = {}", a, sym, b, op(a, b))
}

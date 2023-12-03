use axum::{
    routing::get,
    Router,
};
use log::info;

#[tokio::main]
async fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/foo", get(get_foo).post(post_foo))
        .route("/foo/bar", get(foo_bar));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_foo() {
    info!("Got foo")
}
async fn post_foo() {
    info!("Got foo")
}
async fn foo_bar() {
    info!("Got foo bar")
}
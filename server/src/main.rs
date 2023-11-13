use axum::Router;
use server::{catfacts::routes_catfacts, catimages::routes_catimages};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .merge(routes_catfacts())
        .merge(routes_catimages());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

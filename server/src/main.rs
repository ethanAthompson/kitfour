use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use server::{catfacts::routes_catfacts, catimages::routes_catimages};
use std::net::SocketAddr;
use tower_http::services::{ServeDir, ServeFile};

#[tokio::main]
async fn main() {
    let serve_html = ServeDir::new("public").not_found_service(ServeFile::new("public/index.html"));
    let app = Router::new()
        // Displays index.html as soon as you load the backend
        .nest_service("/public", serve_html.clone())
        .fallback_service(serve_html)
        // These are the other routes that i've merged
        .merge(routes_catfacts())
        .merge(routes_catimages());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

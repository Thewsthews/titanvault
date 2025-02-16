use axum::{routing::post, Router};
use std::net::SocketAddr;
use tokio;

mod routes

#[tokio::main]
async fn main(){
    let app= routes::create_router();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

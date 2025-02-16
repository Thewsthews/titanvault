use std::net::SocketAddr;
use tokio;
use titanvault_backend::routes::create_router;

mod routes;
mod wallet;
pub mod handlers;

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

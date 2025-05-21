use std::net::SocketAddr;
use tokio;
use crate::routes::create_router;

mod routes;
mod wallet;
mod transactions;
pub mod handlers;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = create_router();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .map_err(|e| format!("Server failed to start: {}", e))?;
    Ok(())
}
use axum::{routing::post, Router};
use crate::handlers::{create_wallet, sign_transactions};

pub fn create_router()-> Router{
    Router::new()
        .route("/wallet", post(create_wallet))
        .route("/sign", post(sign_transactions))
}

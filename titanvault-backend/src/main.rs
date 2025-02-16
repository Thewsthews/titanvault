use axum::{routing::post, Router, Json};
use ethers::signers::{coins_bip39::mnemonic, LocalWallet, MnemonicBuilder};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[tokio::main]
async fn main(){
    let app = Router::new()
    .route("/create_wallet", post(create_wallet))
    .route("/sign_transaction",post(sign_transaction));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Serialize)]
struct WalletResponse{
    address: String,
    private_key: String,
    mnemonic: String,
}
async fn create_wallet() -> Json<WalletResponse>{
    let mnemonic = MnemonicBuilder::default().word_count(12).build().unwrap();
    let wallet = mnemonic.clone().build().unwrap();
}
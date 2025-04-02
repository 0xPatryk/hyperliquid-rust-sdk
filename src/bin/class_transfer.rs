use alloy::{
    primitives::Address,
    signer::{LocalWallet, Signer},
};
use hyperliquid_rust_sdk::{BaseUrl, ExchangeClient};
use log::info;

#[tokio::main]
async fn main() {
    env_logger::init();
    // Key was randomly generated for testing and shouldn't be used with any real funds
    let wallet = LocalWallet::from_bytes(
        &hex::decode("e908f86dbb4d55ac876378565aafeabc187f6690f046459397b17d9b9a19688e").unwrap(),
    )
    .unwrap();

    let exchange_client = ExchangeClient::new(None, wallet, Some(BaseUrl::Testnet), None, None)
        .await
        .unwrap();

    let usdc = 1.0; // 1 USD
    let to_perp = false;

    let res = exchange_client
        .class_transfer(usdc, to_perp, None)
        .await
        .unwrap();
    info!("Class transfer result: {res:?}");
}

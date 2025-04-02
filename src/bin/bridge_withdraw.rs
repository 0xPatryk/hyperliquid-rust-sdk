use alloy::{
    primitives::Address,
    signer::{LocalWallet, Signer},
};
use hyperliquid_rust_sdk::{BaseUrl, ExchangeClient};
use log::info;
use std::str::FromStr;

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

    let usd = "5"; // 5 USD
    let destination = Address::from_str("0x0D1d9635D0640821d15e323ac8AdADfA9c111414").unwrap();

    let res = exchange_client
        .withdraw_from_bridge(usd, &destination.to_string(), None)
        .await
        .unwrap();
    info!("Withdraw from bridge result: {res:?}");
}

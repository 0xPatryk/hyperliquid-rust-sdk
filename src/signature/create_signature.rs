use alloy::{
    k256::{
        ecdsa::{RecoveryId, Signature as K256Signature},
        Secp256k1,
    },
    primitives::{FixedBytes, U256},
    signer::{LocalWallet, Signer},
    sol_types::eip712::Eip712,
};

use crate::{prelude::*, proxy_digest::Sha256Proxy, signature::agent::l1, Error};

pub(crate) fn sign_l1_action(
    wallet: &LocalWallet,
    connection_id: FixedBytes<32>,
    is_mainnet: bool,
) -> Result<Signature> {
    let source = if is_mainnet { "a" } else { "b" }.to_string();
    sign_typed_data(
        &l1::Agent {
            source,
            connection_id,
        },
        wallet,
    )
}

pub(crate) fn sign_typed_data<T: Eip712>(payload: &T, wallet: &LocalWallet) -> Result<Signature> {
    let hash = payload.eip712_hash();
    sign_hash(hash, wallet)
}

fn sign_hash(hash: FixedBytes<32>, wallet: &LocalWallet) -> Result<Signature> {
    let msg = alloy::k256::ecdsa::SigningKey::from_bytes(wallet.signer().to_bytes().as_ref())
        .map_err(|e| Error::SignatureFailure(e.to_string()))?;

    let (sig, rec_id) = msg
        .sign_digest_recoverable(Sha256Proxy::from(hash))
        .map_err(|e| Error::SignatureFailure(e.to_string()))?;

    let v = u8::from(rec_id) as u64 + 27;
    let r = U256::from_be_bytes(sig.r().to_bytes());
    let s = U256::from_be_bytes(sig.s().to_bytes());

    Ok(Signature { r, s, v })
}

#[derive(Debug, Clone)]
pub struct Signature {
    pub r: U256,
    pub s: U256,
    pub v: u64,
}

impl ToString for Signature {
    fn to_string(&self) -> String {
        format!(
            "{}{}{}",
            hex::encode(self.r.to_be_bytes::<32>()),
            hex::encode(self.s.to_be_bytes::<32>()),
            hex::encode([(self.v - 27) as u8])
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{UsdSend, Withdraw3};
    use std::str::FromStr;

    fn get_wallet() -> Result<LocalWallet> {
        let priv_key = "e908f86dbb4d55ac876378565aafeabc187f6690f046459397b17d9b9a19688e";
        LocalWallet::from_bytes(&hex::decode(priv_key).map_err(|e| Error::Wallet(e.to_string()))?)
            .map_err(|e| Error::Wallet(e.to_string()))
    }

    #[test]
    fn test_sign_l1_action() -> Result<()> {
        let wallet = get_wallet()?;
        let connection_id = FixedBytes::<32>::from_str(
            "0xde6c4037798a4434ca03cd05f00e3b803126221375cd1e7eaaaf041768be06eb",
        )
        .map_err(|e| Error::GenericParse(e.to_string()))?;

        let expected_mainnet_sig = "fa8a41f6a3fa728206df80801a83bcbfbab08649cd34d9c0bfba7c7b2f99340f53a00226604567b98a1492803190d65a201d6805e5831b7044f17fd530aec7841c";
        assert_eq!(
            sign_l1_action(&wallet, connection_id, true)?.to_string(),
            expected_mainnet_sig
        );
        let expected_testnet_sig = "1713c0fc661b792a50e8ffdd59b637b1ed172d9a3aa4d801d9d88646710fb74b33959f4d075a7ccbec9f2374a6da21ffa4448d58d0413a0d335775f680a881431c";
        assert_eq!(
            sign_l1_action(&wallet, connection_id, false)?.to_string(),
            expected_testnet_sig
        );
        Ok(())
    }

    #[test]
    fn test_sign_usd_transfer_action() -> Result<()> {
        let wallet = get_wallet()?;

        let usd_send = UsdSend {
            signature_chain_id: 421614.into(),
            hyperliquid_chain: "Testnet".to_string(),
            destination: "0x0D1d9635D0640821d15e323ac8AdADfA9c111414".to_string(),
            amount: "1".to_string(),
            time: 1690393044548,
        };

        let expected_sig = "214d507bbdaebba52fa60928f904a8b2df73673e3baba6133d66fe846c7ef70451e82453a6d8db124e7ed6e60fa00d4b7c46e4d96cb2bd61fd81b6e8953cc9d21b";
        assert_eq!(
            sign_typed_data(&usd_send, &wallet)?.to_string(),
            expected_sig
        );
        Ok(())
    }

    #[test]
    fn test_sign_withdraw_from_bridge_action() -> Result<()> {
        let wallet = get_wallet()?;

        let usd_send = Withdraw3 {
            signature_chain_id: 421614.into(),
            hyperliquid_chain: "Testnet".to_string(),
            destination: "0x0D1d9635D0640821d15e323ac8AdADfA9c111414".to_string(),
            amount: "1".to_string(),
            time: 1690393044548,
        };

        let expected_sig = "b3172e33d2262dac2b4cb135ce3c167fda55dafa6c62213564ab728b9f9ba76b769a938e9f6d603dae7154c83bf5a4c3ebab81779dc2db25463a3ed663c82ae41c";
        assert_eq!(
            sign_typed_data(&usd_send, &wallet)?.to_string(),
            expected_sig
        );
        Ok(())
    }
}

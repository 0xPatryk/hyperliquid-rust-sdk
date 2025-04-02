use crate::exchange::{cancel::CancelRequest, modify::ModifyRequest, order::OrderRequest};
use alloy::{
    primitives::{Address, FixedBytes, U256},
    sol_types::{
        eip712::{self, Eip712, Eip712Domain, Eip712Error, Type},
        SolType,
    },
};
use serde::{Deserialize, Serialize};

use super::{cancel::CancelRequestCloid, BuilderInfo};

pub(crate) const HYPERLIQUID_EIP_PREFIX: &str = "HyperliquidTransaction:";

fn eip_712_domain(chain_id: U256) -> Eip712Domain {
    Eip712Domain {
        name: "HyperliquidSignTransaction".to_string(),
        version: "1".to_string(),
        chain_id,
        verifying_contract: Address::from_str("0x0000000000000000000000000000000000000000")
            .unwrap(),
        salt: None,
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UsdSend {
    pub signature_chain_id: U256,
    pub hyperliquid_chain: String,
    pub destination: String,
    pub amount: String,
    pub time: u64,
}

impl Eip712 for UsdSend {
    const NAME: &'static str = "HyperliquidSignTransaction";
    const VERSION: &'static str = "1";

    fn eip712_domain(&self) -> Eip712Domain {
        eip_712_domain(self.signature_chain_id)
    }

    fn eip712_type() -> Type {
        Type::struct_type(
            format!("{HYPERLIQUID_EIP_PREFIX}UsdSend"),
            &[
                ("hyperliquidChain", Type::String),
                ("destination", Type::String),
                ("amount", Type::String),
                ("time", Type::Uint(64)),
            ],
        )
    }

    fn eip712_hash(&self) -> FixedBytes<32> {
        let mut hasher = eip712::Eip712Hasher::new(
            Self::NAME,
            Self::VERSION,
            self.signature_chain_id.as_u64(),
            &self.eip712_domain().verifying_contract,
        );
        hasher.add_type("UsdSend", Self::eip712_type());
        hasher.hash_struct(
            "UsdSend",
            &[
                ("hyperliquidChain", self.hyperliquid_chain.as_str().into()),
                ("destination", self.destination.as_str().into()),
                ("amount", self.amount.as_str().into()),
                ("time", self.time.into()),
            ],
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateLeverage {
    pub asset: u32,
    pub is_cross: bool,
    pub leverage: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateIsolatedMargin {
    pub asset: u32,
    pub is_buy: bool,
    pub ntli: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BulkOrder {
    pub orders: Vec<OrderRequest>,
    pub grouping: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub builder: Option<BuilderInfo>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BulkCancel {
    pub cancels: Vec<CancelRequest>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BulkModify {
    pub modifies: Vec<ModifyRequest>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BulkCancelCloid {
    pub cancels: Vec<CancelRequestCloid>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ApproveAgent {
    pub signature_chain_id: U256,
    pub hyperliquid_chain: String,
    pub agent_address: Address,
    pub agent_name: Option<String>,
    pub nonce: u64,
}

impl Eip712 for ApproveAgent {
    const NAME: &'static str = "HyperliquidSignTransaction";
    const VERSION: &'static str = "1";

    fn eip712_domain(&self) -> Eip712Domain {
        eip_712_domain(self.signature_chain_id)
    }

    fn eip712_type() -> Type {
        Type::struct_type(
            format!("{HYPERLIQUID_EIP_PREFIX}ApproveAgent"),
            &[
                ("hyperliquidChain", Type::String),
                ("agentAddress", Type::Address),
                ("agentName", Type::String),
                ("nonce", Type::Uint(64)),
            ],
        )
    }

    fn eip712_hash(&self) -> FixedBytes<32> {
        let mut hasher = eip712::Eip712Hasher::new(
            Self::NAME,
            Self::VERSION,
            self.signature_chain_id.as_u64(),
            &self.eip712_domain().verifying_contract,
        );
        hasher.add_type("ApproveAgent", Self::eip712_type());
        hasher.hash_struct(
            "ApproveAgent",
            &[
                ("hyperliquidChain", self.hyperliquid_chain.as_str().into()),
                ("agentAddress", self.agent_address.into()),
                (
                    "agentName",
                    self.agent_name.as_deref().unwrap_or_default().into(),
                ),
                ("nonce", self.nonce.into()),
            ],
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Withdraw3 {
    pub hyperliquid_chain: String,
    pub signature_chain_id: U256,
    pub amount: String,
    pub time: u64,
    pub destination: String,
}

impl Eip712 for Withdraw3 {
    const NAME: &'static str = "HyperliquidSignTransaction";
    const VERSION: &'static str = "1";

    fn eip712_domain(&self) -> Eip712Domain {
        eip_712_domain(self.signature_chain_id)
    }

    fn eip712_type() -> Type {
        Type::struct_type(
            format!("{HYPERLIQUID_EIP_PREFIX}Withdraw"),
            &[
                ("hyperliquidChain", Type::String),
                ("destination", Type::String),
                ("amount", Type::String),
                ("time", Type::Uint(64)),
            ],
        )
    }

    fn eip712_hash(&self) -> FixedBytes<32> {
        let mut hasher = eip712::Eip712Hasher::new(
            Self::NAME,
            Self::VERSION,
            self.signature_chain_id.as_u64(),
            &self.eip712_domain().verifying_contract,
        );
        hasher.add_type("Withdraw", Self::eip712_type());
        hasher.hash_struct(
            "Withdraw",
            &[
                ("hyperliquidChain", self.hyperliquid_chain.as_str().into()),
                ("destination", self.destination.as_str().into()),
                ("amount", self.amount.as_str().into()),
                ("time", self.time.into()),
            ],
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SpotSend {
    pub hyperliquid_chain: String,
    pub signature_chain_id: U256,
    pub destination: String,
    pub token: String,
    pub amount: String,
    pub time: u64,
}

impl Eip712 for SpotSend {
    const NAME: &'static str = "HyperliquidSignTransaction";
    const VERSION: &'static str = "1";

    fn eip712_domain(&self) -> Eip712Domain {
        eip_712_domain(self.signature_chain_id)
    }

    fn eip712_type() -> Type {
        Type::struct_type(
            format!("{HYPERLIQUID_EIP_PREFIX}SpotSend"),
            &[
                ("hyperliquidChain", Type::String),
                ("destination", Type::String),
                ("token", Type::String),
                ("amount", Type::String),
                ("time", Type::Uint(64)),
            ],
        )
    }

    fn eip712_hash(&self) -> FixedBytes<32> {
        let mut hasher = eip712::Eip712Hasher::new(
            Self::NAME,
            Self::VERSION,
            self.signature_chain_id.as_u64(),
            &self.eip712_domain().verifying_contract,
        );
        hasher.add_type("SpotSend", Self::eip712_type());
        hasher.hash_struct(
            "SpotSend",
            &[
                ("hyperliquidChain", self.hyperliquid_chain.as_str().into()),
                ("destination", self.destination.as_str().into()),
                ("token", self.token.as_str().into()),
                ("amount", self.amount.as_str().into()),
                ("time", self.time.into()),
            ],
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SpotUser {
    pub class_transfer: ClassTransfer,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ClassTransfer {
    pub usdc: u64,
    pub to_perp: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VaultTransfer {
    pub vault_address: Address,
    pub is_deposit: bool,
    pub usd: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetReferrer {
    pub code: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ApproveBuilderFee {
    pub max_fee_rate: String,
    pub builder: String,
    pub nonce: u64,
    pub signature_chain_id: U256,
    pub hyperliquid_chain: String,
}

impl Eip712 for ApproveBuilderFee {
    const NAME: &'static str = "HyperliquidSignTransaction";
    const VERSION: &'static str = "1";

    fn eip712_domain(&self) -> Eip712Domain {
        eip_712_domain(self.signature_chain_id)
    }

    fn eip712_type() -> Type {
        Type::struct_type(
            format!("{HYPERLIQUID_EIP_PREFIX}ApproveBuilderFee"),
            &[
                ("hyperliquidChain", Type::String),
                ("builder", Type::String),
                ("maxFeeRate", Type::String),
                ("nonce", Type::Uint(64)),
            ],
        )
    }

    fn eip712_hash(&self) -> FixedBytes<32> {
        let mut hasher = eip712::Eip712Hasher::new(
            Self::NAME,
            Self::VERSION,
            self.signature_chain_id.as_u64(),
            &self.eip712_domain().verifying_contract,
        );
        hasher.add_type("ApproveBuilderFee", Self::eip712_type());
        hasher.hash_struct(
            "ApproveBuilderFee",
            &[
                ("hyperliquidChain", self.hyperliquid_chain.as_str().into()),
                ("builder", self.builder.as_str().into()),
                ("maxFeeRate", self.max_fee_rate.as_str().into()),
                ("nonce", self.nonce.into()),
            ],
        )
    }
}

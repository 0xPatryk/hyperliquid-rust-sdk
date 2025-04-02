use alloy::{
    primitives::FixedBytes,
    sol_types::{eip712::Eip712, SolType},
};

pub(crate) mod l1 {
    use super::*;

    #[derive(Debug, Clone)]
    pub(crate) struct Agent {
        pub(crate) source: String,
        pub(crate) connection_id: FixedBytes<32>,
    }

    impl Eip712 for Agent {
        const NAME: &'static str = "Exchange";
        const VERSION: &'static str = "1";
        const CHAIN_ID: u64 = 1337;
        const VERIFYING_CONTRACT: &'static str = "0x0000000000000000000000000000000000000000";

        fn eip712_type() -> alloy::sol_types::eip712::Type {
            alloy::sol_types::eip712::Type::struct_type(
                "Agent",
                &[
                    ("source", alloy::sol_types::eip712::Type::String),
                    (
                        "connection_id",
                        alloy::sol_types::eip712::Type::FixedBytes(32),
                    ),
                ],
            )
        }

        fn eip712_hash(&self) -> FixedBytes<32> {
            let mut hasher = alloy::sol_types::eip712::Eip712Hasher::new(
                Self::NAME,
                Self::VERSION,
                Self::CHAIN_ID,
                Self::VERIFYING_CONTRACT,
            );
            hasher.add_type("Agent", Self::eip712_type());
            hasher.hash_struct(
                "Agent",
                &[
                    ("source", self.source.as_str().into()),
                    ("connection_id", self.connection_id.into()),
                ],
            )
        }
    }
}

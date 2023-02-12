use ethbind_rust_macros::contract;

#[allow(unused)]
mod mock {
    pub struct Ops;

    pub struct Client;

    impl Client {
        pub fn rlp_encoder(&mut self) -> RlpEncoder {
            RlpEncoder::default()
        }

        pub fn deploy_contract(
            &self,
            encoder: RlpEncoder,
            deploy_data: &str,
        ) -> anyhow::Result<TransactionReceipt> {
            Ok(Default::default())
        }
    }

    pub struct Address;

    impl Address {
        pub fn encode(&self, encoder: &mut RlpEncoder) {}
    }

    #[derive(Debug, Default)]
    pub struct TransactionReceipt;

    impl TransactionReceipt {
        pub fn encode(&self, encoder: &mut RlpEncoder) {}
    }

    #[derive(Debug, Default)]
    pub struct RlpEncoder;
}

// The contract name is automatically detected when loading hardhat artifact
contract!("tests/mapping.json", "tests/abi.json");

#[test]
fn test_gen() {}

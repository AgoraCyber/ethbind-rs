use ethbind_rust_macros::contract;

#[allow(unused)]
mod mock {
    pub use serde::{Deserialize, Serialize};

    #[derive(Default)]
    pub struct Ops;

    pub struct Client;

    impl Client {
        pub async fn deploy_contract(
            &self,
            contract_name: &str,
            encoder: String,
            deploy_data: &str,
            ops: Ops,
        ) -> anyhow::Result<Address> {
            Ok(Default::default())
        }

        pub async fn eth_call(
            &self,
            method_name: &str,
            address: &Address,
            encoder: String,
        ) -> anyhow::Result<String> {
            Ok(Default::default())
        }

        pub async fn send_raw_transaction(
            &self,
            method_name: &str,
            address: &Address,
            encoder: String,
            ops: Ops,
        ) -> anyhow::Result<TransactionReceipt> {
            Ok(Default::default())
        }
    }

    pub fn abi_encode<T: Serialize>(value: T) -> anyhow::Result<String> {
        unimplemented!()
    }

    pub fn abi_decode<'de, T: Deserialize<'de>>(data: String) -> anyhow::Result<T> {
        unimplemented!()
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    pub struct Address;

    #[derive(Debug, Default, Serialize, Deserialize)]
    pub struct TransactionReceipt;

    #[derive(Debug, Default, Serialize, Deserialize)]
    pub struct Int<const SIGN: bool, const LEN: usize>(bool);
}

contract!("tests/mapping.json", "tests/abi.json");

#[test]
fn test_gen() {}

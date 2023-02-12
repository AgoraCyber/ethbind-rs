use ethbind_rust_macros::contract;

#[allow(unused)]
mod mock {
    pub struct Ops;

    pub struct Client;

    impl Client {
        pub fn rlp_encoder(&self) -> RlpEncoder {
            RlpEncoder::default()
        }

        pub async fn deploy_contract(
            &self,
            encoder: RlpEncoder,
            deploy_data: &str,
            ops: Ops,
        ) -> anyhow::Result<Address> {
            Ok(Default::default())
        }

        pub async fn eth_call(
            &self,
            address: &Address,
            encoder: RlpEncoder,
        ) -> anyhow::Result<RlpDecoder> {
            Ok(Default::default())
        }

        pub async fn send_raw_transaction(
            &self,
            address: &Address,
            encoder: RlpEncoder,
            ops: Ops,
        ) -> anyhow::Result<TransactionReceipt> {
            Ok(Default::default())
        }
    }

    #[derive(Debug, Default)]
    pub struct Address;

    impl Address {
        pub fn encode(&self, encoder: &mut RlpEncoder) {}

        pub fn decode(decoder: &mut RlpDecoder) -> Self {
            Default::default()
        }
    }

    #[derive(Debug, Default)]
    pub struct TransactionReceipt;

    impl TransactionReceipt {
        pub fn encode(&self, encoder: &mut RlpEncoder) {}

        pub fn decode(decoder: &mut RlpDecoder) -> Self {
            Default::default()
        }
    }

    #[derive(Debug, Default)]
    pub struct RlpEncoder;

    #[derive(Debug, Default)]
    pub struct RlpDecoder;

    #[derive(Debug, Default)]
    pub struct Int<const SIGN: bool, const LEN: usize>;

    impl<const SIGN: bool, const LEN: usize> Int<SIGN, LEN> {
        pub fn encode(&self, encoder: &mut RlpEncoder) {}

        pub fn decode(decoder: &mut RlpDecoder) -> Self {
            Default::default()
        }
    }

    #[derive(Debug, Default)]
    pub struct Fixed<const SIGN: bool, const M: usize, const N: usize>;

    impl<const SIGN: bool, const M: usize, const N: usize> Fixed<SIGN, M, N> {
        pub fn encode(&self, encoder: &mut RlpEncoder) {}

        pub fn decode(decoder: &mut RlpDecoder) -> Self {
            Default::default()
        }
    }

    pub trait RlpEncodable {
        fn encode(&self, encoder: &mut RlpEncoder);
    }

    pub trait RlpDecodable {
        fn decode(decoder: &mut RlpDecoder) -> Self;
    }

    impl RlpEncodable for Vec<u8> {
        fn encode(&self, encoder: &mut RlpEncoder) {}
    }

    impl RlpDecodable for Vec<u8> {
        fn decode(decoder: &mut RlpDecoder) -> Self {
            vec![]
        }
    }

    impl<const LEN: usize> RlpEncodable for [u8; LEN] {
        fn encode(&self, encoder: &mut RlpEncoder) {}
    }

    impl<const LEN: usize> RlpDecodable for [u8; LEN] {
        fn decode(decoder: &mut RlpDecoder) -> Self {
            [0; LEN]
        }
    }
}

contract!("tests/mapping.json", "tests/abi.json");

#[test]
fn test_gen() {}

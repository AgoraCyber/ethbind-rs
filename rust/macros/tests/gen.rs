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

    impl RlpDecodable for Address {
        fn decode(decoder: &mut RlpDecoder) -> Self {
            Default::default()
        }
    }

    impl RlpEncodable for Address {
        fn encode(&self, encoder: &mut RlpEncoder) {}
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

    impl RlpEncoder {
        pub fn rlp_start_encode_tuple(&mut self) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn rlp_end_encode_tuple(&mut self) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn rlp_encode<E: RlpEncodable>(&mut self, value: &E) -> anyhow::Result<()> {
            Ok(())
        }
    }

    #[derive(Debug, Default)]
    pub struct RlpDecoder;

    impl RlpDecoder {
        pub fn rlp_start_decode_tuple(&mut self) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn rlp_end_decode_tuple(&mut self) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn rlp_decode<D: RlpDecodable>(&mut self) -> anyhow::Result<D> {
            Ok(D::decode(self))
        }
    }

    #[derive(Debug, Default)]
    pub struct Int<const SIGN: bool, const LEN: usize>;

    impl<const SIGN: bool, const LEN: usize> RlpEncodable for Int<SIGN, LEN> {
        fn encode(&self, encoder: &mut RlpEncoder) {}
    }

    impl<const SIGN: bool, const LEN: usize> RlpDecodable for Int<SIGN, LEN> {
        fn decode(decoder: &mut RlpDecoder) -> Self {
            Default::default()
        }
    }

    #[derive(Debug, Default)]
    pub struct Fixed<const SIGN: bool, const M: usize, const N: usize>;

    impl<const SIGN: bool, const M: usize, const N: usize> RlpEncodable for Fixed<SIGN, M, N> {
        fn encode(&self, encoder: &mut RlpEncoder) {}
    }

    impl<const SIGN: bool, const M: usize, const N: usize> RlpDecodable for Fixed<SIGN, M, N> {
        fn decode(decoder: &mut RlpDecoder) -> Self {
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

    impl RlpEncodable for bool {
        fn encode(&self, encoder: &mut RlpEncoder) {}
    }

    impl RlpDecodable for bool {
        fn decode(decoder: &mut RlpDecoder) -> Self {
            false
        }
    }

    pub trait LogDecodable: Sized {
        fn decode(decoder: &mut LogDecoder) -> anyhow::Result<Self>;
    }

    #[derive(Default)]
    pub struct LogDecoder(RlpDecoder);

    impl LogDecoder {
        pub fn topic_rlp_decode<D: RlpDecodable>(&mut self, index: usize) -> anyhow::Result<D> {
            Ok(D::decode(&mut RlpDecoder::default()))
        }

        pub fn data_decoder(&mut self) -> &mut RlpDecoder {
            &mut self.0
        }
    }
}

contract!("tests/mapping.json", "tests/abi.json");

#[test]
fn test_gen() {}

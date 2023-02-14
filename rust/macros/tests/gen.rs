use ethbind_rust_macros::contract;

#[allow(unused)]
mod mock {
    pub struct Ops;

    pub struct Client;

    impl Client {
        pub fn abi_encoder(&self) -> AbiEncoder {
            AbiEncoder::default()
        }

        pub async fn deploy_contract(
            &self,
            encoder: AbiEncoder,
            deploy_data: &str,
            ops: Ops,
        ) -> anyhow::Result<Address> {
            Ok(Default::default())
        }

        pub async fn eth_call(
            &self,
            address: &Address,
            encoder: AbiEncoder,
        ) -> anyhow::Result<AbiDecoder> {
            Ok(Default::default())
        }

        pub async fn send_raw_transaction(
            &self,
            address: &Address,
            encoder: AbiEncoder,
            ops: Ops,
        ) -> anyhow::Result<TransactionReceipt> {
            Ok(Default::default())
        }
    }

    #[derive(Debug, Default)]
    pub struct Address;

    impl AbiDecodable for Address {
        fn decode(decoder: &mut AbiDecoder) -> Self {
            Default::default()
        }
    }

    impl AbiEncodable for Address {
        fn encode(&self, encoder: &mut AbiEncoder) {}
    }

    #[derive(Debug, Default)]
    pub struct TransactionReceipt;

    impl TransactionReceipt {
        pub fn encode(&self, encoder: &mut AbiEncoder) {}

        pub fn decode(decoder: &mut AbiDecoder) -> Self {
            Default::default()
        }
    }

    #[derive(Debug, Default)]
    pub struct AbiEncoder;

    impl AbiEncoder {
        pub fn abi_start_encode_tuple(&mut self) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn abi_end_encode_tuple(&mut self) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn abi_encode<E: AbiEncodable>(&mut self, value: &E) -> anyhow::Result<()> {
            Ok(())
        }
    }

    #[derive(Debug, Default)]
    pub struct AbiDecoder;

    impl AbiDecoder {
        pub fn abi_start_decode_tuple(&mut self) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn abi_end_decode_tuple(&mut self) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn abi_decode<D: AbiDecodable>(&mut self) -> anyhow::Result<D> {
            Ok(D::decode(self))
        }
    }

    #[derive(Debug, Default)]
    pub struct Int<const SIGN: bool, const LEN: usize>;

    impl<const SIGN: bool, const LEN: usize> AbiEncodable for Int<SIGN, LEN> {
        fn encode(&self, encoder: &mut AbiEncoder) {}
    }

    impl<const SIGN: bool, const LEN: usize> AbiDecodable for Int<SIGN, LEN> {
        fn decode(decoder: &mut AbiDecoder) -> Self {
            Default::default()
        }
    }

    #[derive(Debug, Default)]
    pub struct Fixed<const SIGN: bool, const M: usize, const N: usize>;

    impl<const SIGN: bool, const M: usize, const N: usize> AbiEncodable for Fixed<SIGN, M, N> {
        fn encode(&self, encoder: &mut AbiEncoder) {}
    }

    impl<const SIGN: bool, const M: usize, const N: usize> AbiDecodable for Fixed<SIGN, M, N> {
        fn decode(decoder: &mut AbiDecoder) -> Self {
            Default::default()
        }
    }

    pub trait AbiEncodable {
        fn encode(&self, encoder: &mut AbiEncoder);
    }

    pub trait AbiDecodable {
        fn decode(decoder: &mut AbiDecoder) -> Self;
    }

    impl AbiEncodable for Vec<u8> {
        fn encode(&self, encoder: &mut AbiEncoder) {}
    }

    impl AbiDecodable for Vec<u8> {
        fn decode(decoder: &mut AbiDecoder) -> Self {
            vec![]
        }
    }

    impl<const LEN: usize> AbiEncodable for [u8; LEN] {
        fn encode(&self, encoder: &mut AbiEncoder) {}
    }

    impl<const LEN: usize> AbiDecodable for [u8; LEN] {
        fn decode(decoder: &mut AbiDecoder) -> Self {
            [0; LEN]
        }
    }

    impl AbiEncodable for bool {
        fn encode(&self, encoder: &mut AbiEncoder) {}
    }

    impl AbiDecodable for bool {
        fn decode(decoder: &mut AbiDecoder) -> Self {
            false
        }
    }

    pub trait LogDecodable: Sized {
        fn decode(decoder: &mut LogDecoder) -> anyhow::Result<Self>;
    }

    #[derive(Default)]
    pub struct LogDecoder(AbiDecoder);

    impl LogDecoder {
        pub fn topic_abi_decode<D: AbiDecodable>(&mut self, index: usize) -> anyhow::Result<D> {
            Ok(D::decode(&mut AbiDecoder::default()))
        }

        pub fn data_decoder(&mut self) -> &mut AbiDecoder {
            &mut self.0
        }
    }
}

contract!("tests/mapping.json", "tests/abi.json");

#[test]
fn test_gen() {}

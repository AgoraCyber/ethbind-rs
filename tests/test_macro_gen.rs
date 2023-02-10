#![allow(unused)]

use ethbind::contract;

#[derive(Default)]
struct Decodable;

#[derive(Default)]
struct Encodable;

macro_rules! mock_rt {
    ($ident: ident) => {
        #[derive(Default)]
        struct $ident;

        impl $ident {
            pub fn decodable() -> Decodable {
                Default::default()
            }

            pub fn encodable(&self) -> Encodable {
                Default::default()
            }
        }
    };
}

mock_rt!(Client);

impl Client {
    fn deploy_contract(&self, data: Vec<u8>, ops: ClientOps) -> anyhow::Result<TransactionReceipt> {
        unimplemented!()
    }
}

mock_rt!(ClientOps);
mock_rt!(Address);
mock_rt!(TransactionReceipt);
mock_rt!(Event);

impl Event {
    pub fn new(inputs: Vec<Decodable>) -> Self {
        Event::default()
    }
}

#[derive(Default)]
struct Int<const SIGN: bool, const LEN: usize>;

impl<const SIGN: bool, const LEN: usize> Int<SIGN, LEN> {
    pub fn decodable() -> Decodable {
        Default::default()
    }
}

#[derive(Debug, Default)]
struct Fixed<const SIGN: bool, const M: usize, const N: usize>;

impl<const SIGN: bool, const M: usize, const N: usize> Fixed<SIGN, M, N> {
    pub fn decodable() -> Decodable {
        Default::default()
    }
}

fn encode_inputs(inputs: Vec<Encodable>) -> anyhow::Result<Vec<u8>> {
    Ok(vec![])
}

contract!(Test, "tests/mapping.json", "tests/abi.json", hardhat);

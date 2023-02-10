#![allow(unused)]

use ethbind::contract;

#[derive(Default, PartialEq, Debug)]
pub struct Decodable(String, bool);

#[derive(Default)]
pub struct Encodable(String);

macro_rules! mock_rt {
    ($ident: ident) => {
        #[derive(Default)]
        pub struct $ident;

        impl $ident {
            pub fn decodable(indexed: bool) -> Decodable {
                Decodable(stringify!($ident).to_string(), indexed)
            }

            pub fn encodable(&self) -> Encodable {
                Encodable(stringify!($ident).to_string())
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

#[derive(Default)]
pub struct Event(Vec<Decodable>);

impl Event {
    pub fn new(inputs: Vec<Decodable>) -> Self {
        Event(inputs)
    }
}

#[derive(Default)]
pub struct Error(Vec<Decodable>);

impl Error {
    pub fn new(inputs: Vec<Decodable>) -> Self {
        Error(inputs)
    }
}

#[derive(Default)]
struct Int<const SIGN: bool, const LEN: usize>;

impl<const SIGN: bool, const LEN: usize> Int<SIGN, LEN> {
    pub fn decodable(indexed: bool) -> Decodable {
        if SIGN {
            Decodable(format!("int{}", LEN), indexed)
        } else {
            Decodable(format!("uint{}", LEN), indexed)
        }
    }
}

#[derive(Debug, Default)]
struct Fixed<const SIGN: bool, const M: usize, const N: usize>;

impl<const SIGN: bool, const M: usize, const N: usize> Fixed<SIGN, M, N> {
    pub fn decodable(indexed: bool) -> Decodable {
        Default::default()
    }
}

fn encode_inputs(inputs: Vec<Encodable>) -> anyhow::Result<Vec<u8>> {
    Ok(vec![])
}

contract!(Test, "tests/mapping.json", "tests/abi.json", hardhat);

#[test]
fn test_contract_event() {
    let event = Test::event_decodable(TestEvents::Delist);

    assert_eq!(
        event.0,
        vec![
            Decodable("Address".to_string(), true),
            Decodable("uint256".to_string(), true),
            Decodable("Address".to_string(), true),
        ]
    );
}

#[test]
fn test_contract_error() {
    let event = Test::error_decodable(TestErrors::DelistError);

    assert_eq!(
        event.0,
        vec![
            Decodable("Address".to_string(), true),
            Decodable("uint256".to_string(), true),
            Decodable("Address".to_string(), true),
        ]
    );
}

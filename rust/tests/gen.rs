use std::str::FromStr;

use ethbind_gen::{JsonRuntimeBinder, SaveTo};
use ethbind_gen_rust::{BindingBuilder, RustGenerator, RustPretty};

#[test]
fn test_gen() {
    let mut contracts = BindingBuilder::new((
        RustGenerator::default(),
        JsonRuntimeBinder::from_str(include_str!("mapping.json"))
            .expect("Load json runtime binder"),
    ))
    .bind_hardhat(include_str!("abi.json"))
    .finalize()
    .expect("Finalize");

    contracts.pretty().expect("Call pretty");

    contracts.save_to("./").expect("Gen");
}

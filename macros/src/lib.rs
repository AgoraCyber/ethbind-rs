use std::{env, fs::read_to_string, path::PathBuf};

use ethbind_core::{BindingBuilder, SerdeTypeMapping};
use ethbind_rust::RustBinding;
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{parse::Parse, parse_macro_input, LitStr, Token};

struct Contract {
    pub contract_name: String,
    pub type_mapping: String,
    pub abi_data: String,
    pub is_hardhat: bool,
}

impl Parse for Contract {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let contract_name: Ident = input.parse()?;

        input.parse::<Token!(,)>()?;

        let type_mapping: LitStr = input.parse()?;

        input.parse::<Token!(,)>()?;

        let abi_data: LitStr = input.parse()?;

        let is_hardhat: Option<contract::hardhat> =
            if let Some(_) = input.parse::<Option<Token!(,)>>()? {
                input.parse()?
            } else {
                None
            };

        Ok(Self {
            contract_name: contract_name.to_string(),
            type_mapping: type_mapping.value(),
            abi_data: abi_data.value(),
            is_hardhat: is_hardhat.is_some(),
        })
    }
}

mod contract {
    syn::custom_keyword!(hardhat);
}

fn load_json_file(path: &str) -> String {
    let dir = env::var("CARGO_MANIFEST_DIR").expect("Find CARGO_MANIFEST_DIR");

    let path = PathBuf::from(dir).join(path);

    read_to_string(path.clone()).expect(&format!("Read json file: {:?}", path))
}

#[proc_macro]
pub fn contract(item: TokenStream) -> TokenStream {
    let contract = parse_macro_input!(item as Contract);

    let type_mapping: SerdeTypeMapping = load_json_file(&contract.type_mapping)
        .parse()
        .expect("Parse mapping data");

    let abi_data = load_json_file(&contract.abi_data);

    let generator = if contract.is_hardhat {
        BindingBuilder::new(RustBinding::new(type_mapping))
            .bind_hardhat(&contract.contract_name, abi_data)
            .finalize()
            .expect("Generate contract/abi binding code")
    } else {
        BindingBuilder::new(RustBinding::new(type_mapping))
            .bind(&contract.contract_name, abi_data)
            .finalize()
            .expect("Generate contract/abi binding code")
    };

    let contracts = generator.to_token_streams().expect("To token streams");

    let contracts = contracts.into_iter().map(|(_, ts)| ts).collect::<Vec<_>>();

    quote!(#(#contracts)*).into()
}

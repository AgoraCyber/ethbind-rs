use ethbind_core::{BindingBuilder, SerdeTypeMapping};
use ethbind_rust::RustBinding;
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{parse::Parse, parse_macro_input, LitStr};

struct Contract {
    pub contract_name: String,
    pub type_mapping: String,
    pub abi_data: String,
    pub is_hardhat: bool,
}

impl Parse for Contract {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let contract_name: Ident = input.parse()?;

        let type_mapping: LitStr = input.parse()?;
        let abi_data: LitStr = input.parse()?;

        use contract::hardhat;

        let is_hardhat: Option<hardhat> = input.parse()?;

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

#[proc_macro]
pub fn contract(item: TokenStream) -> TokenStream {
    let contract = parse_macro_input!(item as Contract);

    let type_mapping: SerdeTypeMapping = contract.type_mapping.parse().expect("Parse mapping data");

    let generator = if contract.is_hardhat {
        BindingBuilder::new(RustBinding::new(type_mapping))
            .bind_hardhat(&contract.contract_name, contract.abi_data)
            .finalize()
            .expect("Generate contract/abi binding code")
    } else {
        BindingBuilder::new(RustBinding::new(type_mapping))
            .bind(&contract.contract_name, contract.abi_data)
            .finalize()
            .expect("Generate contract/abi binding code")
    };

    let contracts = generator.to_token_streams().expect("To token streams");

    let contracts = contracts.into_iter().map(|(_, ts)| ts).collect::<Vec<_>>();

    quote!(#(#contracts)*).into()
}

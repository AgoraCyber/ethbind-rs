use heck::ToSnakeCase;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use ethbind_core::{json::Parameter, Context, SerdeTypeMapping};

use crate::contract::ContractBinding;

/// The generator implemented for the target `Rust`
pub struct RustBinding {
    mapping: SerdeTypeMapping,
    contracts: Vec<ContractBinding>,
}

impl RustBinding {
    /// Create new `RustBinding` instance by parameter `mapping`
    pub fn new(mapping: SerdeTypeMapping) -> RustBinding {
        Self {
            mapping,
            contracts: Default::default(),
        }
    }

    /// Retrieve generated binding code to token streams
    pub fn to_token_streams(&self) -> anyhow::Result<Vec<(String, TokenStream)>> {
        let rt_context = self.get_mapping_token_stream("rt_context", &[])?;
        let rt_contract_event = self.get_mapping_token_stream("rt_contract_event", &[])?;
        let rt_contract_error = self.get_mapping_token_stream("rt_contract_error", &[])?;

        let contracts = self
            .contracts
            .iter()
            .map(|c| {
                (
                    c.contract_ident.to_string(),
                    c.gen_codes(&rt_context, &rt_contract_event, &rt_contract_error),
                )
            })
            .collect::<Vec<_>>();

        Ok(contracts)
    }

    /// Retrieve generated binding code to `String`
    pub fn to_string(&self) -> anyhow::Result<String> {
        let streams = self
            .to_token_streams()?
            .into_iter()
            .map(|(_, stream)| stream)
            .collect::<Vec<_>>();

        Ok(quote! {
            #(#streams)*
        }
        .to_string())
    }

    fn get_mapping_token_stream(
        &self,
        name: &str,
        args: &[(&str, &str)],
    ) -> anyhow::Result<TokenStream> {
        self.mapping
            .get_mapping(name, args)?
            .parse()
            .map_err(|e| anyhow::format_err!("{}", e))
    }

    fn add_impl_stream(&mut self, token_stream: TokenStream) -> &mut Self {
        self.contracts
            .last_mut()
            .expect("Call begin first")
            .add_impl_stream(token_stream);

        self
    }

    fn add_error_stream(&mut self, event_ident: Ident, token_stream: TokenStream) -> &mut Self {
        self.contracts
            .last_mut()
            .expect("Call begin first")
            .add_error_stream(event_ident, token_stream);

        self
    }

    fn add_tuple_stream(&mut self, event_ident: Ident, token_stream: TokenStream) -> &mut Self {
        self.contracts
            .last_mut()
            .expect("Call begin first")
            .add_tuple_stream(event_ident, token_stream);

        self
    }

    fn add_event_stream(&mut self, event_ident: Ident, token_stream: TokenStream) -> &mut Self {
        self.contracts
            .last_mut()
            .expect("Call begin first")
            .add_event_stream(event_ident, token_stream);

        self
    }

    fn contract_name(&self) -> String {
        self.contracts
            .last()
            .expect("Call begin first")
            .contract_ident
            .to_string()
    }

    fn mod_ident(&self) -> Ident {
        Ident::new(&self.contract_name().to_snake_case(), Span::call_site())
    }

    /// Get mapping parameter type token stream.
    ///
    fn mapping_parameter<C: Context>(
        &self,
        context: &mut C,
        parameter: &Parameter,
    ) -> anyhow::Result<TokenStream> {
        context
            .mapping_parameter(&self.mapping, parameter)
            // Convert rt_type string to token stream
            .parse()
            .map_err(|e| anyhow::format_err!("{}", e))
    }
}

mod generator;
pub use generator::*;

mod function;
pub use function::*;

mod encodable;
pub use encodable::*;

mod decodable;
pub use decodable::*;

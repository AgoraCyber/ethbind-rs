use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use ethbind_core::{
    json::{Parameter, SimpleType, Type},
    Context, Generator, SerdeTypeMapping,
};

struct ContractBinding {
    contract_ident: Ident,
    impl_token_streams: Vec<TokenStream>,
    error_token_streams: Vec<(Ident, TokenStream)>,
    event_token_streams: Vec<(Ident, TokenStream)>,
}

impl ContractBinding {
    fn new(contract_ident: Ident) -> Self {
        Self {
            contract_ident,
            impl_token_streams: Default::default(),
            error_token_streams: Default::default(),
            event_token_streams: Default::default(),
        }
    }
    fn add_impl_stream(&mut self, token_stream: TokenStream) -> &mut Self {
        self.impl_token_streams.push(token_stream);

        self
    }

    fn add_error_stream(&mut self, event_ident: Ident, token_stream: TokenStream) -> &mut Self {
        self.error_token_streams.push((event_ident, token_stream));

        self
    }

    fn add_event_stream(&mut self, event_ident: Ident, token_stream: TokenStream) -> &mut Self {
        self.event_token_streams.push((event_ident, token_stream));

        self
    }

    fn gen_codes(&self, rt_context: &TokenStream, rt_event: &TokenStream) -> TokenStream {
        let fns = &self.impl_token_streams;

        let ident = &self.contract_ident;

        let match_patterns = self
            .event_token_streams
            .iter()
            .map(|(ident, expr)| {
                let pattern = ident.to_string();

                quote! {
                    #pattern => #expr
                }
            })
            .collect::<Vec<_>>();

        quote! {
            struct #ident(#rt_context);

            impl #ident {
                #(#fns)*

                pub fn event<E: AsRef<str>>(name: E) -> #rt_event {
                    match name {
                        #(#match_patterns,)*
                        _ => panic(format!("Unknown event {}",name)),
                    }
                }
            }
        }
    }
}

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
        let rt_event = self.get_mapping_token_stream("rt_event", &[])?;

        let contracts = self
            .contracts
            .iter()
            .map(|c| {
                (
                    c.contract_ident.to_string(),
                    c.gen_codes(&rt_context, &rt_event),
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

    /// Generate input parameters token streams, returns tuple (parameter_list,generic_list,param_try_into_clauses,to_encoding_param_clauses, where_clauses)
    fn gen_input_streams<C: Context>(
        &self,
        context: &mut C,
        tag: &str,
        inputs: &[ethbind_core::json::Parameter],
    ) -> anyhow::Result<(
        Vec<TokenStream>,
        Vec<TokenStream>,
        Vec<TokenStream>,
        Vec<TokenStream>,
        Vec<TokenStream>,
    )> {
        let mut parameter_list = vec![];

        let mut generic_list = vec![];

        let mut param_try_into_clauses = vec![];

        let mut to_encoding_param_clauses = vec![];

        let mut where_clauses = vec![];

        let rt_error = self.get_mapping_token_stream("rt_error", &[])?;

        for (index, input) in inputs.iter().enumerate() {
            let variable_name = input.name.to_snake_case();

            let type_ident = Ident::new(&format!("P{}", index), Span::call_site());

            let variable_ident = Ident::new(&variable_name, Span::call_site());

            generic_list.push(quote!(#type_ident));

            parameter_list.push(quote!(#variable_ident : #type_ident));

            let mapping_type: TokenStream = context
                .mapping_parameter(&self.mapping, input)
                .parse()
                .map_err(|e| anyhow::format_err!("{}", e))?;

            where_clauses.push(
                quote!(#type_ident: TryInto<#mapping_type>, #type_ident::Error: Into<#rt_error>),
            );

            param_try_into_clauses.push(quote!(let #variable_ident = #variable_ident.try_into()?));

            match input.r#type {
                Type::Simple(SimpleType::Tuple) => {
                    if let Some(tuple) = &input.components {
                        to_encoding_param_clauses.push(self.tuple_to_encoding_param(
                            tag,
                            &input.name,
                            tuple,
                        )?);
                    } else {
                        log::warn!(
                            "{}: input parameter {} is a tuple, but tuple fields is empty",
                            tag,
                            input.name
                        );
                    }
                }
                _ => {
                    let rt_to_encoding_param: TokenStream = self.get_mapping_token_stream(
                        "rt_to_encodable_param",
                        &[("$param", &variable_name)],
                    )?;

                    to_encoding_param_clauses.push(quote!(#rt_to_encoding_param));
                }
            }
        }

        Ok((
            parameter_list,
            generic_list,
            param_try_into_clauses,
            to_encoding_param_clauses,
            where_clauses,
        ))
    }

    /// Convert contract tuple metadata to rt_to_encoding_param
    fn tuple_to_encoding_param(
        &self,
        tag: &str,
        path: &str,
        tuple: &Vec<Parameter>,
    ) -> anyhow::Result<TokenStream> {
        // let rt_to_encoding_param = self.mapping.get_mapping("rt_to_encoding_param", &[("$param",&variable_name)])?;

        let mut to_token_clauses = vec![];

        for (index, param) in tuple.iter().enumerate() {
            let new_path = format!("{}.{}", path, index);

            match param.r#type {
                Type::Simple(SimpleType::Tuple) => {
                    if let Some(tuple) = &param.components {
                        to_token_clauses.push(self.tuple_to_encoding_param(tag, &new_path, tuple)?);
                    } else {
                        log::warn!(
                            "{}: input parameter {} is a tuple, but tuple fields is empty",
                            tag,
                            new_path
                        );
                    }
                }
                _ => {
                    let rt_to_encoding_param: TokenStream = self.get_mapping_token_stream(
                        "rt_to_encodable_param",
                        &[("$param", &new_path)],
                    )?;

                    to_token_clauses.push(quote!(#rt_to_encoding_param));
                }
            }
        }

        self.get_mapping_token_stream(
            "rt_to_encodable_param",
            &[("$param", &quote!(vec![#(#to_token_clauses,)*]).to_string())],
        )
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

    /// Convert contract parameter to token stream of rt decodable param construct expr
    fn to_decodable_token_stream<C: Context>(
        &self,
        context: &mut C,
        parameter: &Parameter,
    ) -> anyhow::Result<TokenStream> {
        let mapping_type = self.mapping_parameter(context, parameter)?;

        self.get_mapping_token_stream(
            "rt_decodable_param_new",
            &[("$type", &mapping_type.to_string())],
        )
    }
}

#[allow(unused)]
impl Generator for RustBinding {
    type TypeMapping = SerdeTypeMapping;

    fn begin<C: Context>(&mut self, ctx: &mut C, contract_name: &str) -> anyhow::Result<()> {
        let contract_ident = Ident::new(&contract_name.to_upper_camel_case(), Span::call_site());

        self.contracts.push(ContractBinding::new(contract_ident));

        Ok(())
    }

    fn end<C: Context>(&mut self, ctx: &mut C) -> anyhow::Result<()> {
        Ok(())
    }

    fn generate_deploy<C: Context>(
        &mut self,
        ctx: &mut C,
        bytecode: &str,
        inputs: &[ethbind_core::json::Parameter],
    ) -> anyhow::Result<()> {
        let (
            params,
            generic_list,
            param_try_into_clauses,
            to_encoding_param_clauses,
            generic_where_clauses,
        ) = self.gen_input_streams(ctx, "generate deploy fn", inputs)?;

        let rt_context = self.get_mapping_token_stream("rt_context", &[])?;

        let rt_ops = self.get_mapping_token_stream("rt_ops", &[])?;

        let rt_error = self.get_mapping_token_stream("rt_error", &[])?;

        let rt_tx_receipt = self.get_mapping_token_stream("rt_tx_receipt", &[])?;

        let rt_encode_inputs =
            self.get_mapping_token_stream("rt_encode_input", &[("$inputs", "inputs")])?;

        self.add_impl_stream(quote! {
            pub fn deploy<C,#(#generic_list,)* Ops>(context: C, #(#params,)* ops: Ops) -> Result<#rt_tx_receipt,#rt_error>
            where C: TryInto<#rt_context>,  C::Error: Into<#rt_error>, Ops: TryInto<#rt_ops>, Ops::Error: Into<#rt_error>,
            #(#generic_where_clauses,)*
            {
                let context = context.try_into()?;

                #(#param_try_into_clauses;)*

                let ops = ops.try_into()?;

                let inputs = vec![#(#to_encoding_param_clauses;)*];

                let call_data = #rt_encode_inputs;

                context.deploy_contract(call_data,ops)
            }
        });

        Ok(())
    }

    fn generate_error<C: Context>(
        &mut self,
        ctx: &mut C,
        event: &ethbind_core::json::Error,
    ) -> anyhow::Result<()> {
        let mod_ident = self.mod_ident();

        let error_ident = Ident::new(&event.name.to_upper_camel_case(), Span::call_site());

        let mut params = vec![];

        for param in &event.inputs {
            params.push(self.to_decodable_token_stream(ctx, param)?);
        }

        let params = quote!(vec![#(#params,)*]).to_string();

        let error_type_new =
            self.get_mapping_token_stream("rt_event_error_new", &[("$params", &params)])?;

        self.add_error_stream(error_ident, error_type_new);

        Ok(())
    }

    fn generate_event<C: Context>(
        &mut self,
        ctx: &mut C,
        event: &ethbind_core::json::Event,
    ) -> anyhow::Result<()> {
        let event_ident = Ident::new(&event.name.to_upper_camel_case(), Span::call_site());

        let mut params = vec![];

        for param in &event.inputs {
            params.push(self.to_decodable_token_stream(ctx, param)?);
        }

        let params = quote!(vec![#(#params,)*]).to_string();

        let event_type_new =
            self.get_mapping_token_stream("rt_event_new", &[("$params", &params)])?;

        self.add_event_stream(event_ident, event_type_new);

        Ok(())
    }

    fn generate_function<C: Context>(
        &mut self,
        ctx: &mut C,
        function: &ethbind_core::json::Function,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn generate_tuple<C: Context>(
        &mut self,
        ctx: &mut C,
        name: &str,
        tuple: &[ethbind_core::json::Parameter],
    ) -> anyhow::Result<String> {
        Ok("".to_string())
    }

    fn type_mapping(&self) -> &Self::TypeMapping {
        &self.mapping
    }
}

// #[cfg(test)]
// mod tests {
//     use std::{
//         env,
//         fs::{read_to_string, remove_file, File},
//         io::Write,
//         path::PathBuf,
//         process::Command,
//     };

//     use sha3::{Digest, Keccak256};

//     use crate::{BindingBuilder, SerdeTypeMapping};

//     use super::RustBinding;

//     #[test]
//     fn test_gen_rust() {
//         _ = pretty_env_logger::try_init();

//         let types_mapping: SerdeTypeMapping =
//             serde_json::from_str(include_str!("../../../data/mapping.json"))
//                 .expect("Load types mapping data");

//         let codes = BindingBuilder::new(RustBinding::new(types_mapping))
//             .bind_hardhat("test", include_str!("../../../data/abi.json"))
//             .finalize()
//             .expect("Generate codes")
//             .to_string()
//             .expect("Generate codes");

//         let rust_fmt_path =
//             PathBuf::from(env::var("CARGO_HOME").expect("Get CARGO_HOME")).join("bin/rustfmt");

//         let temp_file_name = format!(
//             "{:x}",
//             Keccak256::new().chain_update(codes.as_bytes()).finalize()
//         );

//         let path = env::temp_dir().join(temp_file_name);

//         if path.exists() {
//             remove_file(path.clone()).expect("Remove exists generate file");
//         }

//         let mut file = File::create(path.clone()).expect("Open tmp file");

//         file.write_all(codes.as_bytes()).expect("Write tmp file");

//         // Call rustfmt to fmt tmp file
//         let mut child = Command::new(&rust_fmt_path)
//             .args([path.to_str().unwrap()])
//             .spawn()
//             .expect("failed to execute child");

//         child.wait().expect("failed to wait on child");

//         let formated = read_to_string(path).expect("Read formated codes");

//         log::debug!("generated: \n{}", formated);

//         assert_eq!(formated, include_str!("../../../data/expect.rs"));
//     }
// }

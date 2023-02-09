use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use crate::json::{Parameter, SimpleType, Type};

use super::{mapping::SerdeTypeMapping, Context, Generator};

/// The generator implemented for the target `Rust`
pub struct RustBinding {
    contract_ident: Option<Ident>,
    mapping: SerdeTypeMapping,
    fn_token_streams: Vec<TokenStream>,
}

impl RustBinding {
    /// Create new `RustBinding` instance by parameter `mapping`
    pub fn new(mapping: SerdeTypeMapping) -> RustBinding {
        Self {
            mapping,
            fn_token_streams: Default::default(),
            contract_ident: None,
        }
    }

    pub fn gen_codes(&self) -> anyhow::Result<String> {
        let rt_context = self.get_mapping_token_stream("rt_context", &[])?;

        let fns = &self.fn_token_streams;

        let ident = &self.contract_ident;

        Ok(quote! {
            struct #ident(#rt_context);

            impl #ident {
                #(#fns)*
            }
        }
        .to_string())
    }

    /// Generate input parameters token streams, returns tuple (parameter_list,generic_list,param_try_into_clauses,to_encoding_param_clauses, where_clauses)
    fn gen_input_streams(
        &self,
        context: &mut Context,
        tag: &str,
        inputs: &[crate::json::Parameter],
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
}

#[allow(unused)]
impl Generator for RustBinding {
    type TypeMapping = SerdeTypeMapping;

    fn start_generate_contract(&mut self, ctx: &mut super::Context) -> anyhow::Result<()> {
        let struct_ident = Ident::new(
            &ctx.contract_name().to_upper_camel_case(),
            Span::call_site(),
        );

        self.contract_ident = Some(struct_ident);

        Ok(())
    }
    fn generate_deploy(
        &mut self,
        ctx: &mut super::Context,
        bytecode: &str,
        inputs: &[crate::json::Parameter],
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

        self.fn_token_streams.push(quote! {
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

    fn generate_error(
        &mut self,
        ctx: &mut super::Context,
        event: &crate::json::Error,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn generate_event(
        &mut self,
        ctx: &mut super::Context,
        event: &crate::json::Event,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn generate_function(
        &mut self,
        ctx: &mut super::Context,
        function: &crate::json::Function,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn generate_tuple(
        &mut self,
        ctx: &mut super::Context,
        name: &str,
        tuple: &[crate::json::Parameter],
    ) -> anyhow::Result<String> {
        Ok("".to_string())
    }

    fn type_mapping(&self) -> &Self::TypeMapping {
        &self.mapping
    }
}

#[cfg(test)]
mod tests {
    use std::{
        env,
        fs::{read_to_string, remove_file, File},
        io::Write,
        path::PathBuf,
        process::Command,
    };

    use crate::{
        gen::{mapping::SerdeTypeMapping, Generate},
        json::HardhatArtifact,
    };

    use super::RustBinding;

    #[test]
    fn test_deploy() {
        _ = pretty_env_logger::try_init();

        let artifact: HardhatArtifact = serde_json::from_str(include_str!("../../data/abi.json"))
            .expect("Parse hardhat artifact");

        let types_mapping: SerdeTypeMapping =
            serde_json::from_str(include_str!("../../data/mapping.json"))
                .expect("Load types mapping data");

        let mut rust_binding = RustBinding::new(types_mapping);

        artifact
            .generate("test", &mut rust_binding)
            .expect("Generate codes");

        let codes = rust_binding.gen_codes().expect("Generate contract mode");

        let cargo_toml_directory = env::var("CARGO_MANIFEST_DIR").expect("Get CARGO_MANIFEST_DIR");

        let rust_fmt_path =
            PathBuf::from(env::var("CARGO_HOME").expect("Get CARGO_HOME")).join("bin/rustfmt");

        let path = PathBuf::from(cargo_toml_directory).join("abi.rs");

        if path.exists() {
            remove_file(path.clone()).expect("Remove exists generate file");
        }

        {
            let mut file = File::create(path.clone()).expect("Open tmp file");

            file.write_all(codes.as_bytes()).expect("Write tmp file");
        }

        // Call rustfmt to fmt tmp file
        let mut child = Command::new(&rust_fmt_path)
            .args([path.to_str().unwrap()])
            .spawn()
            .expect("failed to execute child");

        child.wait().expect("failed to wait on child");

        let formated = read_to_string(path).expect("Read formated codes");

        log::debug!("generated: \n{}", formated);

        assert_eq!(formated, include_str!("../../data/expect.rs"));
    }
}

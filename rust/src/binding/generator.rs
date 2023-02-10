use ethbind_core::{Context, Generator, SerdeTypeMapping};
use heck::ToUpperCamelCase;
use proc_macro2::{Ident, Span};
use quote::{format_ident, quote};

use crate::{contract::ContractBinding, RustBinding};

#[allow(unused)]
impl Generator for RustBinding {
    type TypeMapping = SerdeTypeMapping;

    fn begin<C: Context>(&mut self, ctx: &mut C, contract_name: &str) -> anyhow::Result<()> {
        let contract_ident = Ident::new(&contract_name.to_upper_camel_case(), Span::call_site());

        self.contracts.push(ContractBinding::new(contract_ident));

        Ok(())
    }

    fn generate_deploy<C: Context>(
        &mut self,
        context: &mut C,
        bytecode: &str,
        inputs: &[ethbind_core::json::Parameter],
    ) -> anyhow::Result<()> {
        let generic_list = self.to_fn_generic_list(context, "deploy", inputs)?;

        let params = self.to_fn_params_list(context, "deploy", inputs)?;

        let generic_where_clauses = self.to_fn_generic_where_clause(context, "deploy", inputs)?;

        let param_try_into_clauses =
            self.to_fn_params_try_into_call_list(context, "deploy", inputs)?;

        let to_encodable_clauses = self.to_encodable_token_streams(context, inputs)?;

        let rt_context = self.get_mapping_token_stream("rt_context", &[])?;

        let rt_ops = self.get_mapping_token_stream("rt_ops", &[])?;

        let rt_error = self.get_mapping_token_stream("rt_error", &[])?;

        let rt_tx_receipt = self.get_mapping_token_stream("rt_tx_receipt", &[])?;

        let rt_encode_inputs =
            self.get_mapping_token_stream("rt_encode_input", &[("$inputs", "inputs")])?;

        self.add_impl_stream(quote! {
            pub fn deploy<C,#generic_list Ops>(context: C, #params ops: Ops) -> Result<#rt_tx_receipt,#rt_error>
            where C: TryInto<#rt_context>,  C::Error: std::error::Error + Sync + Send + 'static, Ops: TryInto<#rt_ops>, Ops::Error: std::error::Error + Sync + Send + 'static,
            #generic_where_clauses
            {
                let context = context.try_into()?;

                #param_try_into_clauses

                let ops = ops.try_into()?;

                let inputs = vec![#(#to_encodable_clauses,)*];

                let call_data = #rt_encode_inputs?;

                context.deploy_contract(call_data,ops)
            }
        });

        Ok(())
    }

    fn generate_error<C: Context>(
        &mut self,
        context: &mut C,
        event: &ethbind_core::json::Error,
    ) -> anyhow::Result<()> {
        let mod_ident = self.mod_ident();

        let error_ident = Ident::new(&event.name.to_upper_camel_case(), Span::call_site());

        let mut params = self.to_decodable_token_streams(context, &event.inputs)?;

        let params = quote!(vec![#(#params,)*]).to_string();

        let error_type_new =
            self.get_mapping_token_stream("rt_contract_error_new", &[("$params", &params)])?;

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
            self.get_mapping_token_stream("rt_contract_event_new", &[("$params", &params)])?;

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
        context: &mut C,
        r_type: &str,
        tuple: &[ethbind_core::json::Parameter],
    ) -> anyhow::Result<()> {
        log::debug!("gen tuple {}", r_type);
        let tuple_ident = format_ident!("{}", r_type);

        let tuple_fields_token_stream =
            self.to_tuple_fields_token_stream(context, "tuple", tuple)?;

        self.add_tuple_stream(
            tuple_ident.clone(),
            quote! {
                pub struct #tuple_ident {
                    #tuple_fields_token_stream
                }
            },
        );

        Ok(())
    }

    fn mapping_tuple<C: Context>(
        &mut self,
        ctx: &mut C,
        name: &str,
        tuple: &[ethbind_core::json::Parameter],
    ) -> anyhow::Result<String> {
        // handle hardhat internal type

        if name.starts_with("struct ") {
            let name = name[7..].replace(".", "");

            Ok(format!("{}{}", self.contract_name(), name))
        } else {
            Ok(format!("{}Tuple{}", self.contract_name(), name))
        }
    }

    fn type_mapping(&self) -> &Self::TypeMapping {
        &self.mapping
    }
}

use ethbind_gen::Generator;
use ethbind_json::*;
use heck::{ToSnakeCase, ToUpperCamelCase};
use quote::{format_ident, quote};

use crate::RustGenerator;

#[allow(unused)]
impl Generator for RustGenerator {
    fn begin<R: ethbind_gen::RuntimeBinder>(
        &mut self,
        runtime_binder: &mut R,
        name: &str,
    ) -> anyhow::Result<()> {
        self.new_contract(name);

        Ok(())
    }

    fn end<R: ethbind_gen::RuntimeBinder>(
        &mut self,
        runtime_binder: &mut R,
        name: &str,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn finalize<R: ethbind_gen::RuntimeBinder>(
        self,
        runtime_binder: &mut R,
    ) -> anyhow::Result<Vec<ethbind_gen::Contract>> {
        let client_type = self.to_runtime_type_token_stream(runtime_binder, "rt_client")?;
        let adress = self.to_runtime_type_token_stream(runtime_binder, "address")?;

        let mut contracts = vec![];

        for c in &self.contracts {
            contracts.push(c.finalize(&client_type, &adress)?);
        }

        Ok(contracts)
    }

    fn generate_deploy<R: ethbind_gen::RuntimeBinder>(
        &mut self,
        runtime_binder: &mut R,
        contructor: &Constructor,
        deploy_bytes: &str,
    ) -> anyhow::Result<()> {
        let client_type = self.to_runtime_type_token_stream(runtime_binder, "rt_client")?;

        let opts_type = self.to_runtime_type_token_stream(runtime_binder, "rt_opts")?;

        let rlp_decodable =
            self.to_runtime_type_token_stream(runtime_binder, "rt_rlp_decodable")?;

        let rlp_encodable =
            self.to_runtime_type_token_stream(runtime_binder, "rt_rlp_encodable")?;

        let receipt_type = self.to_runtime_type_token_stream(runtime_binder, "rt_receipt")?;

        let error_type = self.to_runtime_type_token_stream(runtime_binder, "rt_error")?;

        let generic_list = self.to_generic_list(runtime_binder, &contructor.inputs)?;

        let param_list = self.to_param_list(runtime_binder, &contructor.inputs)?;

        let where_clause_list = self.to_where_clause_list(runtime_binder, &contructor.inputs)?;

        let try_into_list = self.to_try_into_list(runtime_binder, &contructor.inputs)?;

        let rlp_encode_list = self.to_rlp_encode_list(runtime_binder, &contructor.inputs)?;

        self.current_contract().add_fn_token_stream(quote! {
            pub async fn deploy_contract<C, #(#generic_list,)* Ops>(client: C, #(#param_list,)* ops: Ops) -> std::result::Result<Self,#error_type>
            where C: TryInto<#client_type>, C::Error: std::error::Error + Sync + Send + 'static,
            Ops: TryInto<#opts_type>, Ops::Error: std::error::Error + Sync + Send + 'static,
            #(#where_clause_list,)*
            {
                use #rlp_decodable;
                use #rlp_encodable;

                let mut client = client.try_into()?;
                #(#try_into_list;)*
                let ops = ops.try_into()?;

                let mut outputs = client.rlp_encoder();

                #(#rlp_encode_list;)*

                let address = client.deploy_contract(outputs,#deploy_bytes,ops).await?;

                Ok(Self(client,address))
            }
        });

        Ok(())
    }

    fn generate_error<R: ethbind_gen::RuntimeBinder>(
        &mut self,
        runtime_binder: &mut R,
        error: &Error,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn generate_event<R: ethbind_gen::RuntimeBinder>(
        &mut self,
        runtime_binder: &mut R,
        event: &Event,
    ) -> anyhow::Result<()> {
        log::trace!("generate event {}", event.name);

        let event_field_list = self.to_event_field_list(runtime_binder, &event.inputs)?;

        let error_type = self.to_runtime_type_token_stream(runtime_binder, "rt_error")?;

        let rlp_decodable =
            self.to_runtime_type_token_stream(runtime_binder, "rt_rlp_decodable")?;

        let rlp_encodable =
            self.to_runtime_type_token_stream(runtime_binder, "rt_rlp_encodable")?;

        let event_ident = format_ident!(
            "{}{}",
            self.current_contract().contract_name.to_upper_camel_case(),
            event.name.to_upper_camel_case()
        );

        self.current_contract().add_event_token_stream(quote! {
            pub struct #event_ident {
                #(#event_field_list,)*
            }

            impl #event_ident {

            }
        });

        Ok(())
    }

    fn generate_fn<R: ethbind_gen::RuntimeBinder>(
        &mut self,
        runtime_binder: &mut R,
        function: &Function,
    ) -> anyhow::Result<()> {
        log::trace!("genearte fn {}", function.name);

        let opts_type = self.to_runtime_type_token_stream(runtime_binder, "rt_opts")?;

        let error_type = self.to_runtime_type_token_stream(runtime_binder, "rt_error")?;

        let rlp_decodable =
            self.to_runtime_type_token_stream(runtime_binder, "rt_rlp_decodable")?;

        let rlp_encodable =
            self.to_runtime_type_token_stream(runtime_binder, "rt_rlp_encodable")?;

        let receipt_type = self.to_runtime_type_token_stream(runtime_binder, "rt_receipt")?;

        let generic_list = self.to_generic_list(runtime_binder, &function.inputs)?;

        let param_list = self.to_param_list(runtime_binder, &function.inputs)?;

        let where_clause_list = self.to_where_clause_list(runtime_binder, &function.inputs)?;

        let try_into_list = self.to_try_into_list(runtime_binder, &function.inputs)?;

        let rlp_encode_list = self.to_rlp_encode_list(runtime_binder, &function.inputs)?;

        let outputs_type = self.to_outputs_type(runtime_binder, &function.inputs)?;

        let rlp_decode_list = self.to_rlp_decode_list(runtime_binder, &function.inputs)?;

        let fn_ident = format_ident!("{}", function.name.to_snake_case());

        let send_transaction = match function.state_mutability {
            StateMutability::Pure | StateMutability::View => false,
            _ => true,
        };

        if send_transaction {
            self.current_contract().add_fn_token_stream(quote! {
                pub async fn #fn_ident<Ops, #(#generic_list,)* >(&self, #(#param_list,)* ops: Ops) -> std::result::Result<#receipt_type,#error_type>
                where Ops: TryInto<#opts_type>, Ops::Error: std::error::Error + Sync + Send + 'static,
                #(#where_clause_list,)*
                {

                    use #rlp_decodable;
                    use #rlp_encodable;

                    #(#try_into_list;)*
                    let ops = ops.try_into()?;

                    let mut outputs = self.0.rlp_encoder();

                    #(#rlp_encode_list;)*

                    self.0.send_raw_transaction(&self.1, outputs,ops).await
                }
            });
        } else {
            self.current_contract().add_fn_token_stream(quote! {
                pub async fn #fn_ident<#(#generic_list,)* >(&self, #(#param_list,)*) -> std::result::Result<#outputs_type,#error_type>
                where #(#where_clause_list,)*
                {
                    use #rlp_decodable;
                    use #rlp_encodable;

                    #(#try_into_list;)*

                    let mut outputs = self.0.rlp_encoder();

                    #(#rlp_encode_list;)*

                    let mut inputs = self.0.eth_call(&self.1, outputs).await?;

                    Ok(#rlp_decode_list)
                }
            });
        }

        Ok(())
    }
}

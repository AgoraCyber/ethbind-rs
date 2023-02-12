use ethbind_gen::Generator;
use ethbind_json::*;
use quote::quote;

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

    fn finalize(self) -> Vec<ethbind_gen::Contract> {
        unimplemented!()
    }

    fn generate_deploy<R: ethbind_gen::RuntimeBinder>(
        &mut self,
        runtime_binder: &mut R,
        contructor: &Constructor,
        deploy_bytes: &str,
    ) -> anyhow::Result<()> {
        let client_type = self.to_runtime_type_token_stream(runtime_binder, "rt_client")?;

        let opts_type = self.to_runtime_type_token_stream(runtime_binder, "rt_opts")?;

        let receipt_type = self.to_runtime_type_token_stream(runtime_binder, "rt_receipt")?;

        let error_type = self.to_runtime_type_token_stream(runtime_binder, "rt_error")?;

        let generic_list = self.to_generic_list(runtime_binder, &contructor.inputs)?;

        let param_list = self.to_param_list(runtime_binder, &contructor.inputs)?;

        let where_clause_list = self.to_where_clause_list(runtime_binder, &contructor.inputs)?;

        let try_into_list = self.to_try_into_list(runtime_binder, &contructor.inputs)?;

        let rlp_encode_list = self.to_rlp_encode_list(runtime_binder, &contructor.inputs)?;

        self.current_contract().add_fn_token_stream(quote! {
            pub fn deploy_contract<C, #(#generic_list,)* Ops>(client: C, #(#param_list,)* ops: Ops) -> Result<#receipt_type,#error_type>
            where C: TryInto<#client_type>, C::Error: std::error::Error + Syn + Send + 'static,
            Ops: TryInto<#opts_type>, Ops::Error: std::error::Error + Syn + Send + 'static,
            #(#where_clause_list,)*
            {
                let client = client.try_into()?;
                #(#try_into_list;)*
                let ops = ops.try_into()?;

                let outputs = client.rlp_encoder();

                #(#rlp_encode_list;)*

                client.deploy_contract(outputs,#deploy_bytes)
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
        Ok(())
    }

    fn generate_fn<R: ethbind_gen::RuntimeBinder>(
        &mut self,
        runtime_binder: &mut R,
        r#fn: &Function,
    ) -> anyhow::Result<()> {
        Ok(())
    }
}

use ethbind_core::{json::Parameter, Context};
use heck::ToSnakeCase;
use proc_macro2::TokenStream;

use crate::RustBinding;

impl RustBinding {
    #[allow(unused_variables)]
    pub(crate) fn to_encodable_token_stream<C: Context>(
        &self,
        context: &mut C,
        parameter: &Parameter,
    ) -> anyhow::Result<TokenStream> {
        self.get_mapping_token_stream(
            "rt_to_encodable",
            &[("$var", &parameter.name.to_snake_case())],
        )
    }

    #[allow(unused_variables)]
    pub(crate) fn to_encodable_token_streams<C: Context>(
        &self,
        context: &mut C,
        inputs: &[Parameter],
    ) -> anyhow::Result<Vec<TokenStream>> {
        let mut streams = vec![];

        for input in inputs {
            streams.push(self.to_encodable_token_stream(context, input)?);
        }

        Ok(streams)
    }
}

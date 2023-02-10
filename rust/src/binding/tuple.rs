use ethbind_core::{json::Parameter, Context};
use heck::ToSnakeCase;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use crate::RustBinding;

impl RustBinding {
    #[allow(unused_variables)]
    pub(crate) fn to_tuple_fields_token_stream<C: Context>(
        &self,
        context: &mut C,
        tag: &str,
        inputs: &[Parameter],
    ) -> anyhow::Result<TokenStream> {
        let mut parameter_list = vec![];

        for (index, input) in inputs.iter().enumerate() {
            let variable_name = input.name.to_snake_case();

            let type_ident = self.mapping_parameter(context, input)?;

            let variable_ident = Ident::new(&variable_name, Span::call_site());

            parameter_list.push(quote!(#variable_ident : #type_ident));
        }

        Ok(quote!(#(#parameter_list,)*))
    }
}

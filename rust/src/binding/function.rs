use ethbind_core::{json::Parameter, Context};
use heck::ToSnakeCase;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use crate::RustBinding;

impl RustBinding {
    #[allow(unused_variables)]
    pub(crate) fn to_fn_generic_list<C: Context>(
        &self,
        context: &mut C,
        tag: &str,
        inputs: &[Parameter],
    ) -> anyhow::Result<TokenStream> {
        let mut generic_list = vec![];

        for (index, _) in inputs.iter().enumerate() {
            let type_ident = Ident::new(&format!("P{}", index), Span::call_site());

            generic_list.push(quote!(#type_ident));
        }

        Ok(quote!(#(#generic_list,)*))
    }

    #[allow(unused_variables)]
    pub(crate) fn to_fn_params_list<C: Context>(
        &self,
        context: &mut C,
        tag: &str,
        inputs: &[Parameter],
    ) -> anyhow::Result<TokenStream> {
        let mut parameter_list = vec![];

        for (index, input) in inputs.iter().enumerate() {
            let variable_name = input.name.to_snake_case();

            let type_ident = Ident::new(&format!("P{}", index), Span::call_site());

            let variable_ident = Ident::new(&variable_name, Span::call_site());

            parameter_list.push(quote!(#variable_ident : #type_ident));
        }

        Ok(quote!(#(#parameter_list,)*))
    }

    #[allow(unused_variables)]
    pub(crate) fn to_fn_generic_where_clause<C: Context>(
        &self,
        context: &mut C,
        tag: &str,
        inputs: &[Parameter],
    ) -> anyhow::Result<TokenStream> {
        let mut where_clauses = vec![];

        for (index, input) in inputs.iter().enumerate() {
            let variable_name = input.name.to_snake_case();

            let type_ident = Ident::new(&format!("P{}", index), Span::call_site());

            let variable_ident = Ident::new(&variable_name, Span::call_site());

            let mapping_type: TokenStream = context
                .mapping_parameter(&self.mapping, input)
                .parse()
                .map_err(|e| anyhow::format_err!("{}", e))?;

            where_clauses.push(
                quote!(#type_ident: TryInto<#mapping_type>, #type_ident::Error: std::error::Error + Sync + Send + 'static),
            );
        }

        Ok(quote!(#(#where_clauses,)*))
    }

    #[allow(unused_variables)]
    pub(crate) fn to_fn_params_try_into_call_list<C: Context>(
        &self,
        context: &mut C,
        tag: &str,
        inputs: &[Parameter],
    ) -> anyhow::Result<TokenStream> {
        let mut try_into_call_list = vec![];

        for (index, input) in inputs.iter().enumerate() {
            let variable_name = input.name.to_snake_case();

            let variable_ident = Ident::new(&variable_name, Span::call_site());

            try_into_call_list.push(quote!(let #variable_ident = #variable_ident.try_into()?));
        }

        Ok(quote!(#(#try_into_call_list;)*))
    }
}

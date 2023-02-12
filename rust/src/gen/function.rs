use ethbind_json::Parameter;
use heck::ToKebabCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::RustGenerator;

impl RustGenerator {
    /// Convert `params` to generic list
    pub(crate) fn to_param_list<R: ethbind_gen::RuntimeBinder>(
        &self,
        _runtime_binder: &mut R,
        params: &[Parameter],
    ) -> anyhow::Result<Vec<TokenStream>> {
        let mut token_streams = vec![];

        for (index, param) in params.iter().enumerate() {
            let type_ident = format_ident!("P{}", index);

            let var_ident = format_ident!("{}", param.name.to_kebab_case());

            token_streams.push(quote!(#var_ident: #type_ident));
        }

        Ok(token_streams)
    }

    /// Convert fn param list to fn generic list
    pub(crate) fn to_generic_list<R: ethbind_gen::RuntimeBinder>(
        &self,
        _runtime_binder: &mut R,
        params: &[Parameter],
    ) -> anyhow::Result<Vec<TokenStream>> {
        let mut token_streams = vec![];

        for (index, _) in params.iter().enumerate() {
            let type_ident = format_ident!("P{}", index);

            token_streams.push(quote!(#type_ident));
        }

        Ok(token_streams)
    }

    /// Convert fn param list to fn where clause list
    pub(crate) fn to_where_clause_list<R: ethbind_gen::RuntimeBinder>(
        &self,
        runtime_binder: &mut R,
        params: &[Parameter],
    ) -> anyhow::Result<Vec<TokenStream>> {
        let mut token_streams = vec![];

        for (index, param) in params.iter().enumerate() {
            let type_ident = format_ident!("P{}", index);

            let try_into_type = self.to_rust_type(runtime_binder, param)?;

            token_streams.push(quote!(#type_ident: TryInto<#try_into_type>, #type_ident::Error: std::error::Error + Syn + Send + 'static));
        }

        Ok(token_streams)
    }

    /// Convert fn param list to try_into statement
    pub(crate) fn to_try_into_list<R: ethbind_gen::RuntimeBinder>(
        &self,
        _runtime_binder: &mut R,
        params: &[Parameter],
    ) -> anyhow::Result<Vec<TokenStream>> {
        let mut token_streams = vec![];

        for (_, param) in params.iter().enumerate() {
            let var_ident = format_ident!("{}", param.name.to_kebab_case());

            token_streams.push(quote!(let #var_ident = #var_ident.try_into()?;));
        }

        Ok(token_streams)
    }

    #[allow(unused)]
    pub(crate) fn to_rust_type<R: ethbind_gen::RuntimeBinder>(
        &self,
        runtime_binder: &mut R,
        param: &Parameter,
    ) -> anyhow::Result<TokenStream> {
        unimplemented!()
    }
}

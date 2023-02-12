use ethbind_gen::RuntimeType;
use ethbind_json::{Parameter, Type};
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

            let var_ident = if param.name != "" {
                format_ident!("{}", param.name)
            } else {
                format_ident!("p{}", index)
            };

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

            token_streams.push(quote!(#type_ident: TryInto<#try_into_type>, #type_ident::Error: std::error::Error + Sync + Send + 'static));
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

        for (index, param) in params.iter().enumerate() {
            let var_ident = if param.name != "" {
                format_ident!("{}", param.name)
            } else {
                format_ident!("p{}", index)
            };

            token_streams.push(quote!(let #var_ident = #var_ident.try_into()?));
        }

        Ok(token_streams)
    }

    /// Convert fn param list to rlp encode statement
    pub(crate) fn to_rlp_encode_list<R: ethbind_gen::RuntimeBinder>(
        &self,
        runtime_binder: &mut R,
        params: &[Parameter],
    ) -> anyhow::Result<Vec<TokenStream>> {
        let mut token_streams = vec![];

        static NULL: Vec<Parameter> = vec![];

        for (_, param) in params.iter().enumerate() {
            let token_stream = self.to_rlp_encode(
                runtime_binder,
                &param.name,
                &param.r#type,
                param.components.as_ref().unwrap_or(&NULL).as_slice(),
            )?;

            token_streams.push(quote!(#token_stream));
        }

        Ok(token_streams)
    }

    pub(crate) fn to_rlp_decode_list<R: ethbind_gen::RuntimeBinder>(
        &self,
        runtime_binder: &mut R,
        params: &[Parameter],
    ) -> anyhow::Result<TokenStream> {
        let mut token_streams = vec![];

        static NULL: Vec<Parameter> = vec![];

        for (_, param) in params.iter().enumerate() {
            let token_stream = self.to_rlp_decode(
                runtime_binder,
                &param.r#type,
                param.components.as_ref().unwrap_or(&NULL).as_slice(),
            )?;

            token_streams.push(quote!(#token_stream));
        }

        Ok(quote!((#(#token_streams,)*)))
    }

    #[allow(unused)]
    pub(crate) fn to_rust_type<R: ethbind_gen::RuntimeBinder>(
        &self,
        runtime_binder: &mut R,
        param: &Parameter,
    ) -> anyhow::Result<TokenStream> {
        if let Some(runtime_type) = runtime_binder.to_runtime_type(&param.r#type)? {
            let runtime_type: TokenStream = runtime_type
                .declare_type()
                .parse()
                .map_err(|err| anyhow::format_err!("{}", err))?;

            return Ok(runtime_type);
        } else {
            let components = param
                .components
                .as_ref()
                .expect("Tuple parameter's components field is None");

            let mut tuple_token_streams = vec![];

            for c in components {
                tuple_token_streams.push(self.to_rust_type(runtime_binder, c)?);
            }

            return Ok(quote!((#(#tuple_token_streams,)*)));
        }
    }

    pub(crate) fn to_outputs_type<R: ethbind_gen::RuntimeBinder>(
        &self,
        runtime_binder: &mut R,
        outputs: &[Parameter],
    ) -> anyhow::Result<TokenStream> {
        let mut tuple_token_streams = vec![];
        for param in outputs {
            tuple_token_streams.push(self.to_rust_type(runtime_binder, param)?);
        }

        Ok(quote!((#(#tuple_token_streams,)*)))
    }

    #[allow(unused)]
    pub(crate) fn to_rlp_encode<R: ethbind_gen::RuntimeBinder>(
        &self,
        runtime_binder: &mut R,
        var_name: &str,
        r#type: &Type,
        components: &[Parameter],
    ) -> anyhow::Result<TokenStream> {
        if let Some(runtime_type) = runtime_binder.to_runtime_type(r#type)? {
            let runtime_type: TokenStream = runtime_type
                .rlp_encode(var_name, "outputs")
                .parse()
                .map_err(|err| anyhow::format_err!("{}", err))?;

            return Ok(runtime_type);
        } else {
            let mut tuple_token_streams = vec![];

            static NULL: Vec<Parameter> = vec![];

            for (index, c) in components.iter().enumerate() {
                tuple_token_streams.push(self.to_rlp_encode(
                    runtime_binder,
                    &format!("{}.{}", var_name, index),
                    &c.r#type,
                    c.components.as_ref().unwrap_or(&NULL).as_slice(),
                )?);
            }

            return Ok(quote!((#(#tuple_token_streams,)*)));
        }
    }

    pub(crate) fn to_rlp_decode<R: ethbind_gen::RuntimeBinder>(
        &self,
        runtime_binder: &mut R,
        r#type: &Type,
        components: &[Parameter],
    ) -> anyhow::Result<TokenStream> {
        if let Some(runtime_type) = runtime_binder.to_runtime_type(r#type)? {
            let runtime_type: TokenStream = runtime_type
                .rlp_decode("inputs")
                .parse()
                .map_err(|err| anyhow::format_err!("{}", err))?;

            return Ok(runtime_type);
        } else {
            let mut tuple_token_streams = vec![];

            static NULL: Vec<Parameter> = vec![];

            for c in components.iter() {
                tuple_token_streams.push(self.to_rlp_decode(
                    runtime_binder,
                    &c.r#type,
                    c.components.as_ref().unwrap_or(&NULL).as_slice(),
                )?);
            }

            return Ok(quote!((#(#tuple_token_streams,)*)));
        }
    }
}

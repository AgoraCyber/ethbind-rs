use ethbind_json::{Event, Parameter, Type};
use heck::ToSnakeCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::RustGenerator;

impl RustGenerator {
    pub(crate) fn to_event_field_list<R: ethbind_gen::RuntimeBinder>(
        &self,
        runtime_binder: &mut R,
        params: &[Parameter],
    ) -> anyhow::Result<Vec<TokenStream>> {
        let mut token_streams = vec![];

        for (index, param) in params.iter().enumerate() {
            let type_ident = self.to_rust_type(runtime_binder, param)?;

            let var_ident = if param.name != "" {
                format_ident!("{}", param.name.to_snake_case())
            } else {
                format_ident!("p{}", index)
            };

            token_streams.push(quote!(#var_ident: #type_ident));
        }

        Ok(token_streams)
    }

    pub(crate) fn to_event_init_list<R: ethbind_gen::RuntimeBinder>(
        &self,
        _runtime_binder: &mut R,
        params: &[Parameter],
    ) -> anyhow::Result<Vec<TokenStream>> {
        let mut token_streams = vec![];

        for (index, param) in params.iter().enumerate() {
            let var_ident = if param.name != "" {
                format_ident!("{}", param.name.to_snake_case())
            } else {
                format_ident!("p{}", index)
            };

            token_streams.push(quote!(#var_ident));
        }

        Ok(token_streams)
    }

    pub(crate) fn to_event_docode_token_streams<R: ethbind_gen::RuntimeBinder>(
        &self,
        runtime_binder: &mut R,
        event: &Event,
    ) -> anyhow::Result<TokenStream> {
        let mut token_streams = vec![];

        let base_index = if event.anonymous { 0 } else { 1 };

        for (index, param) in event.inputs.iter().enumerate() {
            let var_ident = if param.name != "" {
                format_ident!("{}", param.name.to_snake_case())
            } else {
                format_ident!("p{}", index)
            };

            match &param.r#type {
                Type::Simple(simple) if simple.is_tuple() => {
                    let token_stream = self.decode_from_data(runtime_binder, param)?;

                    token_streams.push(quote!(let #var_ident = #token_stream));
                }
                _ => {
                    if param.indexed {
                        let index = base_index + index;
                        token_streams
                            .push(quote!(let #var_ident = decoder.topic_abi_decode(#index)?));
                    } else {
                        let token_stream = self.decode_from_data(runtime_binder, param)?;

                        token_streams.push(quote!(let #var_ident = #token_stream));
                    }
                }
            }
        }

        Ok(quote!(#(#token_streams;)*))
    }

    fn decode_from_data<R: ethbind_gen::RuntimeBinder>(
        &self,
        runtime_binder: &mut R,
        parameter: &Parameter,
    ) -> anyhow::Result<TokenStream> {
        if runtime_binder.to_runtime_type(&parameter.r#type)?.is_some() {
            return Ok(quote!(decoder.data_decoder().abi_decode()?));
        } else {
            let mut tuple_token_streams = vec![];

            for c in parameter
                .components
                .as_ref()
                .expect("Tuple componenets is None")
                .iter()
            {
                tuple_token_streams.push(self.to_abi_decode(runtime_binder, c)?);
            }

            return Ok(quote! {{
                decoder.data_decoder().abi_start_decode_tuple()?;
                let result = (#(#tuple_token_streams,)*);
                decoder.data_decoder().abi_end_decode_tuple()?;
                result
            }});
        }
    }
}

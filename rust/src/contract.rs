use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

pub(crate) struct ContractBinding {
    pub(crate) contract_ident: Ident,
    impl_token_streams: Vec<TokenStream>,
    error_token_streams: Vec<(Ident, TokenStream)>,
    event_token_streams: Vec<(Ident, TokenStream)>,
    tuple_token_streams: Vec<(Ident, TokenStream)>,
}

impl ContractBinding {
    pub(crate) fn new(contract_ident: Ident) -> Self {
        Self {
            contract_ident,
            impl_token_streams: Default::default(),
            error_token_streams: Default::default(),
            event_token_streams: Default::default(),
            tuple_token_streams: Default::default(),
        }
    }
    pub(crate) fn add_impl_stream(&mut self, token_stream: TokenStream) -> &mut Self {
        self.impl_token_streams.push(token_stream);

        self
    }

    pub(crate) fn add_tuple_stream(
        &mut self,
        event_ident: Ident,
        token_stream: TokenStream,
    ) -> &mut Self {
        self.tuple_token_streams.push((event_ident, token_stream));

        self
    }

    pub(crate) fn add_error_stream(
        &mut self,
        event_ident: Ident,
        token_stream: TokenStream,
    ) -> &mut Self {
        self.error_token_streams.push((event_ident, token_stream));

        self
    }

    pub(crate) fn add_event_stream(
        &mut self,
        event_ident: Ident,
        token_stream: TokenStream,
    ) -> &mut Self {
        self.event_token_streams.push((event_ident, token_stream));

        self
    }

    pub(crate) fn gen_codes(
        &self,
        rt_context: &TokenStream,
        rt_event: &TokenStream,
        rt_error: &TokenStream,
    ) -> TokenStream {
        let fns = &self.impl_token_streams;

        let ident = &self.contract_ident;

        let event_enum_ident = format_ident!("{}Events", ident);

        let error_enum_ident = format_ident!("{}Errors", ident);

        let event_match_patterns = self
            .event_token_streams
            .iter()
            .map(|(ident, expr)| {
                quote! {
                    #ident => #expr
                }
            })
            .collect::<Vec<_>>();

        let event_enum_items = self
            .event_token_streams
            .iter()
            .map(|(ident, _)| quote!(#ident))
            .collect::<Vec<_>>();

        let error_match_patterns = self
            .error_token_streams
            .iter()
            .map(|(ident, expr)| {
                quote! {
                    #ident => #expr
                }
            })
            .collect::<Vec<_>>();

        let error_enum_items = self
            .error_token_streams
            .iter()
            .map(|(ident, _)| quote!(#ident))
            .collect::<Vec<_>>();

        let tuple_token_streams = self
            .tuple_token_streams
            .iter()
            .map(|(_, s)| s)
            .collect::<Vec<_>>();

        quote! {
            pub struct #ident(#rt_context);

            impl #ident {
                #(#fns)*

                pub fn event_decodable(event_type: #event_enum_ident) -> #rt_event {
                    match event_type {
                        #(#event_match_patterns,)*
                    }
                }

                pub fn error_decodable(error_type: #error_enum_ident) -> #rt_error {
                    match error_type {
                        #(#error_match_patterns,)*
                    }
                }
            }

            pub enum #error_enum_ident {
                #(#error_enum_items,)*
            }

            pub enum #event_enum_ident {
                #(#event_enum_items,)*
            }

            #(#tuple_token_streams)*
        }
    }
}

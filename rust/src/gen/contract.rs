use proc_macro2::TokenStream;

#[derive(Debug, Default)]
#[allow(unused)]
pub(crate) struct ContractGenerator {
    contract_name: String,
    fn_token_streams: Vec<TokenStream>,
    event_token_streams: Vec<TokenStream>,
    error_token_streams: Vec<TokenStream>,
}

impl ContractGenerator {
    pub(crate) fn new(contract_name: &str) -> Self {
        Self {
            contract_name: contract_name.to_owned(),
            ..Default::default()
        }
    }

    pub(crate) fn add_fn_token_stream(&mut self, token_stream: TokenStream) {
        self.fn_token_streams.push(token_stream);
    }
}

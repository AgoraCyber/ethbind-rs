use ethbind_core::{json::Parameter, Context};
use proc_macro2::TokenStream;
use regex::Regex;

use crate::RustBinding;

impl RustBinding {
    /// Convert contract parameter to token stream of rt decodable param construct expr
    pub(crate) fn to_decodable_token_stream<C: Context>(
        &self,
        context: &mut C,
        parameter: &Parameter,
    ) -> anyhow::Result<TokenStream> {
        let mapping_type = self.mapping_parameter(context, parameter)?;

        let mapping_type = mapping_type.to_string();

        let regex = Regex::new(r"<.+>").unwrap();

        let mut current = mapping_type.clone();

        for m in regex.find_iter(&mapping_type) {
            current = current.replace(m.as_str(), &format!("::{}", m.as_str()));
        }

        self.get_mapping_token_stream(
            "rt_to_decodable",
            &[
                ("$type", &current),
                ("$indexed", &parameter.indexed.to_string()),
            ],
        )
    }

    pub(crate) fn to_decodable_token_streams<C: Context>(
        &self,
        context: &mut C,
        inputs: &[Parameter],
    ) -> anyhow::Result<Vec<TokenStream>> {
        let mut streams = vec![];

        for input in inputs {
            streams.push(self.to_decodable_token_stream(context, input)?);
        }

        Ok(streams)
    }
}

mod gen;
pub use gen::*;

mod pretty;
pub use pretty::*;

mod token_stream;
pub use token_stream::*;

mod error;
pub use error::*;

pub type BindingBuilder = ethbind_gen::BindingBuilder<
    ethbind_gen::Executor<RustGenerator, ethbind_gen::JsonRuntimeBinder>,
>;

pub use ethbind_gen::*;
pub use ethbind_json::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen() {
        _ = pretty_env_logger::try_init();

        let runtime_binder: JsonRuntimeBinder = include_str!("../macros/tests/mapping.json")
            .parse()
            .expect("Parse mapping");

        let mut contracts = BindingBuilder::new((RustGenerator::default(), runtime_binder))
            .bind_hardhat(include_str!("../macros/tests/abi.json"))
            .finalize()
            .expect("Generate data");

        contracts.pretty().expect("Pretty");

        // contracts.save_to("./");
    }
}

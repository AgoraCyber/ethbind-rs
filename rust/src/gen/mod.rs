mod contract;
use contract::*;

/// The rust language generator for `Ethbind`
#[derive(Debug, Default)]
pub struct RustGenerator {
    contracts: Vec<ContractGenerator>,
}

impl RustGenerator {
    /// Push new contract generator to back end of generation list
    pub(crate) fn new_contract(&mut self, name: &str) {
        self.contracts.push(ContractGenerator::new(name))
    }

    /// Returns contract generator at back edn of generation list.
    pub(crate) fn current_contract(&mut self) -> &mut ContractGenerator {
        self.contracts.last_mut().expect("Call new_contract first")
    }

    pub(crate) fn to_runtime_type_ident<R: ethbind_gen::RuntimeBinder>(
        &self,
        runtime_binder: &mut R,
        name: &str,
    ) -> anyhow::Result<Ident> {
        Ok(format_ident!(
            "{}",
            runtime_binder.get(name)?.declare_type()
        ))
    }
}

mod generator;
use ethbind_gen::RuntimeType;
pub use generator::*;

mod function;
pub use function::*;
use proc_macro2::Ident;
use quote::format_ident;

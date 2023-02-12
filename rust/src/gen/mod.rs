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
}

mod generator;
pub use generator::*;

mod function;
pub use function::*;

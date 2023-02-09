use super::{mapping::SerdeTypeMapping, Generator};

/// The generator implemented for the target `Rust`
pub struct RustBinding {
    mapping: SerdeTypeMapping,
}

impl RustBinding {
    /// Create new `RustBinding` instance by parameter `mapping`
    pub fn new(mapping: SerdeTypeMapping) -> RustBinding {
        Self { mapping }
    }
}

#[allow(unused)]
impl Generator for RustBinding {
    type TypeMapping = SerdeTypeMapping;
    fn generate_deploy(
        &mut self,
        ctx: &super::Context,
        bytecode: &str,
        inputs: &[crate::json::Parameter],
    ) -> anyhow::Result<()> {
        unimplemented!()
    }

    fn generate_error(
        &mut self,
        ctx: &super::Context,
        event: &crate::json::Error,
    ) -> anyhow::Result<()> {
        unimplemented!()
    }

    fn generate_event(
        &mut self,
        ctx: &super::Context,
        event: &crate::json::Event,
    ) -> anyhow::Result<()> {
        unimplemented!()
    }

    fn generate_function(
        &mut self,
        ctx: &super::Context,
        function: &crate::json::Function,
    ) -> anyhow::Result<()> {
        unimplemented!()
    }

    fn generate_tuple(
        &mut self,
        ctx: &super::Context,
        name: &str,
        tuple: &[crate::json::Parameter],
    ) -> anyhow::Result<()> {
        unimplemented!()
    }

    fn type_mapping(&self) -> &Self::TypeMapping {
        &self.mapping
    }
}

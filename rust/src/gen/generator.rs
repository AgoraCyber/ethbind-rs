use ethbind_gen::Generator;
use ethbind_json::*;

use crate::RustGenerator;

#[allow(unused)]
impl Generator for RustGenerator {
    fn begin<R: ethbind_gen::RuntimeBinder>(
        &mut self,
        runtime_binder: &mut R,
        name: &str,
    ) -> anyhow::Result<()> {
        self.new_contract(name);

        Ok(())
    }

    fn end<R: ethbind_gen::RuntimeBinder>(
        &mut self,
        runtime_binder: &mut R,
        name: &str,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn finalize(self) -> Vec<ethbind_gen::Contract> {
        unimplemented!()
    }

    fn generate_deploy<R: ethbind_gen::RuntimeBinder>(
        &mut self,
        runtime_binder: &mut R,
        contructor: &Constructor,
        deploy_bytes: &str,
    ) -> anyhow::Result<()> {
        unimplemented!()
    }

    fn generate_error<R: ethbind_gen::RuntimeBinder>(
        &mut self,
        runtime_binder: &mut R,
        error: &Error,
    ) -> anyhow::Result<()> {
        unimplemented!()
    }

    fn generate_event<R: ethbind_gen::RuntimeBinder>(
        &mut self,
        runtime_binder: &mut R,
        event: &Event,
    ) -> anyhow::Result<()> {
        unimplemented!()
    }

    fn generate_fn<R: ethbind_gen::RuntimeBinder>(
        &mut self,
        runtime_binder: &mut R,
        r#fn: &Function,
    ) -> anyhow::Result<()> {
        unimplemented!()
    }
}

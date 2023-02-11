use ethbind_json::*;

use ethbind_gen::{Contract, Generator};

/// The rust language generator for `Ethbind`
#[derive(Debug, Default)]
pub struct RustGenerator {}

#[allow(unused)]
impl Generator for RustGenerator {
    fn begin<R: ethbind_gen::RuntimeBinder>(
        &mut self,
        runtime_binder: &R,
        name: &str,
    ) -> anyhow::Result<()> {
        unimplemented!()
    }

    fn finalize(self) -> Vec<ethbind_gen::Contract> {
        unimplemented!()
    }

    fn generate_deploy<R: ethbind_gen::RuntimeBinder>(
        &mut self,
        runtime_binder: &R,
        contructor: &Constructor,
        deploy_bytes: &str,
    ) -> anyhow::Result<()> {
        unimplemented!()
    }

    fn generate_error<R: ethbind_gen::RuntimeBinder>(
        &mut self,
        runtime_binder: &R,
        error: &Error,
    ) -> anyhow::Result<()> {
        unimplemented!()
    }

    fn generate_event<R: ethbind_gen::RuntimeBinder>(
        &mut self,
        runtime_binder: &R,
        event: &Event,
    ) -> anyhow::Result<()> {
        unimplemented!()
    }

    fn generate_fn<R: ethbind_gen::RuntimeBinder>(
        &mut self,
        runtime_binder: &R,
        r#fn: &Function,
    ) -> anyhow::Result<()> {
        unimplemented!()
    }
}

/// The trait to support `Rust` language formatting
pub trait RustPretty {
    /// Invoke this `fn` to perform the `formatting codes action`
    fn pretty(&mut self) -> anyhow::Result<()>;
}

impl RustPretty for Contract {
    fn pretty(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
}

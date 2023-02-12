use ethbind_gen::Contract;

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

//! This crate provide core types of ethbind generation system
//!
//!
//!

use ethbind_json::{Constructor, Error, Event, Function, Type};

/// ABI data structure that can be generated into arbitrary programming language supported by `Ethbind`.
///
/// `Ethbind` provides `Generatable` implementations for types of crate [`ethbind-json`](https://docs.rs/ethbind-json)
pub trait Generatable {
    /// Generate abi data to `Target` programming language code.
    fn generate<C: Context>(&self, context: C) -> anyhow::Result<()>;
}

/// `Ethbind` code generation system `Context` instance
pub trait Context {
    /// Target programming language runtime/strongly typed binder
    type Runtime: RuntimeBinder;

    /// `Target` programming language code generator
    type Language: Generator;

    /// Get context binding programming language [`Generator`]
    fn generator(&mut self) -> &mut Self::Language;

    /// Get context binding programming language [`runtime binder`](RuntimeBinder)
    fn runtime(&self) -> &Self::Runtime;
}

/// Runtime type trait for [`RuntimeBinder`]
pub trait RuntimeType {
    /// Type grammar when declare. e.g, `Vec<u8>`
    fn declare_type(&self) -> &str;
    /// Type grammar when reference. e.g, `Vec::<u8>`
    fn ref_type(&self) -> &str;
    /// Rlp encode calling syntax, parameter `variable_name` is the name of variable to encode
    fn rlp_encode(&self, variable_name: &str) -> &str;
    /// Rlp decode calling syntax, parameter `inputs_variable_name` is the name of variable for `input data`
    fn rlp_decode(&self, inputs_variable_name: &str) -> &str;
}

/// Binder for mapping contract type system to `target` programming language runtime type system.
pub trait RuntimeBinder {
    /// Runtime type.
    type RuntimeType: RuntimeType;

    /// Convert contract [`abi type`](Type) to [`runtime type`](RuntimeBinder::RuntimeType)
    fn to_runtime_type(&self, r#type: Type) -> anyhow::Result<Self::RuntimeType>;
}

/// Programming language code generator supported by `Ethbind`.
///
/// The implementation must support multi-round generation process.
pub trait Generator {
    /// [`Generatable`] or `Executor` call this fn to start a new contract generation round.
    fn begin<C: Context>(&mut self, context: C, name: &str) -> anyhow::Result<()>;

    /// Generate contract method ,call this fn after call [`begin`](Generator::begin) at least once.
    fn generate_fn<C: Context>(&mut self, context: C, r#fn: Function) -> anyhow::Result<()>;

    /// Generate contract deploy method ,call this fn after call [`begin`](Generator::begin) at least once.
    fn generate_deploy<C: Context>(
        &mut self,
        context: C,
        contructor: Constructor,
    ) -> anyhow::Result<()>;

    /// Generate event handle interface ,call this fn after call [`begin`](Generator::begin) at least once.
    fn generate_event<C: Context>(&mut self, context: C, event: Event) -> anyhow::Result<()>;

    /// Generate error handle interface ,call this fn after call [`begin`](Generator::begin) at least once.
    fn generate_error<C: Context>(&mut self, context: C, error: Error) -> anyhow::Result<()>;

    /// Close generator and return generated contract codes.
    fn finalize(self) -> Vec<CodeContract>;
}

/// Generated contract codes package
pub struct CodeContract {
    pub files: Vec<CodeFile>,
}

/// Generated code data and file name
pub struct CodeFile {
    pub name: String,
    pub data: String,
}

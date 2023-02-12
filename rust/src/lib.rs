mod gen;
use ethbind_gen::{Executor, JsonRuntimeBinder};
pub use gen::*;

mod pretty;
pub use pretty::*;

pub type BindingBuilder = ethbind_gen::BindingBuilder<Executor<RustGenerator, JsonRuntimeBinder>>;

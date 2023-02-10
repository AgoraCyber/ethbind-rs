use crate::{
    json::{AbiField, Error, Event, Function, HardhatArtifact, Parameter},
    Context, TypeMapping,
};

/// Target language root generator trait
pub trait Generator {
    type TypeMapping: TypeMapping;

    /// Return generator hold [`TypeMapping`] instance
    fn type_mapping(&self) -> &Self::TypeMapping;

    /// Generate contract scope ,tuple/structure types
    fn begin<C: Context>(&mut self, ctx: &mut C, contract_name: &str) -> anyhow::Result<()>;

    fn mapping_tuple<C: Context>(
        &mut self,
        ctx: &mut C,
        name: &str,
        tuple: &[Parameter],
    ) -> anyhow::Result<String>;

    /// Generate contract scope ,tuple/structure types
    fn generate_tuple<C: Context>(
        &mut self,
        ctx: &mut C,
        r_type: &str,
        tuple: &[Parameter],
    ) -> anyhow::Result<()>;

    /// Call this function to generate deploy method/function
    fn generate_deploy<C: Context>(
        &mut self,
        ctx: &mut C,
        bytecode: &str,
        inputs: &[Parameter],
    ) -> anyhow::Result<()>;

    /// Generate constract interface function bind code
    fn generate_function<C: Context>(
        &mut self,
        ctx: &mut C,
        function: &Function,
    ) -> anyhow::Result<()>;

    /// Generate constract interface event bind code
    fn generate_event<C: Context>(&mut self, ctx: &mut C, event: &Event) -> anyhow::Result<()>;

    /// Generate constract interface error bind code
    fn generate_error<C: Context>(&mut self, ctx: &mut C, event: &Error) -> anyhow::Result<()>;
}

/// Types that support code generation must implement this trait.
///
/// This trait is used by [`Executor`] method [`generate`](crate::Executor::generate)
pub trait Generate {
    /// Generate target codes by parameter `context` and `generator`
    fn generate<C, G>(&self, context: &mut C, generator: &mut G) -> anyhow::Result<()>
    where
        C: Context,
        G: Generator;

    /// Types that support tuple type generation should implement this fn
    #[allow(unused)]
    fn register_tuples<C>(&self, context: &mut C)
    where
        C: Context,
    {
    }
}

impl Generate for HardhatArtifact {
    fn generate<C, G>(&self, context: &mut C, generator: &mut G) -> anyhow::Result<()>
    where
        C: Context,
        G: Generator,
    {
        // Try get constructor input parameter list
        let mut inputs = [].as_slice();

        for field in &self.abi {
            if let AbiField::Constructor(c) = field {
                inputs = c.inputs.as_slice();
                break;
            }
        }

        // Generate contract struct/class code binding
        self.abi.generate(context, generator)?;

        // Generate deploy method/function code binding
        generator.generate_deploy(context, &self.bytecode, inputs)?;

        Ok(())
    }

    fn register_tuples<C>(&self, context: &mut C)
    where
        C: Context,
    {
        self.abi.register_tuples(context)
    }
}

impl Generate for Vec<AbiField> {
    fn generate<C, G>(&self, context: &mut C, generator: &mut G) -> anyhow::Result<()>
    where
        C: Context,
        G: Generator,
    {
        for field in self {
            match field {
                AbiField::Function(function) => {
                    generator.generate_function(context, function)?;
                }
                AbiField::Event(event) => {
                    generator.generate_event(context, event)?;
                }
                AbiField::Error(error) => {
                    generator.generate_error(context, error)?;
                }
                _ => {
                    // Skip generate codes for constructor/receive/fallback.
                    // - Call `Generator::generate_deploy` for [`HardhatArtifact`]'s trait `Generate` to generate the constructor's binding code.
                    // - It seems that end users do not directly call receive/fallback api, so we skip their generation step.
                }
            }
        }

        Ok(())
    }

    fn register_tuples<C>(&self, context: &mut C)
    where
        C: Context,
    {
        for field in self {
            match field {
                AbiField::Constructor(constructor) => {
                    register_tuples(context, constructor.inputs.as_ref());
                }
                AbiField::Function(function) => {
                    register_tuples(context, function.inputs.as_ref());
                    register_tuples(context, function.outputs.as_ref());
                }

                AbiField::Event(event) => {
                    register_tuples(context, event.inputs.as_ref());
                }
                AbiField::Error(error) => {
                    register_tuples(context, error.inputs.as_ref());
                }
                _ => {}
            }
        }
    }
}

fn register_tuples<C: Context>(context: &mut C, params: &[Parameter]) {
    for param in params {
        if param.components.is_some() {
            context.register_tuple(param);
        }
    }
}

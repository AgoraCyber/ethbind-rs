use std::collections::HashMap;

use crate::{
    error::TypeMappingError,
    json::{
        AbiField, Error, Event, FixedMN, Function, HardhatArtifact, IntegerM, Parameter,
        SimpleType, Type,
    },
};

/// Code generator context structure
#[derive(Debug, Default)]
pub struct Context {
    constract_name: String,
    tuples: HashMap<String, Vec<Parameter>>,
    mapping_tuples: HashMap<String, String>,
}

impl Context {
    fn reigster_tuple(&mut self, name: &str, tuple: &[Parameter]) {
        if !self.tuples.contains_key(name) {
            self.tuples.insert(name.to_string(), tuple.into());
        }
    }

    fn register_mapping_tuple_type(&mut self, name: &str, mapping_type_path: String) {
        if !self.mapping_tuples.contains_key(name) {
            self.mapping_tuples
                .insert(name.to_string(), mapping_type_path);
        }
    }

    /// Get generating contract name.
    pub fn contract_name(&self) -> &str {
        &self.constract_name
    }

    pub fn mapping_parameter<M: TypeMapping>(
        &mut self,
        type_mapping: &M,
        parameter: &Parameter,
    ) -> String {
        static NULL_TUPLE: Vec<Parameter> = vec![];
        self.get_mapping_type(
            type_mapping,
            parameter.r#type.clone(),
            &parameter.components.as_ref().unwrap_or(&NULL_TUPLE),
        )
    }

    fn get_mapping_type<M: TypeMapping>(
        &mut self,
        type_mapping: &M,
        r#type: Type,
        tuple: &[Parameter],
    ) -> String {
        match r#type {
            Type::Simple(simple) if simple.is_tuple() => {
                let key = generate_tuple_type_declare(self, tuple);

                let mapping_path = self
                    .mapping_tuples
                    .get(&key)
                    .ok_or(TypeMappingError::NotFound(key))
                    .unwrap();

                mapping_path.to_owned()
            }
            Type::Simple(simple) => type_mapping.simple(&simple),
            Type::Array(array) => {
                type_mapping.array(&self.get_mapping_type(type_mapping, array.element, &[]))
            }
            Type::ArrayM(array_m) => type_mapping.array_m(
                &self.get_mapping_type(type_mapping, array_m.element, &[]),
                array_m.m,
            ),

            Type::BytesM(bytes_m) => type_mapping.bytes_m(bytes_m.m),

            Type::FixedMN(fixed_m_n) => type_mapping.fixed_m_n(fixed_m_n),

            Type::IntegerM(int_m) => type_mapping.integer_m(int_m),
        }
    }
}

/// Target language root generator trait
pub trait Generator {
    type TypeMapping: TypeMapping;

    /// Return generator hold [`TypeMapping`] instance
    fn type_mapping(&self) -> &Self::TypeMapping;

    /// Generate contract scope ,tuple/structure types
    fn start_generate_contract(&mut self, ctx: &mut Context) -> anyhow::Result<()>;

    /// Generate contract scope ,tuple/structure types
    fn generate_tuple(
        &mut self,
        ctx: &mut Context,
        name: &str,
        tuple: &[Parameter],
    ) -> anyhow::Result<String>;

    /// Call this function to generate deploy method/function
    fn generate_deploy(
        &mut self,
        ctx: &mut Context,
        bytecode: &str,
        inputs: &[Parameter],
    ) -> anyhow::Result<()>;

    /// Generate constract interface function bind code
    fn generate_function(&mut self, ctx: &mut Context, function: &Function) -> anyhow::Result<()>;

    /// Generate constract interface event bind code
    fn generate_event(&mut self, ctx: &mut Context, event: &Event) -> anyhow::Result<()>;

    /// Generate constract interface error bind code
    fn generate_error(&mut self, ctx: &mut Context, event: &Error) -> anyhow::Result<()>;
}

pub trait TypeMapping {
    fn simple(&self, t: &SimpleType) -> String;

    fn bytes_m(&self, m: usize) -> String;

    fn integer_m(&self, t: IntegerM) -> String;

    fn fixed_m_n(&self, t: FixedMN) -> String;

    fn array_m(&self, element: &str, m: usize) -> String;

    fn array(&self, element: &str) -> String;
}

/// The trait to support generate target program language code
trait Generate {
    /// Generate new contract context structure with providing `constract_name`
    fn create_context(&self, constract_name: &str) -> anyhow::Result<Context>;

    /// Using providing `context` object to generate projection codes.
    ///
    /// Usually can invoke [`Self::create_context`](Generate::create_context) to generate valid context instance.
    fn generate_with_context<G>(
        &self,
        context: &mut Context,
        generator: &mut G,
    ) -> anyhow::Result<()>
    where
        G: Generator;

    /// Generate contract binding code by `constract_name` and `generator`
    ///
    /// # Parameters
    ///
    /// - `constract_name` The associated contract name for generation code,
    ///                    usually that is the declare name of class/struct
    /// - `generator` Special projection language codes generator, e.g, rust/c++/typescript/..
    fn generate<G>(&self, constract_name: &str, generator: &mut G) -> anyhow::Result<()>
    where
        G: Generator,
    {
        let mut context = self.create_context(constract_name)?;

        self.generate_with_context(&mut context, generator)
    }
}

impl Generate for HardhatArtifact {
    fn generate_with_context<G>(
        &self,
        context: &mut Context,
        generator: &mut G,
    ) -> anyhow::Result<()>
    where
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

        // Generate deploy method/function code binding
        generator.generate_deploy(context, &self.bytecode, inputs)?;

        // Generate contract struct/class code binding
        self.abi.generate_with_context(context, generator)?;

        Ok(())
    }

    fn create_context(&self, constract_name: &str) -> anyhow::Result<Context> {
        self.abi.create_context(constract_name)
    }
}

impl Generate for Vec<AbiField> {
    fn create_context(&self, constract_name: &str) -> anyhow::Result<Context> {
        let mut context = Context::default();

        context.constract_name = constract_name.to_owned();

        for field in self {
            match field {
                AbiField::Constructor(constructor) => {
                    handle_input_output_params(&mut context, constructor.inputs.as_ref());
                }
                AbiField::Function(function) => {
                    handle_input_output_params(&mut context, function.inputs.as_ref());
                    handle_input_output_params(&mut context, function.outputs.as_ref());
                }

                AbiField::Event(event) => {
                    handle_input_output_params(&mut context, event.inputs.as_ref());
                }
                AbiField::Error(error) => {
                    handle_input_output_params(&mut context, error.inputs.as_ref());
                }
                _ => {}
            }
        }

        Ok(context)
    }

    fn generate_with_context<G>(
        &self,
        context: &mut Context,
        generator: &mut G,
    ) -> anyhow::Result<()>
    where
        G: Generator,
    {
        generator.start_generate_contract(context)?;

        // Generate contract references tuples
        let tuples = context.tuples.clone();
        for (k, v) in tuples.iter() {
            let mapping_type_path = generator.generate_tuple(context, k, v)?;

            context.register_mapping_tuple_type(k, mapping_type_path);
        }

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
}

/// Register one contract's tuples.
fn handle_input_output_params(context: &mut Context, parms: &Vec<Parameter>) {
    for param in parms {
        if let Some(tuple) = &param.components {
            let el = generate_tuple_type_declare(context, tuple);

            context.reigster_tuple(&el, tuple);
        }
    }
}

fn generate_tuple_type_declare(context: &mut Context, components: &[Parameter]) -> String {
    let mut els = vec![];

    for parm in components {
        if let Some(child_components) = &parm.components {
            let el = generate_tuple_type_declare(context, child_components);

            context.reigster_tuple(&el, child_components);

            els.push(el);
        }
    }

    format!("({})", els.join(","))
}

pub mod mapping;
pub mod rust;

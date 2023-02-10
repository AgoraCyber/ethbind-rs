use std::{collections::HashMap, fs::read_to_string, path::PathBuf};

use crate::{
    error::TypeMappingError,
    json::{AbiField, HardhatArtifact, Parameter, Type},
    Generate, Generator, TypeMapping,
};

pub trait Context {
    /// Register new tuple type to context instance
    ///
    /// Returns tuple context scope unique name.
    fn register_tuple(&mut self, tuple: &[Parameter]) -> String;

    /// Try register tuple into context, if exists returns false.
    fn reigster_tuple_mapping(&mut self, name: &str, to_type_path: String) -> bool;

    /// Mapping parameter type to target language runtime type
    fn mapping_parameter<TM: TypeMapping>(
        &mut self,
        type_mapping: &TM,
        parameter: &Parameter,
    ) -> String;
}

/// Code generator context instance.
#[derive(Debug, Default)]
#[allow(unused)]
pub struct Executor {
    tuples: HashMap<String, Vec<Parameter>>,
    mapping_tuples: HashMap<String, String>,
}

#[allow(unused)]
impl Context for Executor {
    fn mapping_parameter<TM: TypeMapping>(
        &mut self,
        type_mapping: &TM,
        parameter: &Parameter,
    ) -> String {
        static NULL_TUPLE: Vec<Parameter> = vec![];
        self.mapping_type(
            type_mapping,
            parameter.r#type.clone(),
            &parameter.components.as_ref().unwrap_or(&NULL_TUPLE),
        )
    }

    fn register_tuple(&mut self, tuple: &[Parameter]) -> String {
        self.tuple_name(tuple)
    }

    fn reigster_tuple_mapping(&mut self, name: &str, to_type_path: String) -> bool {
        if !self.mapping_tuples.contains_key(name) {
            self.mapping_tuples.insert(name.to_string(), to_type_path);
            true
        } else {
            false
        }
    }
}

impl Executor {
    /// Generate tuple context scope unique name
    fn tuple_name(&mut self, tuple: &[Parameter]) -> String {
        let mut els = vec![];

        for parm in tuple {
            if let Some(child_components) = &parm.components {
                let el = self.tuple_name(child_components);

                els.push(el);
            }
        }

        let key = format!("({})", els.join(","));

        // cascade register tuple
        self.add_tuple(&key, tuple);

        key
    }

    /// Add tuple if not register
    fn add_tuple(&mut self, name: &str, tuple: &[Parameter]) {
        if !self.tuples.contains_key(name) {
            self.tuples.insert(name.to_string(), tuple.into());
        }
    }

    /// Mapping abi json type to target language runtime type
    fn mapping_type<TM: TypeMapping>(
        &mut self,
        type_mapping: &TM,
        r#type: Type,
        tuple: &[Parameter],
    ) -> String {
        match r#type {
            Type::Simple(simple) if simple.is_tuple() => {
                let key = self.tuple_name(tuple);

                let mapping_path = self
                    .mapping_tuples
                    .get(&key)
                    .ok_or(TypeMappingError::NotFound(key))
                    .unwrap();

                mapping_path.to_owned()
            }
            Type::Simple(simple) => type_mapping.simple(&simple),
            Type::Array(array) => {
                type_mapping.array(&self.mapping_type(type_mapping, array.element, &[]))
            }
            Type::ArrayM(array_m) => type_mapping.array_m(
                &self.mapping_type(type_mapping, array_m.element, &[]),
                array_m.m,
            ),

            Type::BytesM(bytes_m) => type_mapping.bytes_m(bytes_m.m),

            Type::FixedMN(fixed_m_n) => type_mapping.fixed_m_n(fixed_m_n),

            Type::IntegerM(int_m) => type_mapping.integer_m(int_m),
        }
    }
}

impl Executor {
    /// Generate one contract binding codes
    ///
    /// # Parameters
    ///
    /// - `generator` The language-specific code [`generator`](Generator) instance
    /// - `contract` [`Vec<AbiField>`],[`HardhatArtifact`](crate::json::HardhatArtifact) or other Types that implement trait [`Generate`]
    /// - `register_tuples` True if register tuple types associated with `contract` to context
    pub fn generate_one<C: Generate, G: Generator>(
        &mut self,
        generator: &mut G,
        contract_name: &str,
        contract: &C,
        register_tuples: bool,
    ) -> anyhow::Result<()> {
        if register_tuples {
            // register contract tuples
            contract.register_tuples(self);
        }

        Ok(contract.generate(self, generator, contract_name)?)
    }
}

pub struct BindingBuilder<G: Generator> {
    generator: G,
    builders: Vec<Box<dyn Fn(&mut Executor, &mut G) -> anyhow::Result<()>>>,
}

impl<G: Generator + Default> Default for BindingBuilder<G> {
    fn default() -> Self {
        Self {
            generator: Default::default(),
            builders: Default::default(),
        }
    }
}

impl<G: Generator> BindingBuilder<G> {
    /// Create new binder builder with providing [`generator`](Generator)
    pub fn new(generator: G) -> Self {
        Self {
            generator,
            builders: Default::default(),
        }
    }

    /// Generate binding codes with contract/abi data
    pub fn bind<C: AsRef<str> + 'static, CN: AsRef<str>>(
        mut self,
        contract_name: CN,
        contract: C,
    ) -> Self {
        let contract_name = contract_name.as_ref().to_string();

        self.builders.push(Box::new(move |c, g| {
            let fields: Vec<AbiField> = serde_json::from_str(contract.as_ref())?;

            c.generate_one(g, &contract_name, &fields, true)?;

            Ok(())
        }));

        self
    }

    /// Generate binding codes with hardhat artifact data
    pub fn bind_hardhat<C: AsRef<str> + 'static, CN: AsRef<str>>(
        mut self,
        contract_name: CN,
        contract: C,
    ) -> Self {
        let contract_name = contract_name.as_ref().to_string();

        self.builders.push(Box::new(move |c, g| {
            let fields: HardhatArtifact = serde_json::from_str(contract.as_ref())?;

            c.generate_one(g, &contract_name, &fields, true)?;

            Ok(())
        }));

        self
    }

    /// Generate binding codes with contract/abi file path
    pub fn bind_file<P: Into<PathBuf>, CN: AsRef<str>>(
        mut self,
        contract_name: CN,
        path: P,
    ) -> Self {
        let path: PathBuf = path.into();

        let contract_name = contract_name.as_ref().to_string();

        self.builders.push(Box::new(move |c, g| {
            let contract = read_to_string(&path)?;

            let fields: Vec<AbiField> = serde_json::from_str(contract.as_ref())?;

            c.generate_one(g, &contract_name, &fields, true)?;

            Ok(())
        }));

        self
    }

    /// Generate binding codes with hardhat artifact file path
    pub fn bind_hardhat_file<P: Into<PathBuf>, CN: AsRef<str>>(
        mut self,
        contract_name: CN,
        path: P,
    ) -> Self {
        let path: PathBuf = path.into();

        let contract_name = contract_name.as_ref().to_string();

        self.builders.push(Box::new(move |c, g| {
            let contract = read_to_string(&path)?;

            let fields: HardhatArtifact = serde_json::from_str(contract.as_ref())?;

            c.generate_one(g, &contract_name, &fields, true)?;

            Ok(())
        }));

        self
    }

    /// Retrieve [`result`](Generator) and consume binding builder instance.
    pub fn finalize(mut self) -> anyhow::Result<G> {
        let mut executor = Executor::default();

        for builder in self.builders {
            builder(&mut executor, &mut self.generator)?;
        }

        Ok(self.generator)
    }
}

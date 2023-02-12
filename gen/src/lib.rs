//! This crate provide core types of ethbind generation system
//!
//!
//!

use std::{
    collections::HashMap,
    fs::{self, read_to_string},
    path::Path,
    str::FromStr,
};

use ethbind_json::{
    AbiField, Array, ArrayM, Constructor, Error, Event, FixedMN, Function, HardhatArtifact,
    IntegerM, Type,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Typed **bind** error
#[derive(Debug, Error)]
pub enum BindError {
    /// This error is returned by [`RuntimeBinder`] to indicate
    /// that a runtime type binding for the contract type was not found.
    #[error("Runtime binder didn't found mapping runtime type for {0}")]
    UnknownType(String),
}

/// ABI data structure that can be generated into arbitrary programming language supported by `Ethbind`.
///
/// `Ethbind` provides `Generatable` implementations for types of crate [`ethbind-json`](https://docs.rs/ethbind-json)
pub trait Generatable {
    /// Generate abi data to `Target` programming language code.
    fn generate<C: Context>(&self, context: &mut C) -> anyhow::Result<()>;
}

/// `Ethbind` code generation system `Context` instance
pub trait Context {
    /// Target programming language runtime/strongly typed binder
    type Runtime: RuntimeBinder;

    /// `Target` programming language code generator
    type Language: Generator;

    /// Get context binding programming language [`Generator`] and [`runtime binder`](RuntimeBinder)
    fn get_mut(&mut self) -> (&mut Self::Language, &mut Self::Runtime);

    fn finalize(self) -> (Self::Language, Self::Runtime);
}

/// Runtime type trait for [`RuntimeBinder`]
pub trait RuntimeType {
    /// Type grammar when declare. e.g, `Vec<u8>`
    fn declare_type(&self) -> &str;
    /// Rlp encode calling syntax, parameter `variable_name` is the name of variable to encode
    fn rlp_encode(&self, variable_name: &str) -> String;
    /// Rlp decode calling syntax, parameter `inputs_variable_name` is the name of variable for `input data`
    fn rlp_decode(&self, inputs_variable_name: &str) -> String;
}

/// Binder for mapping contract type system to `target` programming language runtime type system.
pub trait RuntimeBinder {
    /// Runtime type.
    type RuntimeType: RuntimeType;

    /// Convert contract [`abi type`](Type) to [`runtime type`](RuntimeBinder::RuntimeType)
    ///
    /// If the [`runtime type`](RuntimeBinder::RuntimeType) is tuple or array of tuple returns [`None`]
    fn to_runtime_type(&mut self, r#type: &Type) -> anyhow::Result<Option<&Self::RuntimeType>>;

    /// Get runtime type by metadata `name`, If not found the implementation must return [`Err(BindError::UnknownType)`]
    fn get(&mut self, name: &str) -> anyhow::Result<&Self::RuntimeType>;
}

/// Programming language code generator supported by `Ethbind`.
///
/// The implementation must support multi-round generation process.
pub trait Generator {
    /// [`Generatable`] or `Executor` call this fn to start a new contract generation round.
    fn begin<R: RuntimeBinder>(&mut self, runtime_binder: &mut R, name: &str)
        -> anyhow::Result<()>;

    /// Close current generation contract round.
    fn end<R: RuntimeBinder>(&mut self, runtime_binder: &mut R, name: &str) -> anyhow::Result<()>;

    /// Generate contract method ,call this fn after call [`begin`](Generator::begin) at least once.
    fn generate_fn<R: RuntimeBinder>(
        &mut self,
        runtime_binder: &mut R,
        r#fn: &Function,
    ) -> anyhow::Result<()>;

    /// Generate contract deploy method ,call this fn after call [`begin`](Generator::begin) at least once.
    fn generate_deploy<R: RuntimeBinder>(
        &mut self,
        runtime_binder: &mut R,
        contructor: &Constructor,
        deploy_bytes: &str,
    ) -> anyhow::Result<()>;

    /// Generate event handle interface ,call this fn after call [`begin`](Generator::begin) at least once.
    fn generate_event<R: RuntimeBinder>(
        &mut self,
        runtime_binder: &mut R,
        event: &Event,
    ) -> anyhow::Result<()>;

    /// Generate error handle interface ,call this fn after call [`begin`](Generator::begin) at least once.
    fn generate_error<R: RuntimeBinder>(
        &mut self,
        runtime_binder: &mut R,
        error: &Error,
    ) -> anyhow::Result<()>;

    /// Close generator and return generated contract codes.
    fn finalize(self) -> Vec<Contract>;
}

/// Generated contract files archive
pub struct Contract {
    pub files: Vec<File>,
}

/// Generated codes and file name
pub struct File {
    pub name: String,
    pub data: String,
}

impl Contract {
    /// Write generated contract codes to file system.
    pub fn write_all<P: AsRef<Path>>(&self, output_dir: P) -> anyhow::Result<()> {
        if !output_dir.as_ref().exists() {
            fs::create_dir_all(&output_dir)?;
        }

        for file in &self.files {
            let path = output_dir.as_ref().join(&file.name);

            fs::write(path, &file.data)?;
        }

        Ok(())
    }
}

/// A [`RuntimeBinder`] implementation which load runtime types mapping metadata from json.
#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRuntimeBinder {
    #[serde(flatten)]
    runtime_types: HashMap<String, JsonRuntimeType>,

    #[serde(skip)]
    components: HashMap<String, JsonRuntimeType>,
}

impl FromStr for JsonRuntimeBinder {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

impl TryFrom<&str> for JsonRuntimeBinder {
    type Error = serde_json::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(value.as_ref())
    }
}

impl TryFrom<String> for JsonRuntimeBinder {
    type Error = serde_json::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        serde_json::from_str(value.as_ref())
    }
}

impl TryFrom<&[u8]> for JsonRuntimeBinder {
    type Error = serde_json::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        serde_json::from_slice(value)
    }
}

impl JsonRuntimeBinder {
    /// Create `JsonRuntimeBinder` from [`Reader`](std::io::Read)
    pub fn from_reader<R>(reader: R) -> anyhow::Result<Self>
    where
        R: std::io::Read,
    {
        Ok(serde_json::from_reader(reader)?)
    }

    /// Try load `JsonRuntimeBinder` from json file with `path` parameter
    pub fn load<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        Ok(fs::read_to_string(path)?.try_into()?)
    }

    /// Try map contract abi basic types's `type_name` to [`runtime type`](JsonRuntimeType), if not found returns `Err(EthBindError::UnknownType)`
    fn search_basic_type<T: AsRef<str>>(&self, type_name: T) -> anyhow::Result<&JsonRuntimeType> {
        self.runtime_types
            .get(type_name.as_ref())
            .ok_or(BindError::UnknownType(type_name.as_ref().to_owned()).into())
    }

    fn to_array_m(&mut self, element: &ArrayM) -> anyhow::Result<Option<&JsonRuntimeType>> {
        let tag = element.to_string();

        // Returns exists runtime type
        if self.components.contains_key(&tag) {
            return Ok(self.components.get(&tag));
        }

        let el_type = { self.to_runtime_type(&element.element)?.map(|c| c.clone()) };

        if el_type.is_none() {
            return Ok(None);
        }

        let el_type = el_type.unwrap();

        let runtime_type = self.search_basic_type("array_m")?;

        let m = element.m;

        let declare_type = runtime_type
            .declare_type()
            .replace("$el", el_type.declare_type())
            .replace("$m", &m.to_string());

        let rlp_encode = runtime_type
            .rlp_encode
            .replace("$el", &el_type.rlp_encode)
            .replace("$m", &m.to_string());

        let rlp_decode = runtime_type
            .rlp_decode
            .replace("$el", &el_type.declare_type)
            .replace("$m", &m.to_string());

        let rtype = JsonRuntimeType {
            declare_type,
            rlp_decode,
            rlp_encode,
        };

        self.components.insert(tag.clone(), rtype);

        Ok(self.components.get(&tag))
    }

    fn to_bytes_m(&mut self, m: usize) -> anyhow::Result<&JsonRuntimeType> {
        let tag = format!("array{}", m);

        // Returns exists runtime type
        if self.components.contains_key(&tag) {
            return Ok(self.components.get(&tag).unwrap());
        }

        let runtime_type = self.search_basic_type("bytes_m")?;

        let declare_type = runtime_type.declare_type().replace("$m", &m.to_string());

        let rlp_encode = runtime_type.rlp_encode.replace("$m", &m.to_string());

        let rlp_decode = runtime_type.rlp_decode.replace("$m", &m.to_string());

        let rtype = JsonRuntimeType {
            declare_type,
            rlp_decode,
            rlp_encode,
        };

        self.components.insert(tag.clone(), rtype);

        Ok(self.components.get(&tag).unwrap())
    }

    fn to_array(&mut self, element: &Array) -> anyhow::Result<Option<&JsonRuntimeType>> {
        let tag = element.to_string();

        // Returns exists runtime type
        if self.components.contains_key(&tag) {
            return Ok(self.components.get(&tag));
        }

        let el_type = { self.to_runtime_type(&element.element)?.map(|c| c.clone()) };

        if el_type.is_none() {
            return Ok(None);
        }

        let el_type = el_type.unwrap().clone();

        let runtime_type = self.search_basic_type("array")?;

        let declare_type = runtime_type
            .declare_type()
            .replace("$el", el_type.declare_type());

        let rlp_encode = runtime_type.rlp_encode.replace("$el", &el_type.rlp_encode);

        let rlp_decode = runtime_type
            .rlp_decode
            .replace("$el", &el_type.declare_type);

        let rtype = JsonRuntimeType {
            declare_type,
            rlp_decode,
            rlp_encode,
        };

        self.components.insert(tag.clone(), rtype);

        Ok(self.components.get(&tag))
    }

    fn to_fixed_m_n(&mut self, fixed: &FixedMN) -> anyhow::Result<&JsonRuntimeType> {
        let tag = fixed.to_string();

        // Returns exists runtime type
        if self.components.contains_key(&tag) {
            return Ok(self.components.get(&tag).unwrap());
        }

        let runtime_type = self.search_basic_type("fixed_m_n")?;

        let declare_type = runtime_type
            .declare_type()
            .replace("$m", &fixed.m.to_string())
            .replace("$n", &fixed.n.to_string());

        let rlp_encode = runtime_type
            .rlp_encode
            .replace("$m", &fixed.m.to_string())
            .replace("$n", &fixed.n.to_string());

        let rlp_decode = runtime_type
            .rlp_decode
            .replace("$m", &fixed.m.to_string())
            .replace("$n", &fixed.n.to_string());

        let rtype = JsonRuntimeType {
            declare_type,
            rlp_decode,
            rlp_encode,
        };

        self.components.insert(tag.clone(), rtype);

        Ok(self.components.get(&tag).unwrap())
    }

    fn to_integer_m(&mut self, integer_m: &IntegerM) -> anyhow::Result<&JsonRuntimeType> {
        let tag = integer_m.to_string();

        // Returns exists runtime type
        if self.components.contains_key(&tag) {
            return Ok(self.components.get(&tag).unwrap());
        }

        let runtime_type = if integer_m.signed {
            self.search_basic_type("int_m")?
        } else {
            self.search_basic_type("uint_m")?
        };

        let declare_type = runtime_type
            .declare_type()
            .replace("$m", &integer_m.m.to_string());

        let rlp_encode = runtime_type
            .rlp_encode
            .replace("$m", &integer_m.m.to_string());

        let rlp_decode = runtime_type
            .rlp_decode
            .replace("$m", &integer_m.m.to_string());

        let rtype = JsonRuntimeType {
            declare_type,
            rlp_decode,
            rlp_encode,
        };

        self.components.insert(tag.clone(), rtype);

        Ok(self.components.get(&tag).unwrap())
    }
}

impl RuntimeBinder for JsonRuntimeBinder {
    type RuntimeType = JsonRuntimeType;
    fn to_runtime_type(&mut self, r#type: &Type) -> anyhow::Result<Option<&Self::RuntimeType>> {
        match r#type {
            Type::Simple(element) if element.is_tuple() => Ok(None),
            Type::Simple(element) => self.search_basic_type(element.to_string()).map(|c| Some(c)),
            Type::ArrayM(element) => self.to_array_m(&element),
            Type::Array(element) => self.to_array(&element),
            Type::BytesM(element) => self.to_bytes_m(element.m).map(|c| Some(c)),
            Type::FixedMN(element) => self.to_fixed_m_n(element).map(|c| Some(c)),
            Type::IntegerM(element) => self.to_integer_m(element).map(|c| Some(c)),
        }
    }

    fn get(&mut self, name: &str) -> anyhow::Result<&Self::RuntimeType> {
        Ok(self
            .runtime_types
            .get(name)
            .ok_or(BindError::UnknownType(name.to_string()))?)
    }
}

/// Associated [`RuntimeType`] for [`JsonRuntimeBinder`]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JsonRuntimeType {
    declare_type: String,
    rlp_encode: String,
    rlp_decode: String,
}

impl RuntimeType for JsonRuntimeType {
    fn declare_type(&self) -> &str {
        &self.declare_type
    }

    fn rlp_decode(&self, inputs_variable_name: &str) -> String {
        self.rlp_decode.replace("$inputs", inputs_variable_name)
    }

    fn rlp_encode(&self, variable_name: &str) -> String {
        self.rlp_encode.replace("$var", variable_name)
    }
}

/// `Ethbind` executor
pub struct Executor<L, R> {
    generator: L,
    runtime_binder: R,
}

impl<L, R> Context for Executor<L, R>
where
    R: RuntimeBinder,
    L: Generator,
{
    type Language = L;
    type Runtime = R;

    fn get_mut(&mut self) -> (&mut Self::Language, &mut Self::Runtime) {
        (&mut self.generator, &mut self.runtime_binder)
    }

    fn finalize(self) -> (Self::Language, Self::Runtime) {
        (self.generator, self.runtime_binder)
    }
}

pub struct BindingBuilder<C: Context> {
    context: C,
    builders: Vec<Box<dyn Fn(&mut C) -> anyhow::Result<()>>>,
}

impl<C: Context + Default> Default for BindingBuilder<C> {
    fn default() -> Self {
        Self {
            context: Default::default(),
            builders: Default::default(),
        }
    }
}

impl<C: Context> BindingBuilder<C> {
    /// Create new binder builder with providing [`generator`](Generator)
    pub fn new(context: C) -> Self {
        Self {
            context,
            builders: Default::default(),
        }
    }

    /// Generate binding codes with contract/abi data
    pub fn bind<S: AsRef<str> + 'static, CN: AsRef<str>>(
        mut self,
        contract_name: CN,
        contract: S,
    ) -> Self {
        let contract_name = contract_name.as_ref().to_string();

        self.builders.push(Box::new(move |c| {
            let fields: Vec<AbiField> = serde_json::from_str(contract.as_ref())?;

            {
                let (generator, runtime_binder) = c.get_mut();

                generator.begin(runtime_binder, &contract_name)?;
            }

            fields.generate(c)?;

            Ok(())
        }));

        self
    }

    /// Generate binding codes with hardhat artifact data
    pub fn bind_hardhat<S: AsRef<str> + 'static>(mut self, contract: S) -> Self {
        self.builders.push(Box::new(move |c| {
            let fields: HardhatArtifact = serde_json::from_str(contract.as_ref())?;

            {
                let (generator, runtime_binder) = c.get_mut();

                generator.begin(runtime_binder, &fields.contract_name)?;
            }

            fields.generate(c)?;

            Ok(())
        }));

        self
    }

    /// Generate binding codes with contract/abi file path
    pub fn bind_file<P: AsRef<Path> + 'static, CN: AsRef<str>>(
        mut self,
        contract_name: CN,
        path: P,
    ) -> Self {
        let contract_name = contract_name.as_ref().to_string();

        self.builders.push(Box::new(move |c| {
            let contract = read_to_string(&path)?;

            let fields: Vec<AbiField> = serde_json::from_str(contract.as_ref())?;

            {
                let (generator, runtime_binder) = c.get_mut();

                generator.begin(runtime_binder, &contract_name)?;
            }

            fields.generate(c)?;

            Ok(())
        }));

        self
    }

    /// Generate binding codes with hardhat artifact file path
    pub fn bind_hardhat_file<P: AsRef<Path> + 'static>(mut self, path: P) -> Self {
        self.builders.push(Box::new(move |c| {
            let contract = read_to_string(&path)?;

            let fields: HardhatArtifact = serde_json::from_str(contract.as_ref())?;

            {
                let (generator, runtime_binder) = c.get_mut();

                generator.begin(runtime_binder, &fields.contract_name)?;
            }

            fields.generate(c)?;

            Ok(())
        }));

        self
    }

    /// Retrieve [`result`](Generator) and consume binding builder instance.
    pub fn finalize(mut self) -> anyhow::Result<Vec<Contract>> {
        for builder in self.builders {
            builder(&mut self.context)?;
        }

        let (generator, _) = self.context.finalize();

        Ok(generator.finalize())
    }
}

impl Generatable for HardhatArtifact {
    fn generate<C: Context>(&self, context: &mut C) -> anyhow::Result<()> {
        self.abi.generate(context)?;

        for abi in &self.abi {
            match abi {
                AbiField::Constructor(contructor) => {
                    // Generate deploy fn
                    let (generator, runtime_binder) = context.get_mut();

                    generator.generate_deploy(runtime_binder, contructor, &self.bytecode)?;
                }
                _ => {}
            }
        }

        Ok(())
    }
}

impl Generatable for Vec<AbiField> {
    fn generate<C: Context>(&self, context: &mut C) -> anyhow::Result<()> {
        let (generator, runtime_binder) = context.get_mut();

        for abi in self {
            match abi {
                AbiField::Function(function) => generator.generate_fn(runtime_binder, function)?,
                AbiField::Event(event) => generator.generate_event(runtime_binder, event)?,
                AbiField::Error(error) => generator.generate_error(runtime_binder, error)?,
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

#[cfg(test)]
mod tests {
    use ethbind_json::Type;

    use crate::{JsonRuntimeBinder, RuntimeBinder};

    #[test]
    fn test_json_runtime_binder() {
        let mut runtime_binder: JsonRuntimeBinder =
            include_str!("./abi.json").parse().expect("Load abi");

        let t: Type = "uint256".parse().expect("Parse type string");

        let runtime_type = runtime_binder
            .to_runtime_type(&t)
            .expect("Get runtime type")
            .expect("Is not tuple type");

        assert_eq!(runtime_type.declare_type, "int<false,256>");
        assert_eq!(runtime_type.rlp_encode, "$var.encode()");
        assert_eq!(runtime_type.rlp_decode, "int::<false,256>::decode($inputs)");

        let t: Type = "fixed128x8".parse().expect("Parse type string");

        let runtime_type = runtime_binder
            .to_runtime_type(&t)
            .expect("Get runtime type")
            .expect("Is not tuple type");

        assert_eq!(runtime_type.declare_type, "fixed<true,128,8>");
        assert_eq!(runtime_type.rlp_encode, "$var.encode()");
        assert_eq!(
            runtime_type.rlp_decode,
            "fixed::<true,128,8>::decode($inputs)"
        );

        let t: Type = "uint256[20]".parse().expect("Parse type string");

        let runtime_type = runtime_binder
            .to_runtime_type(&t)
            .expect("Get runtime type")
            .expect("Is not tuple type");

        assert_eq!(runtime_type.declare_type, "[int<false,256>;20]");
        assert_eq!(runtime_type.rlp_encode, "$var.encode()");
        assert_eq!(
            runtime_type.rlp_decode,
            "<[int<false,256>;20]>::decode($inputs)"
        );

        let t: Type = "uint256[]".parse().expect("Parse type string");

        let runtime_type = runtime_binder
            .to_runtime_type(&t)
            .expect("Get runtime type")
            .expect("Is not tuple type");

        assert_eq!(runtime_type.declare_type, "Vec<int<false,256>>");
        assert_eq!(runtime_type.rlp_encode, "$var.encode()");
        assert_eq!(
            runtime_type.rlp_decode,
            "Vec<int<false,256>>::decode($inputs)"
        );

        let t: Type = "bytes24".parse().expect("Parse type string");

        let runtime_type = runtime_binder
            .to_runtime_type(&t)
            .expect("Get runtime type")
            .expect("Is not tuple type");

        assert_eq!(runtime_type.declare_type, "[u8;24]");
        assert_eq!(runtime_type.rlp_encode, "$var.encode()");
        assert_eq!(runtime_type.rlp_decode, "<[u8;24]>::decode($inputs)");

        let t: Type = "address".parse().expect("Parse type string");

        let runtime_type = runtime_binder
            .to_runtime_type(&t)
            .expect("Get runtime type")
            .expect("Is not tuple type");

        assert_eq!(runtime_type.declare_type, "Address");
        assert_eq!(runtime_type.rlp_encode, "$var.encode()");
        assert_eq!(runtime_type.rlp_decode, "Address::decode($inputs)");
    }
}

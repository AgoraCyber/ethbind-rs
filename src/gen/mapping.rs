use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::error::TypeMappingError;

use super::TypeMapping;

/// The basic [`TypeMapping`](super::TypeMapping) implementation with [`serde`] Serialize/Deserialize supporting.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(unused)]
pub struct SerdeTypeMapping {
    /// Mapping contract tuple name to generating language tuple name.
    #[serde(skip)]
    pub tuple_names: HashMap<String, String>,
    /// Mapping contract basic types to generating target language runtime types.
    #[serde(flatten)]
    types_mapping: HashMap<String, String>,
}

impl SerdeTypeMapping {
    /// Get mapping rust types string by parameter `name`
    pub fn get_mapping(&self, name: &str, args: &[(&str, &str)]) -> anyhow::Result<String> {
        let mut fmt_str = self
            .types_mapping
            .get(name)
            .ok_or(TypeMappingError::NotFound(name.to_owned()))
            .unwrap()
            .clone();

        for (place_holder, value) in args {
            fmt_str = fmt_str.replace(place_holder, value);
        }

        if fmt_str.find("$").is_some() {
            let wildcards = args
                .iter()
                .map(|(c, _)| c.to_string())
                .collect::<Vec<_>>()
                .join(" ");
            return Err(TypeMappingError::Serde(fmt_str, wildcards).into());
        }

        Ok(fmt_str)
    }
}

#[allow(unused)]
impl TypeMapping for SerdeTypeMapping {
    fn array(&self, element: &str) -> String {
        self.get_mapping("array", &[("$element", element)]).unwrap()
    }

    fn array_m(&self, element: &str, m: usize) -> String {
        self.get_mapping("array_m", &[("$element", element), ("$m", &m.to_string())])
            .unwrap()
    }

    fn bytes_m(&self, m: usize) -> String {
        self.get_mapping("bytes_m", &[("$m", &m.to_string())])
            .unwrap()
    }

    fn fixed_m_n(&self, t: crate::json::FixedMN) -> String {
        if t.signed {
            self.get_mapping(
                "fixed_m_n",
                &[("$m", &t.m.to_string()), ("$n", &t.n.to_string())],
            )
            .unwrap()
        } else {
            self.get_mapping(
                "ufixed_m_n",
                &[("$m", &t.m.to_string()), ("$n", &t.n.to_string())],
            )
            .unwrap()
        }
    }

    fn integer_m(&self, t: crate::json::IntegerM) -> String {
        if t.signed {
            self.get_mapping("int_m", &[("$m", &t.m.to_string())])
                .unwrap()
        } else {
            self.get_mapping("uint_m", &[("$m", &t.m.to_string())])
                .unwrap()
        }
    }

    fn simple(&self, t: &crate::json::SimpleType) -> String {
        self.get_mapping(&t.to_string(), &[]).unwrap()
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        gen::TypeMapping,
        json::{FixedMN, IntegerM, SimpleType},
    };

    use super::SerdeTypeMapping;

    #[test]
    fn test_load_mapping_from_json() {
        let mapping: SerdeTypeMapping =
            serde_json::from_str(include_str!("../../data/mapping.json"))
                .expect("Loading mapping from json file");

        assert_eq!(mapping.array("uint256"), "Vec<uint256>");

        assert_eq!(mapping.array_m("uint256", 64), "[uint256;64]");

        assert_eq!(mapping.bytes_m(32), "[u8;32]");

        assert_eq!(
            mapping.fixed_m_n(FixedMN {
                signed: true,
                m: 256,
                n: 30
            }),
            "Fixed<true,256,30>"
        );

        assert_eq!(
            mapping.fixed_m_n(FixedMN {
                signed: false,
                m: 256,
                n: 30
            }),
            "Fixed<false,256,30>"
        );

        assert_eq!(
            mapping.integer_m(IntegerM {
                signed: true,
                m: 256,
            }),
            "Int<true,256>"
        );

        assert_eq!(
            mapping.integer_m(IntegerM {
                signed: false,
                m: 128,
            }),
            "Int<false,128>"
        );

        assert_eq!(mapping.simple(&SimpleType::Address), "ethers_rs::Address");
    }
}

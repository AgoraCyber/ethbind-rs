//! Ethereum constract abi json format encode/decode support
//!
//! Visit [`official document`](https://docs.soliditylang.org/en/v0.8.17/abi-spec.html#json) for details

use serde::{Deserialize, Serialize};

/// The root type structures for solidity contract abi json format
#[derive(Debug, Serialize, Deserialize)]
pub struct Constract {}

/// Contract interface type enum
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum FieldType {
    Function(Function),
    Constructor,
    Receive,
    Fallback,
    Event,
    Error,
}

/// A structure type to represent `function` abi
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Function {
    /// the function name
    pub name: String,
    /// An array of function's input params
    pub inputs: Vec<Parameter>,
    /// An array of function's output params
    pub outputs: Vec<Parameter>,
}

/// A structure type to represent `event` abi
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    /// the function name
    pub name: String,
    /// An array of function's input params
    pub inputs: Vec<Parameter>,
    /// `true` if the event was declared as anonymous
    pub anonymous: bool,
}
/// Handle Function/Event/Error 's input or output parameter type
#[derive(Debug, Serialize, Deserialize)]
pub struct Parameter {
    /// The name of the parameter
    pub name: String,
    /// The canonical type of the parameter
    pub r#type: String,
    /// used for tuple types, only if the type field start with prefix `tuple`. e.g, `tupe[]`,`tuple`
    pub components: Option<Vec<Parameter>>,
}

#[derive(Debug)]
pub struct Integer {
    pub signed: bool,
    pub len: usize,
}

/// Contract abi simple types enum
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SimpleType {
    Address,
    Uint,
    Int,
    Bool,
    Fixed,
    Ufixed,
    /// an address (20 bytes) followed by a function selector (4 bytes). Encoded identical to bytes24.
    Function,
    Bytes,
    String,
}

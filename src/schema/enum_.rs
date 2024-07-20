use serde::{Deserialize, Serialize};

use crate::EnumValue;

/// The type of the integer enum value.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ValueType {
    INT8,
    UINT8,
    INT16,
    UINT16,
    INT32,
    UINT32,
    INT64,
    UINT64,
    OTHER(String),
}

/// An object defining the values of an enum.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Enum {
    /// The name of the enum, e.g. for display purposes.
    pub name: Option<String>,
    /// The description of the enum.
    pub description: Option<String>,
    /// The type of the integer enum value.
    pub value_type: Option<ValueType>,
    /// An array of enum values. Duplicate names or duplicate integer values are not allowed.
    pub values: Vec<EnumValue>,
}

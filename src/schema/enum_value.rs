use serde::{Deserialize, Serialize};

/// An enum value.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EnumValue {
    /// The name of the enum value.
    pub name: String,
    /// The description of the enum value.
    pub description: Option<String>,
    /// The integer enum value.
    pub value: i32,
}

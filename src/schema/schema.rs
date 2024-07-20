use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{Class, Enum};

/// An object defining classes and enums.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Schema {
    /// Unique identifier for the schema. Schema IDs shall be alphanumeric identifiers matching the regular expression `^[a-zA-Z_][a-zA-Z0-9_]*$`.
    pub id: String,
    /// The name of the schema, e.g. for display purposes.
    pub name: Option<String>,
    /// The description of the schema.
    pub description: Option<String>,
    /// "Application-specific version of the schema."
    pub version: Option<String>,
    /// A dictionary, where each key is a class ID and each value is an object defining the class. Class IDs shall be alphanumeric identifiers matching the regular expression `^[a-zA-Z_][a-zA-Z0-9_]*$`.
    pub classes: Option<HashMap<String, Class>>,
    /// A dictionary, where each key is an enum ID and each value is an object defining the values for the enum. Enum IDs shall be alphanumeric identifiers matching the regular expression `^[a-zA-Z_][a-zA-Z0-9_]*$`.
    pub enums: Option<HashMap<String, Enum>>,
}

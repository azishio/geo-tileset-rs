use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::ClassProperty;

/// A class containing a set of properties.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Class {
    /// The name of the class, e.g. for display purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The description of the class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// A dictionary, where each key is a property ID and each value is an object defining the property. Property IDs shall be alphanumeric identifiers matching the regular expression `^[a-zA-Z_][a-zA-Z0-9_]*$`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, ClassProperty>>,
}

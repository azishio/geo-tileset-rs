use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::PropertyTableProperty;

/// Properties conforming to a class, organized as property values stored in binary columnar arrays.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PropertyTable {
    /// The name of the property table, e.g. for display purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The class that property values conform to. The value shall be a class ID declared in the `classes` dictionary.
    pub class: String,
    /// The number of elements in each property array.
    pub count: i32,
    /// A dictionary, where each key corresponds to a property ID in the class' `properties` dictionary and each value is an object describing where property values are stored. Required properties shall be included in this dictionary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, PropertyTableProperty>>,
}

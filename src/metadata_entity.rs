use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::AnyValue;

/// An object containing a reference to a class from a metadata schema, and property values that conform to the properties of that class.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MetadataEntity {
    /// The class that property values conform to. The value shall be a class ID declared in the `classes` dictionary of the metadata schema.
    pub class: String,

    /// A dictionary, where each key corresponds to a property ID in the class' `properties` dictionary and each value contains the property values. The type of the value shall match the property definition: For `BOOLEAN` use `true` or `false`. For `STRING` use a JSON string. For numeric types use a JSON number. For `ENUM` use a valid enum `name`, not an integer value. For `ARRAY`, `VECN`, and `MATN` types use a JSON array containing values matching the `componentType`. Required properties shall be included in this dictionary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, AnyValue>>,
}

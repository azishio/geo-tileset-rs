use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::StatisticsClassProperty;

/// Statistics about entities that conform to a class that was defined in a metadata schema.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StatisticsClass {
    /// The number of entities that conform to the class.
    pub count: Option<i32>,
    /// A dictionary, where each key corresponds to a property ID in the class' `properties` dictionary and each value is an object containing statistics about property values.
    pub properties: Option<HashMap<String, StatisticsClassProperty>>,
}

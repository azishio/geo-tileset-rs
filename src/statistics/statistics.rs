use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::StatisticsClass;

/// Statistics about entities.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Statistics {
    /// A dictionary, where each key corresponds to a class ID in the `classes` dictionary of the metatata schema that was defined for the tileset that contains these statistics. Each value is an object containing statistics about entities that conform to the class.
    pub classes: Option<HashMap<String, StatisticsClass>>,
}

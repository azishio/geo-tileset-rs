use serde::{Deserialize, Serialize};

/// A dictionary object of metadata about per-feature properties.
#[deprecated]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Properties {
    /// The maximum value of this property of all the features in the tileset. The maximum value shall not be smaller than the minimum value.
    pub maximum: f64,
    /// The minimum value of this property of all the features in the tileset. The maximum value shall not be smaller than the minimum value.
    pub minimum: f64,
}

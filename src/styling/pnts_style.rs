use serde::{Deserialize, Serialize};

use crate::{StyleConditions, StyleNumberExpression};

/// A `number expression` or `conditions` property which determines the size of the points in pixels.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PointSize {
    NumberExpression(StyleNumberExpression),
    Conditions(StyleConditions),
}

impl Default for PointSize {
    fn default() -> Self {
        PointSize::NumberExpression(StyleNumberExpression::Number(1.0))
    }
}

/// A 3D Tiles style with additional properties for Point Clouds.
#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PntsStyle {
    /// A `number expression` or `conditions` property which determines the size of the points in pixels.
    pub point_size: PointSize,
}

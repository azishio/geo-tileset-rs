use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{StyleBooleanExpression, StyleColorExpression, StyleConditions, StyleExpression, StyleMeta};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Show {
    Boolean(StyleBooleanExpression),
    Conditions(StyleConditions),
}

impl Default for Show {
    fn default() -> Self {
        Show::Boolean(StyleBooleanExpression::Boolean(true))
    }
}

/// A `color expression` or `conditions` property which determines the color blended with the feature's intrinsic color.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Color {
    Color(StyleColorExpression),
    Conditions(StyleConditions),
}

impl Default for Color {
    fn default() -> Self {
        Color::Color("color('#FFFFFF')".to_string())
    }
}

/// A 3D Tiles style.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Style {
    /// A dictionary object of `expression` strings mapped to a variable name key that may be referenced throughout the style. If an expression references a defined variable, it is replaced with the evaluated result of the corresponding expression.
    pub defines: Option<HashMap<String, StyleExpression>>,
    /// A `boolean expression` or `conditions` property which determines if a feature should be shown.
    pub show: Option<Show>,
    /// A `color expression` or `conditions` property which determines the color blended with the feature's intrinsic color.
    pub color: Option<Color>,
    /// A `meta` object which determines the values of non-visual properties of the feature.
    pub meta: Option<StyleMeta>,
}

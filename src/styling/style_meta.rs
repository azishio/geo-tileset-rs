use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{RootProperty, StyleExpression};

pub enum MetaProperty {}

/// A series of property names and the `expression` to evaluate for the value of that property.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StyleMeta {
    #[serde(flatten)]
    pub root: RootProperty,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional: Option<HashMap<String, StyleExpression>>,
}


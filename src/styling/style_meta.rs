use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{RootProperty, StyleExpression};

pub enum MetaProperty {}

/// A series of property names and the `expression` to evaluate for the value of that property.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StyleMeta {
    pub root: RootProperty,
    pub additional: Option<HashMap<String, StyleExpression>>,
}


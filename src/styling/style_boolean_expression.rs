use serde::{Deserialize, Serialize};

/// A boolean or string with a 3D Tiles style expression that evaluates to a boolean. Details are described in the 3D Tiles Styling specification.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum StyleBooleanExpression {
    Boolean(bool),
    String(String),
}

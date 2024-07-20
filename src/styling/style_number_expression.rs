use serde::{Deserialize, Serialize};


/// 3D Tiles style expression that evaluates to a number. Details are described in the 3D Tiles Styling specification.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum StyleNumberExpression {
    Number(f64),
    String(String),
}

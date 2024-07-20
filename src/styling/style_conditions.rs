use serde::{Deserialize, Serialize};

use crate::StyleConditionsCondition;


/// A series of conditions evaluated in order, like a series of if...else statements that result in an expression being evaluated.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StyleConditions {
    /// A series of boolean conditions evaluated in order. For the first one that evaluates to true, its value, the 'result' (which is also an expression), is evaluated and returned. Result expressions shall all be the same type. If no condition evaluates to true, the result is `undefined`. When conditions is `undefined`, `null`, or an empty object, the result is `undefined`.
    pub conditions: Vec<StyleConditionsCondition>,
}

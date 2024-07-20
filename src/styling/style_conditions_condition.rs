use crate::StyleExpression;

/// An `expression` evaluated as the result of a condition being true. An array of two expressions. If the first expression is evaluated and the result is `true`, then the second expression is evaluated and returned as the result of the condition.
pub type StyleConditionsCondition = [StyleExpression; 2];


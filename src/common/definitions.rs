use serde::{Deserialize, Serialize};


/// For `SCALAR` this is a number. For `VECN` this is an array of `N` numbers. For `MATN` this is an array of `N²` numbers. For fixed-length arrays this is an array of `count` elements of the given `type`.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum NumericValue {
    Number(f64),

    /// An array of numeric values
    NumericArray1D(Vec<f64>),

    /// An array of arrays of numeric values
    NumericArray2D(Vec<Vec<f64>>),
}


/// For `SCALAR` this is a number. For `STRING` this is a string. For `ENUM` this is a string that shall be a valid enum `name`, not an integer value. For `VECN` this is an array of `N` numbers. For `MATN` this is an array of `N²` numbers. For fixed-length arrays this is an array of `count` elements of the given `type`.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum NoDataValue {
    /// For `SCALAR` this is a number. For `VECN` this is an array of `N` numbers. For `MATN` this is an array of `N²` numbers. For fixed-length arrays this is an array of `count` elements of the given `type`.
    NumericValue(NumericValue),

    String(String),

    /// An array of string values
    StringArray1D(Vec<String>),
}


/// For `SCALAR` this is a number. For `STRING` this is a string. For `ENUM` this is a string that shall be a valid enum `name`, not an integer value. For `BOOLEAN` this is a boolean. For `VECN` this is an array of `N` numbers. For `MATN` this is an array of `N²` numbers. For fixed-length array this is an array of `count` elements of the given `type`. For variable-length arrays this is an array of any length of the given `type`.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum AnyValue {
    /// For `SCALAR` this is a number. For `VECN` this is an array of `N` numbers. For `MATN` this is an array of `N²` numbers. For fixed-length arrays this is an array of `count` elements of the given `type`.
    NumericValue(NumericValue),

    String(String),

    /// An array of string values
    StringArray1D(Vec<String>),

    Boolean(bool),

    /// An array of boolean values
    BooleanArray1D(Vec<bool>),
}

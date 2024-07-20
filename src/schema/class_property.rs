use serde::{Deserialize, Serialize};

use crate::{AnyValue, NoDataValue, NumericValue};

/// The element type.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Type {
    SCALAR,
    VEC2,
    VEC3,
    VEC4,
    MAT2,
    MAT3,
    MAT4,
    STRING,
    BOOLEAN,
    ENUM,
    #[serde(untagged)]
    OTHER(String),
}


/// The datatype of the element's components. Required for `SCALAR`, `VECN`, and `MATN` types, and disallowed for other types.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ComponentType {
    INT8,
    UINT8,
    INT16,
    UINT16,
    INT32,
    UINT32,
    INT64,
    UINT64,
    FLOAT32,
    FLOAT64,
    #[serde(untagged)]
    OTHER(String),
}


/// A single property of a metadata class.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ClassProperty {
    /// The name of the property, e.g. for display purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The description of the property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The element type.
    #[serde(rename = "type")]
    pub type_: Type,

    /// The datatype of the element's components. Required for `SCALAR`, `VECN`, and `MATN` types, and disallowed for other types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_type: Option<ComponentType>,

    /// Enum ID as declared in the `enums` dictionary. Required when `type` is `ENUM`. Disallowed when `type` is not `ENUM`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enum_type: Option<String>,

    /// Whether the property is an array. When `count` is defined the property is a fixed-length array. Otherwise the property is a variable-length array.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub array: Option<bool>,

    /// The number of array elements. May only be defined when `array` is `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,

    /// Specifies whether integer values are normalized. Only applicable to `SCALAR`, `VECN`, and `MATN` types with integer component types. For unsigned integer component types, values are normalized between `[0.0, 1.0]`. For signed integer component types, values are normalized between `[-1.0, 1.0]`. For all other component types, this property shall be false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normalized: Option<bool>,

    /// An offset to apply to property values. Only applicable to `SCALAR`, `VECN`, and `MATN` types when the component type is `FLOAT32` or `FLOAT64`, or when the property is `normalized`. Not applicable to variable-length arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<NumericValue>,

    /// A scale to apply to property values. Only applicable to `SCALAR`, `VECN`, and `MATN` types when the component type is `FLOAT32` or `FLOAT64`, or when the property is `normalized`. Not applicable to variable-length arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<NumericValue>,

    /// Maximum allowed value for the property. Only applicable to `SCALAR`, `VECN`, and `MATN` types. This is the maximum of all property values, after the transforms based on the `normalized`, `offset`, and `scale` properties have been applied. Not applicable to variable-length arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<NumericValue>,

    /// Minimum allowed value for the property. Only applicable to `SCALAR`, `VECN`, and `MATN` types. This is the minimum of all property values, after the transforms based on the `normalized`, `offset`, and `scale` properties have been applied. Not applicable to variable-length arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<NumericValue>,

    /// If required, the property shall be present in every entity conforming to the class. If not required, individual entities may include `noData` values, or the entire property may be omitted. As a result, `noData` has no effect on a required property. Client implementations may use required properties to make performance optimizations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,

    /// A `noData` value represents missing data — also known as a sentinel value — wherever it appears. `BOOLEAN` properties may not specify `noData` values. This is given as the plain property value, without the transforms from the `normalized`, `offset`, and `scale` properties. Shall not be defined if `required` is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_data: Option<NoDataValue>,

    /// A default value to use when encountering a `noData` value or an omitted property. The value is given in its final form, taking the effect of `normalized`, `offset`, and `scale` properties into account. Shall not be defined if `required` is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<AnyValue>,

    /// An identifier that describes how this property should be interpreted. The semantic cannot be used by other properties in the class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic: Option<String>,
}

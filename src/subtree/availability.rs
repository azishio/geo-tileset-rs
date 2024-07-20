use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[repr(i32)]
pub enum Constant {
    UNAVAILABLE = 0,
    AVAILABLE = 1,
    Other(i32),
}


/// An object describing the availability of a set of elements.
#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Availability {
    /// Index of a buffer view that indicates whether each element is available. The bitstream conforms to the boolean array encoding described in the 3D Metadata specification. If an element is available, its bit is 1, and if it is unavailable, its bit is 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitstream: Option<i32>,

    /// A number indicating how many 1 bits exist in the availability bitstream.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_count: Option<i32>,

    /// Integer indicating whether all of the elements are available (1) or all are unavailable (0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant: Option<Constant>,
}

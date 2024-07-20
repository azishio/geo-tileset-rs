use serde::{Deserialize, Serialize};

/// A buffer is a binary blob. It is either the binary chunk of the subtree file, or an external buffer referenced by a URI.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Buffer {
    /// The URI (or IRI) of the file that contains the binary buffer data. Relative paths are relative to the file containing the buffer JSON. `uri` is required when using the JSON subtree format and not required when using the binary subtree format - when omitted the buffer refers to the binary chunk of the subtree file. Data URIs are not allowed.
    pub uri: Option<String>,
    /// The length of the buffer in bytes.
    pub byte_length: i32,
    /// The name of the buffer.
    pub name: Option<String>,
}

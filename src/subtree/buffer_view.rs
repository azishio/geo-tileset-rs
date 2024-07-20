use serde::{Deserialize, Serialize};

/// A contiguous subset of a buffer
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BufferView {
    /// The index of the buffer.
    pub buffer: i32,
    /// The offset into the buffer in bytes.
    pub byte_offset: i32,
    /// The total byte length of the buffer view.
    pub byte_length: i32,
    /// The name of the `bufferView`.
    pub name: Option<String>,
}

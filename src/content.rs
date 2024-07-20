use serde::{Deserialize, Serialize};

use crate::{BoundingVolume, MetadataEntity};

/// Metadata about the tile's content and a link to the content.
#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Content {
    /// An optional bounding volume that tightly encloses tile content. tile.boundingVolume provides spatial coherence and tile.content.boundingVolume enables tight view frustum culling. When this is omitted, tile.boundingVolume is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounding_volume: Option<BoundingVolume>,
    /// A uri that points to tile content. When the uri is relative, it is relative to the referring tileset JSON file.
    pub uri: String,
    /// Metadata that is associated with this content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<MetadataEntity>,
    /// "The group this content belongs to. The value is an index into the array of `groups` that is defined for the containing tileset."
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<i32>,
}

use serde::{Deserialize, Serialize};

use crate::{Availability, Buffer, BufferView, MetadataEntity, PropertyTable};

/// An object describing the availability of tiles and content in a subtree, as well as availability of children subtrees. May also store metadata for available tiles and content.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Subtree {
    /// An array of buffers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffers: Option<Vec<Buffer>>,

    /// An array of buffer views.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffer_views: Option<Vec<BufferView>>,

    /// An array of property tables.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_tables: Option<Vec<PropertyTable>>,

    /// The availability of tiles in the subtree. The availability bitstream is a 1D boolean array where tiles are ordered by their level in the subtree and Morton index within that level. A tile's availability is determined by a single bit, 1 meaning a tile exists at that spatial index, and 0 meaning it does not. The number of elements in the array is `(N^subtreeLevels - 1)/(N - 1)` where N is 4 for subdivision scheme `QUADTREE` and 8 for `OCTREE`. Availability may be stored in a buffer view or as a constant value that applies to all tiles. If a non-root tile's availability is 1 its parent tile's availability shall also be 1. `tileAvailability.constant: 0` is disallowed, as subtrees shall have at least one tile.
    pub tile_availability: Availability,

    /// An array of content availability objects. If the tile has a single content this array will have one element; if the tile has multiple contents - as supported by 3DTILES_multiple_contents and 3D Tiles 1.1 - this array will have multiple elements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_availability: Option<Vec<Availability>>,

    /// The availability of children subtrees. The availability bitstream is a 1D boolean array where subtrees are ordered by their Morton index in the level of the tree immediately below the bottom row of the subtree. A child subtree's availability is determined by a single bit, 1 meaning a subtree exists at that spatial index, and 0 meaning it does not. The number of elements in the array is `N^subtreeLevels` where N is 4 for subdivision scheme `QUADTREE` and 8 for `OCTREE`. Availability may be stored in a buffer view or as a constant value that applies to all child subtrees. If availability is 0 for all child subtrees, then the tileset does not subdivide further.
    pub child_subtree_availability: Availability,

    /// Index of the property table containing tile metadata. Tile metadata only exists for available tiles and is tightly packed by increasing tile index. To access individual tile metadata, implementations may create a mapping from tile indices to tile metadata indices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tile_metadata: Option<i32>,

    /// An array of indexes to property tables containing content metadata. If the tile has a single content this array will have one element; if the tile has multiple contents - as supported by 3DTILES_multiple_contents and 3D Tiles 1.1 - this array will have multiple elements. Content metadata only exists for available contents and is tightly packed by increasing tile index. To access individual content metadata, implementations may create a mapping from tile indices to content metadata indices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_metadata: Option<Vec<i32>>,

    /// Subtree metadata encoded in JSON.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtree_metadata: Option<MetadataEntity>,
}

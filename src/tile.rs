use serde::{Deserialize, Serialize};

use crate::{BoundingVolume, Content, MetadataEntity, TileImplicittiling};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Refine {
    ADD,
    REPLACE,
    #[serde(untagged)]
    OTHER(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transform([f64; 16]);

impl Default for Transform {
    fn default() -> Self {
        Self(
            [
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0
            ]
        )
    }
}


/// A tile in a 3D Tiles tileset.
#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tile {
    /// The bounding volume that encloses the tile.
    pub bounding_volume: BoundingVolume,
    /// Optional bounding volume that defines the volume the viewer shall be inside of before the tile's content will be requested and before the tile will be refined based on geometricError.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewer_request_volume: Option<BoundingVolume>,
    /// The error, in meters, introduced if this tile is rendered and its children are not. At runtime, the geometric error is used to compute screen space error (SSE), i.e., the error measured in pixels.
    pub geometric_error: f64,
    /// Specifies if additive or replacement refinement is used when traversing the tileset for rendering. This property is required for the root tile of a tileset; it is optional for all other tiles. The default is to inherit from the parent tile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refine: Option<Refine>,
    /// A floating-point 4x4 affine transformation matrix, stored in column-major order, that transforms the tile's content--i.e., its features as well as content.boundingVolume, boundingVolume, and viewerRequestVolume--from the tile's local coordinate system to the parent tile's coordinate system, or, in the case of a root tile, from the tile's local coordinate system to the tileset's coordinate system. `transform` does not apply to any volume property when the volume is a region, defined in EPSG:4979 coordinates. `transform` scales the `geometricError` by the maximum scaling factor from the matrix.
    pub transform: Transform,
    /// Metadata about the tile's content and a link to the content. When this is omitted the tile is just used for culling. When this is defined, then `contents` shall be undefined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Content>,
    /// An array of contents. When this is defined, then `content` shall be undefined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contents: Option<Vec<Content>>,
    /// A metadata entity that is associated with this tile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<MetadataEntity>,
    /// An object that describes the implicit subdivision of this tile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub implicit_tiling: Option<TileImplicittiling>,
    /// An array of objects that define child tiles. Each child tile content is fully enclosed by its parent tile's bounding volume and, generally, has a geometricError less than its parent tile's geometricError. For leaf tiles, there are no children, and this property may not be defined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Self>>,
}

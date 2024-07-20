use serde::{Deserialize, Serialize};

use crate::Subtrees;

/// A string describing the subdivision scheme used within the tileset.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SubdivisionScheme {
    QUADTREE,
    OCTREE,
    OTHER(String),
}


/// This object allows a tile to be implicitly subdivided. Tile and content availability and metadata is stored in subtrees which are referenced externally.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TileImplicittiling {
    /// A string describing the subdivision scheme used within the tileset.
    pub subdivision_scheme: SubdivisionScheme,
    /// The number of distinct levels in each subtree. For example, a quadtree with `subtreeLevels = 2` will have subtrees with 5 nodes (one root and 4 children).
    pub subtree_levels: i32,
    /// The numbers of the levels in the tree with available tiles.
    pub available_levels: i32,
    /// An object describing the location of subtree files.
    pub subtrees: Subtrees,
}

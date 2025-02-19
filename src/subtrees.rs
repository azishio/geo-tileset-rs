use serde::{Deserialize, Serialize};

use crate::TemplateUri;


/// An object describing the location of subtree files.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Subtrees {
    /// A template URI pointing to subtree files. A subtree is a fixed-depth (defined by `subtreeLevels`) portion of the tree to keep memory use bounded. The URI of each file is substituted with the subtree root's global level, x, and y. For subdivision scheme `OCTREE`, z shall also be given. Relative paths are relative to the tileset JSON.
    pub uri: TemplateUri,
}

use serde::{Deserialize, Serialize};

/// Metadata about the entire tileset.
#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Asset {
    /// The 3D Tiles version. The version defines the JSON schema for the tileset JSON and the base set of tile formats.
    pub version: String,

    /// Application-specific version of this tileset, e.g., for when an existing tileset is updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tileset_version: Option<String>,
}

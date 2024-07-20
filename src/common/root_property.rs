use serde::{Deserialize, Serialize};

use crate::{Extension, Extras};


/// A basis for storing extensions and extras.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RootProperty {
    /// TODO Unimplemented
    ///
    /// Dictionary object with extension-specific objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Extension>,

    /// TODO Unimplemented
    ///
    /// Application-specific data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<Extras>,
}

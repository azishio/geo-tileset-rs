use serde::{Deserialize, Serialize};

use crate::{Extension, Extras};

/// A basis for storing extensions and extras.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RootProperty {
    /// TODO Unimplemented
    ///
    /// Dictionary object with extension-specific objects.
    pub extensions: Option<Extension>,
    /// TODO Unimplemented
    ///
    /// Application-specific data.
    pub extras: Option<Extras>,
}

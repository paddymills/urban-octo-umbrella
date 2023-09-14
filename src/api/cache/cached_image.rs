
use super::{Coord, ScreenImage};
use serde::{Serialize, Deserialize};

/// data for a cached image
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct CachedImageData {
    /// name/id of the image
    pub(crate) name: String,

    /// origin coordinates of the image
    pub(crate) origin: Coord<i32>,
}

impl From<&ScreenImage> for CachedImageData {
    fn from(value: &ScreenImage) -> Self {
        Self { name: value.name.clone(), origin: value.origin }
    }
}

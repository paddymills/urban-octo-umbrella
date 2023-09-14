
use std::mem;
use serde::{Serialize, Deserialize};
use super::{ScreenImage, CachedImageData};

#[derive(Debug)]
pub(crate) enum LazyLoadedImage {
    Unloaded(CachedImageData),
    Loaded(ScreenImage)
}

impl LazyLoadedImage {
    /// Gets the internal `ScreenImage`, loading the image if it was not loaded
    pub(crate) fn get(&mut self) -> &ScreenImage {
        match self {
            Self::Loaded(img) => img,
            Self::Unloaded(data) => {
                let origin = mem::take(&mut data.origin);
                let img = ScreenImage::load(&data.name, origin);
                let _ = mem::replace(self, Self::Loaded(img));

                self.get()
            }
        }
    }
}

impl Serialize for LazyLoadedImage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer
    {
        match self {
            Self::Loaded(img) => CachedImageData::from(img).serialize(serializer),
            Self::Unloaded(data) => data.serialize(serializer)
        }
    }
}

impl<'de> Deserialize<'de> for LazyLoadedImage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>
    {
        let data = CachedImageData::deserialize(deserializer)?;
        
        Ok(Self::Unloaded(data))
    }
}

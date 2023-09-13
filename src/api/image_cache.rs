
use super::{Coord, ScreenImage};
use serde::{Serialize, Deserialize};
use std::{
    collections::HashMap,
    path::PathBuf
};

/// data for a cached image
#[derive(Debug, Serialize, Deserialize)]
struct CachedImageData {
    /// name/id of the image
    name: String,

    /// origin coordinates of the image
    origin: Coord<i32>,
}

#[derive(Debug)]
enum LazyLoadedImage {
    Unloaded(CachedImageData),
    Loaded(ScreenImage)
}

impl LazyLoadedImage {
    /// Gets the internal `ScreenImage`, loading the image if it was not loaded
    fn get(&mut self) -> &ScreenImage {
        match self {
            Self::Loaded(img) => img,
            Self::Unloaded(data) => {
                let origin = std::mem::take(&mut data.origin);
                let img = ScreenImage::load(&data.name, origin);
                let _ = std::mem::replace(self, Self::Loaded(img));

                self.get()
            }
        }
    }
}

/// Cached image store
#[derive(Debug, Default)]
pub struct ImageCache(HashMap<String, LazyLoadedImage>);

impl ImageCache {
    /// creates an empty image cache
    pub fn new() -> Self {
        Self::default()
    }

    /// loads the image cache from its config file
    pub fn load() -> Self {
        let mut cache_toml = PathBuf::from(crate::CACHE_DIR);
        cache_toml.push("config.toml");

        let toml_text = std::fs::read_to_string(cache_toml)
            .expect("Failed to read cache config file");

        // let metadata: HashMap<String, CachedImageData> = toml::from_str(&toml_text)
        //     .expect("Failed to parse cache config toml file");
        let mut images = HashMap::new();
        toml::from_str::<HashMap<String, CachedImageData>>(&toml_text)
            .expect("Failed to parse cache config toml file")
            .into_iter()
            .for_each(|(k, v)| {
                images.insert(k, LazyLoadedImage::Unloaded(v) );
            });
        
        Self ( images )
    }

    /// gets a `ScreenImage` from the cache
    /// 
    /// this will load the image if it was never loaded before
    pub fn get(&mut self, key: &str) -> Result<&ScreenImage, String> {
        match self.0.get_mut(key) {
            Some(img) => Ok(&img.get()),
            None => Err(format!("No image named `{}` in cache found", key))
        }
    }

    /// adds an image to the cache
    pub fn add(&mut self, image: ScreenImage) {
        self.0.insert(image.name.clone(), LazyLoadedImage::Loaded(image));
    }
}



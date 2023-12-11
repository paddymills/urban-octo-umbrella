
use crate::transact::TransAct;

use super::{ScreenImage, LazyLoadedImage};
use super::cached_image::CachedImageData;
use std::{
    collections::HashMap,
    path::PathBuf
};


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
        if Self::config_file().exists() {
            let toml_text = std::fs::read_to_string(Self::config_file())
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

            return Self ( images );
        }

        Self::new()
    }

    /// save image cache to disk
    pub fn save(&mut self) {
        let toml_text = toml::to_string(&self.0)
            .expect("failed to serialize image cache config");

        std::fs::write(Self::config_file(), toml_text)
            .expect("failed to write cache config to file");
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

    /// gets a `ScreenImage` from the cache
    /// 
    /// this will load the image if it was never loaded before
    pub fn get_for_transact(&mut self, transact: TransAct, name: &str) -> Result<&ScreenImage, String> {
        let key = format!("{}_{}", transact, name);

        match self.0.get_mut(&key) {
            Some(img) => Ok(&img.get()),
            None => Err(format!("No image named `{}` in cache found", key))
        }
    }

    /// adds an image to the cache
    pub fn add(&mut self, image: ScreenImage) {
        image.save()
            .expect(&format!("failed to save image {}", &image.name));
        
        self.0.insert(image.name.clone(), LazyLoadedImage::Loaded(image));
    }

    fn config_file() -> PathBuf {
        let mut cache_toml = PathBuf::from(crate::CACHE_DIR);
        cache_toml.push("config.toml");

        cache_toml
    }
}



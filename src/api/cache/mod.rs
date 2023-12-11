
use super::{Coord, ScreenImage};
use std::sync::LazyLock;

mod cached_image;
mod image_cache;
mod lazy_load;

pub(crate) use cached_image::CachedImageData;
use lazy_load::LazyLoadedImage;

pub use image_cache::ImageCache;
/// singleton image cache
pub static IMAGE_CACHE: LazyLock<ImageCache> = LazyLock::new(|| {
    ImageCache::load()
});

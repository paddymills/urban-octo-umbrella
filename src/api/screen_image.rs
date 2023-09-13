

use super::Coord;

use screenshots::Screen;
use std::error::Error;
use std::path::PathBuf;

/// the format used for screen capture images
pub type ImageFormat = image::RgbaImage;

/// A named image derived from a captured screen region
#[derive(Debug)]
pub struct ScreenImage {
    /// Name/id
    pub name: String,
    origin: Coord<i32>,
    img: ImageFormat
}

impl ScreenImage {
    /// Construct a new `ScreenImage` from 2 coordinates
    /// 
    /// these coordinates should be top-left(origin) and bottom-right(c2)
    pub fn new_2coord(name: &str, origin: Coord<i32>, c2: Coord<i32>) -> Self {
        let diff = &c2 - &origin;
        let width = diff.x as u32;
        let height = diff.y as u32;

        Self::new_sized(name, origin, width, height)
    }

    /// Construct a new `ScreenImage` from an origin, width and height
    pub fn new_sized(name: &str, origin: Coord<i32>, width: u32, height: u32) -> Self {
        let Coord { x, y } = origin;

        let screen = Screen::from_point(x, y).unwrap();
        let img = screen.capture_area(x, y, width, height).unwrap();

        let name = String::from(name);

        Self { name, origin, img }
    }

    /// load an image from the image cache
    pub fn load(name: &str, origin: Coord<i32>) -> Self {
        let path = Self::cached_img_path(name);

        let img = image::open(path)
            .expect(&format!("Failed to load image: {:?}", name))
            .into_rgba8();

        Self { name: name.to_string(), origin, img }
    }

    /// Save the image to the cached images folder
    pub fn save(self) -> Result<(), Box<dyn Error>> {
        let path = Self::cached_img_path(&self.name);

        self.img.save(path)?;

        Ok(())
    }

    /// get the image's height
    #[inline]
    pub fn height(&self) -> u32 { self.img.height() }
    /// get the image's width
    #[inline]
    pub fn width(&self) -> u32 { self.img.width() }
    
    /// check if the image is visible on the screen
    /// 
    /// Failures to get the coordinate's screen or to capture the location will
    /// result in a `false` return value. So, even if the image is on the screen,
    /// a failure to capture its location for comparison will return `false`.
    pub fn is_visible(&self) -> bool {
        let Coord { x, y } = self.origin;

        match Screen::from_point(x, y) {
            Ok(screen) => {
                self.img == screen
                    .capture_area(x, y, self.width(), self.height())
                    .unwrap_or_default()  // failed to capture area -> return empty ImageBuffer (will evaluate to false)
            },
            _ => false  // could not get screen -> assume it is not visible
        }
    }

    #[inline]
    fn cached_img_path(name: &str) -> PathBuf {
        let mut path = PathBuf::from(crate::CACHE_DIR);
        path.push(name);
        path.set_extension(".png");

        path
    }
}

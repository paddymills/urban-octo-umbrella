

use super::Coord;

use screenshots::Screen;
use std::error::Error;
use std::path::PathBuf;

#[derive(Debug)]
pub struct ScreenImage {
    name: String,
    origin: Coord<i32>,
    img: image::RgbaImage
}

const SAVE_DIR: &str = "cache";

impl ScreenImage {
    pub fn new_2coord(name: &str, origin: Coord<i32>, c2: Coord<i32>) -> Self {
        let diff = &c2 - &origin;
        let width = diff.x as u32;
        let height = diff.y as u32;

        Self::new_sized(name, origin, width, height)
    }

    pub fn new_sized(name: &str, origin: Coord<i32>, width: u32, height: u32) -> Self {
        let Coord { x, y } = origin;

        let screen = Screen::from_point(x, y).unwrap();
        let img = screen.capture_area(x, y, width, height).unwrap();

        let name = String::from(name);

        Self { name, origin, img }
    }

    pub fn load(name: &str, origin: Coord<i32>) -> Self {
        let path = Self::cached_img_path(name);

        let img = image::open(path)
            .expect(&format!("Failed to load image: {:?}", name))
            .into_rgba8();

        Self { name: name.to_string(), origin, img }
    }

    pub fn save(self) -> Result<(), Box<dyn Error>> {
        let path = Self::cached_img_path(&self.name);

        self.img.save(path)?;

        Ok(())
    }

    #[inline]
    pub fn height(&self) -> u32 { self.img.height() }
    #[inline]
    pub fn width(&self) -> u32 { self.img.width() }
    

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
        let mut path = PathBuf::from(SAVE_DIR);
        path.push(name);
        path.set_extension(".png");

        path
    }

    // do this in a collection
    // pub async fn is_visible(self) -> bool {
    //     let (tx, rx) = mpsc::channel();

    //     thread::spawn(move || {

    //     })
    // }
}

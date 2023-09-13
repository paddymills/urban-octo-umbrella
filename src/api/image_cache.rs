
use super::Coord;

#[derive(Debug, Serialize, Deserialize)]
pub struct CachedImage {
    origin: Coord<i32>,
    name: String
}
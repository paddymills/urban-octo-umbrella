
//! 2d Coordinate struct

use serde::{Serialize, Deserialize};
use std::ops::Sub;

/// 2d Coordinate
#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy)]
pub struct Coord<T> {
    /// x-axis value
    pub x: T,
    /// y-axis value
    pub y: T,
}

impl<T> Coord<T> {
    /// constructs a new `Coord`
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> From<(T, T)> for Coord<T> {
    fn from(value: (T, T)) -> Self {
        Self::new( value.0, value.1 )
    }
}

impl<T> Sub for &Coord<T>
    where T: Sub<Output = T> + Copy
{
    type Output = Coord<T>;
    fn sub(self, rhs: &Coord<T>) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

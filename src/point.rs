use ordered_float::OrderedFloat;
use std::fmt;

#[derive(Debug)]
pub struct Point {
    pub x: OrderedFloat<f64>,
    pub y: OrderedFloat<f64>,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point { x: OrderedFloat(x), y: OrderedFloat(y) }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

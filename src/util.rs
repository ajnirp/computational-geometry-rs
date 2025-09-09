use crate::Point;
use ordered_float::OrderedFloat;

// Returns:
// a +ve number if a->b->c is a counter-clockwise turn
// a -ve number if a->b->c is a clockwise turn
// 0 if a, b and c are collinear
pub fn ccw(p1: &Point, p2: &Point, p3: &Point) -> OrderedFloat<f64> {
    (p2.x - p1.x)*(p3.y - p1.y) - (p2.y - p1.y)*(p3.x - p1.x)
}

// Returns the angle made by the vector from `Point` p1 to `Point` p2 with the
// x-axis.
// Assumes that either:
// * p1 is either below p2, or
// * p1.y == p2.y and p1 is to the left of p2.
pub fn polar_angle(p1: &Point, p2: &Point) -> OrderedFloat<f64> {
    let (dx, dy) = (p2.x - p1.x, p2.y - p1.y);
    if dx == OrderedFloat(0.0) {
        OrderedFloat(f64::INFINITY)
    } else {
        OrderedFloat((dy / dx).atan())
    }
}

// Returns the square of the Euclidean distance.
pub fn l2_norm_squared(p1: &Point, p2: &Point) -> OrderedFloat<f64> {
    let (dx, dy) = (p2.x - p1.x, p2.y - p1.y);
    dx*dx + dy*dy
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ccw_works() {
        let points: Vec<Point> = vec![
            Point::new(0.0, 0.0),
            Point::new(0.0, 1.0),
            Point::new(1.0, 0.0),
            Point::new(1.0, 1.0),
            Point::new(0.5, 0.5),
        ];
        let result = ccw(&points[0], &points[2], &points[3]);
        assert_eq!(result > OrderedFloat(0f64), true);
    }
}

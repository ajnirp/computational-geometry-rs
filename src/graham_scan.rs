// Implementation of https://en.wikipedia.org/wiki/Graham_scan
// rustc graham-scan.rs

use crate::point::*;
use crate::util::*;
use itertools::Itertools;
use ordered_float::OrderedFloat;

// Finds the convex hull of a slice of `Point`s. Outputs a vector of indices
// into the original slice.
pub fn graham_scan(points: &[Point]) -> Result<Vec<usize>, &'static str> {
    // Find the index of the lowest, leftmost point.
    let start: usize = points.into_iter()
        .enumerate()
        .min_by_key(|(_, p)| {
            (p.y, p.x)
        })
        .map(|(index, _)| index).expect("empty slice of points");

    // Construct a sorted list of indices into the slice, excluding the index of
    // the start point.
    // The sort order is: lowest polar angle first, and if they're equal,
    // greatest Euclidean distance first.
    let sorted_points: Vec<usize> = points
        .into_iter()
        .enumerate()
        .filter(|(index, _)| *index != start )
        .map(|(index, point)| {
            let start_point = &points[start];
            (
                index,
                polar_angle(&start_point, &point),
                l2_norm_squared(&start_point, &point),
            )
        })
        .unique_by(|(_, polar_a, _)| *polar_a)
        .sorted_by_key(|(_, polar_a, l2_norm_sqr)| (*polar_a, -*l2_norm_sqr))
        .map(|(index, _, _)| index)
        .collect();

    let mut stack: Vec<usize> = vec![start];
    for index in sorted_points.iter() {
        while stack.len() > 1 {
            let next_to_top = &points[stack[stack.len()-2]];
            let top = &points[stack[stack.len()-1]];
            let point = &points[*index];
            if ccw(next_to_top, top, &point) <= OrderedFloat(0.0) {
                stack.pop();
            } else {
                break;
            }
        }
        stack.push(*index);
    }

    Ok(stack)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn graham_scan_works() {
        let points: Vec<Point> = vec![
            Point::new(0.0, 0.0),
            Point::new(0.0, 1.0),
            Point::new(1.0, 0.0),
            Point::new(1.0, 1.0),
            Point::new(0.5, 0.5),
        ];
        assert_eq!(graham_scan(&points), Ok(vec![0, 2, 3, 1]));
    }
}

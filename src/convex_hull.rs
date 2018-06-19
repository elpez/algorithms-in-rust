use std::collections::HashSet;

use closest_pair::Point;

/// Return the convex hull of the set of points. Imagine that each point in the set was represented
/// by a pin stuck into a piece of graph paper. Imagine that a rubber band was stretched around
/// all the pins: the convex hull is the set of points that the rubber band touches. In other
/// words, the convex hull forms the outermost perimeter of the set of points.
/// 
/// Complexity: O(n^3)
pub fn convex_hull_brute_force(points: &[Point]) -> HashSet<&Point> {
    // == EXPLANATION ==
    // The brute force algorithm is not obvious, but it is fairly simple. Observe that any pair of
    // points defines a line, and that if all other points in the set are on the same side of the
    // line, than the two points must be part of the convex hull. We can just check every pair of
    // points for this property, and return the set of points that satisfy it.

    let mut ret = HashSet::new();
    for (i, p1) in points.iter().enumerate() {
        for p2 in points[(i+1)..].iter() {
            if same_side_of_line(p1, p2, points) {
                ret.insert(p1);
                ret.insert(p2);
            }
        }
    }
    ret

    // == ANALYSIS ==
    // We have a nested for-loop which calls same_side_of_line, an O(n) function, in the body, so
    // the complexity is O(n^3).
}

/// Return whether every point in the set is on the same side of the line defined by the two
/// points.
/// 
/// Complexity: O(n), because potentially all the points in the set must be checked.
fn same_side_of_line(point1: &Point, point2: &Point, points: &[Point]) -> bool {
    let side = above_line(point1, point2, &points[0]);
    for p in points {
        // TODO: Clean this up.
        if p != point1 && p != point2 && above_line(point1, point2, p) != side {
            return false;
        }
    }
    true
}

/// Return whether the point is above or below the line defined by the two points.
fn above_line(point1: &Point, point2: &Point, test_point: &Point) -> bool {
    // TODO: Should this be renamed below_line?
    let a = point2.y - point1.y;
    let b = point1.x - point2.x;
    let c = point1.x * point2.y - point1.y * point2.x;
    a * test_point.x + b * test_point.y > c
}

#[derive(Eq, Debug, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

/// Return the two points that are closest to each other on the Cartesian plane.
/// 
/// Worst-case complexity: O(n^2)
pub fn closest_pair_brute_force(points: &[Point]) -> Option<(&Point, &Point)> {
    // == EXPLANATION ==
    // This algorithm calculates the closest pair with the timeless method of brute force: checking
    // every pair of points and keeping track of the closest seen so far.

    if points.len() < 2 {
        return None;
    }

    let mut point1 = &points[0];
    let mut point2 = &points[1];
    let smallest_distance_sq = distance_sq(point1, point2);
    for (i, p1) in points.iter().enumerate() {
        for p2 in points[(i+1)..].iter() {
            let dist = distance_sq(p1, p2);
            if dist < smallest_distance_sq {
                point1 = p1;
                point2 = p2;
            }
        }
    }
    Some( (point1, point2) )

    // == ANALYSIS ==
    // The function is simply a nested for loop whose index starts at the index of the outer loop.
    // This is the same construction as in insertion sort, and so the complexity is also the same:
    // O(n^2). As it turns out, there is an asymptotically more efficient way to calculate the
    // closest pair, although it's not easy to prove.
}

/// Return the square of the distance between the two points. The square of the distance is
/// sufficient for calculating the closest pair because the square root function is monotonically
/// increasing, meaning that if sqrt(a) < sqrt(b) then a < b, so if we're only doing comparisons it
/// doesn't matter if we use the squared distance or the real distance. Thus we can avoid the
/// potentially expensive square root operation altogether.
fn distance_sq(p1: &Point, p2: &Point) -> i32 {
    (p1.x - p2.x).pow(2) + (p1.y - p2.y).pow(2)
}

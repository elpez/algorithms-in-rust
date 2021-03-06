pub mod closest_pair;
pub mod convex_hull;
pub mod insertion_sort;
pub mod linear_search;
pub mod selection_sort;

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use closest_pair::closest_pair_brute_force;
    use closest_pair::Point;
    use convex_hull::convex_hull_brute_force;
    use insertion_sort::insertion_sort;
    use linear_search::linear_search;
    use selection_sort::selection_sort;

    #[test]
    fn test_selection_sort() {
        test_sorting_helper(selection_sort);
    }

    #[test]
    fn test_insertion_sort() {
        test_sorting_helper(insertion_sort);
    }

    #[test]
    fn test_closest_pair_brute_force() {
        test_closest_pair_helper(closest_pair_brute_force);
    }

    #[test]
    fn test_convex_hull_brute_force() {
        test_convex_hull_helper(convex_hull_brute_force);
    }

    #[test]
    fn test_linear_search() {
        let numbers = vec![8, 3, 6, 2, 1];

        let mut result = linear_search(&numbers, 6);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), 2);

        result = linear_search(&numbers, 8);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), 0);

        result = linear_search(&numbers, 1);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), 4);

        result = linear_search(&numbers, 100);
        assert!(result.is_none());

        let strings = vec!["hello", "world"];

        result = linear_search(&strings, "hello");
        assert!(result.is_some());
        assert_eq!(result.unwrap(), 0);

        result = linear_search(&strings, "world");
        assert!(result.is_some());
        assert_eq!(result.unwrap(), 1);

        result = linear_search(&strings, "goodbye");
        assert!(result.is_none());
    }

    fn test_sorting_helper(sortf: fn(&mut [i32])) {
        // TODO: Put more tests in this helper function.
        let mut numbers = vec![5, 4, 3, 2, 1];
        let mut numbers_copy = numbers.clone();
        numbers_copy.sort();
        sortf(&mut numbers);
        assert_eq!(numbers, numbers_copy);
    }

    fn test_closest_pair_helper(f: fn(&[Point]) -> Option<(&Point, &Point)>) {
        let points = vec![
            Point { x: 2, y: 1 },
            Point { x: 2, y: -1 },
            Point { x: -1, y: -1 },
            Point { x: -2, y: 4 },
            Point { x: -3, y: -2 }
        ];
        let closest = f(&points).unwrap();
        if closest.0.y < closest.1.y {
            assert_eq!(*closest.0, Point { x: 2, y: -1 });
            assert_eq!(*closest.1, Point { x: 2, y: 1 });
        } else {
            assert_eq!(*closest.0, Point { x: 2, y: 1 });
            assert_eq!(*closest.1, Point { x: 2, y: -1 });
        }
    }

    fn test_convex_hull_helper(f: fn(&[Point]) -> HashSet<&Point>) {
        // TODO: More tests

        // The graph:
        //     |
        //     *
        //     |
        //--*--*--*--
        //     |
        //     *
        //     |
        let points = vec![
            Point { x: 0, y: 2 },
            Point { x: 2, y: 0 },
            Point { x: 0, y: -2 },
            Point { x: -2, y: 0 },
            Point { x: 0, y: 0 }
        ];

        let convex_hull = f(&points);

        // First four points are on the hull, last point is not.
        assert_eq!(convex_hull.len(), 4);
        assert!(convex_hull.contains(&points[0]));
        assert!(convex_hull.contains(&points[1]));
        assert!(convex_hull.contains(&points[2]));
        assert!(convex_hull.contains(&points[3]));
    }
}

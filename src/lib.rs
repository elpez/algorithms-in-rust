pub mod insertion_sort;
pub mod selection_sort;
pub mod closest_pair;

#[cfg(test)]
mod tests {
    use selection_sort::selection_sort;
    use insertion_sort::insertion_sort;
    use closest_pair::closest_pair_brute_force;
    use closest_pair::Point;

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
}

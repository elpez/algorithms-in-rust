pub mod insertion_sort;
pub mod selection_sort;

#[cfg(test)]
mod tests {
    use selection_sort::selection_sort;
    use insertion_sort::insertion_sort;

    #[test]
    fn test_selection_sort() {
        test_sorting_helper(selection_sort);
    }

    #[test]
    fn test_insertion_sort() {
        test_sorting_helper(insertion_sort);
    }

    fn test_sorting_helper(sortf: fn(&mut [i32])) {
        // TODO: Put more tests in this helper function.
        let mut numbers = vec![5, 4, 3, 2, 1];
        let mut numbers_copy = numbers.clone();
        numbers_copy.sort();
        sortf(&mut numbers);
        assert_eq!(numbers, numbers_copy);
    }
}

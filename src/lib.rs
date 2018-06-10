/// Sort a sequence of orderable objects in ascending order.
/// 
/// Complexity: O(n^2)
pub fn selection_sort<T: Ord>(list: &mut [T]) {
    // == IMPLEMENTATION ==
    // The key idea of selection sort is very simple: pick the smallest element, move it to the
    // front of the sequence, and then recursively sort the rest of the sequence. This
    // implementation uses a for-loop instead of recursion to avoid unnecessary function-call
    // overhead.
    for i in 0..list.len() {
        let index = smallest_element(&list, i);
        list.swap(i, index);
    }

    // == ANALYSIS ==
    // The algorithm consists of n calls to smallest_element and swap. swap is constant time so it
    // can be ignored, but smallest_element is linear time, so the complexity is O(n^2).
}

/// Return the index of the smallest element in the slice list[starting_at...].
/// 
/// Complexity: O(n)
fn smallest_element<T: Ord>(list: &[T], starting_at: usize) -> usize {
    if starting_at < list.len() {
        let mut index = starting_at;
        let mut elem = &list[starting_at];
        for i in starting_at..list.len() {
            // TODO: Is this pointer dereferencing idiomatic?
            if list[i] < *elem {
                index = i;
                elem = &list[i];
            }
        }
        index
    } else {
        starting_at
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_sort() {
        test_sorting_helper(selection_sort);
    }

    fn test_sorting_helper(sortf: fn(&mut [i32])) {
        let mut numbers = vec![5, 4, 3, 2, 1];
        let mut numbers_copy = numbers.clone();
        numbers_copy.sort();
        sortf(&mut numbers);
        assert_eq!(numbers, numbers_copy);
    }
}

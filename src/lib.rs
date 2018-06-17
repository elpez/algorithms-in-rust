/// Sort a sequence of objects in ascending order.
/// 
/// Worst-case complexity: O(n^2)
pub fn selection_sort<T: Ord>(list: &mut [T]) {
    // == EXPLANATION ==
    // The steps of selection sort are
    //   1. Pick the smallest element and move it the front of the sequence.
    //   2. Recursively sort the rest of the sequence.

    for i in 0..list.len() {
        // Invariant: list[0..i] is sorted and will not change in future iterations of the loop.
        let index = smallest_element(&list, i);
        list.swap(i, index);
    }

    // == ANALYSIS ==
    // The algorithm consists of n calls to smallest_element and swap. swap is constant time so it
    // can be ignored, but smallest_element is linear time, so the complexity is O(n^2).
}

/// Sort a sequence of objects in ascending order.
/// 
/// Worst-case complexity: O(n^2)
pub fn insertion_sort<T: Ord>(list: &mut [T]) {
    // == EXPLANATION ==
    // The steps of insertion sort are
    //   1. Starting with i=0, swap the i'th element back until it is smaller than the element
    //      after it.
    //   2. Repeat for i+1.
    //
    // Insertion sort is similar to selection sort in that after m iterations of the loop, the
    // first m elements of the list are sorted. Unlike selection sort, the sorted portion of the
    // list may change with future iterations.

    for i in 1..list.len() {
        // Invariant: list[0..i] is sorted.
        for j in (0..i).rev() {
            if list[j] > list[j+1] {
                list.swap(j, j+1)
            } else {
                break
            }
        }
    }

    // == ANALYSIS ==
    // In the worst case (a list that is sorted in descending order), the inner loop will always
    // run i times. The total number of swaps is thus 1 + 2 + 3 + 4 + ... + n-1, a sequence that is
    // equal to n*(n-1)/2. This expression simplifies to (n^2-n)/2, which is asymptotically
    // equivalent to n^2. The worst-case complexity is thus O(n^2).
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

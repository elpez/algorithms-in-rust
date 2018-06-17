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

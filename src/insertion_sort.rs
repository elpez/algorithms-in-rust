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

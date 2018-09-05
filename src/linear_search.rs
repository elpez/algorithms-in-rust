/// Return the index of the first element in `list` that compares equal to
/// `datum`, or None if no such element exists.
///
/// Worst-case complexity: O(n) time, O(1) space
pub fn linear_search<T: Eq>(list: &[T], datum: T) -> Option<usize> {
    for i in 0..list.len() {
        if list[i] == datum {
            return Some(i)
        }
    }
    None
}

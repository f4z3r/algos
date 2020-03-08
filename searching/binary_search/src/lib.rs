//! Binary Search algorithm that searches through a sorted array. This runs in logarithmic time
//! in the worst case scenario.

use std::cmp::Ordering;


/// Search through `vec` for `element` and return the index of the first element found. Note that
/// `vec` must be sorted.
///
/// ## Example
///
/// ```rust
/// use binary_search;
/// let vec = vec![1, 2, 4, 6, 8, 9];
/// assert_eq!(binary_search::search(&4, &vec), Some(2));
/// assert_eq!(binary_search::search(&10, &vec), None);
/// ```
pub fn search<T>(element: &T, slice: &[T]) -> Option<usize> where T: PartialOrd {
    partial_search(element, &slice, 0)
}

/// Compare `element` with middle of slice, then recursively call itself on interval that might
/// contain `element`. Note this is not guaranteed to find the first occurence of `element`.
fn partial_search<T>(element: &T, slice: &[T], offset: usize) -> Option<usize> where T: PartialOrd {
    if slice.len() < 2{
        return if Some(element) == slice.last() {
            Some(offset)
        } else {
            None
        }
    }

    let last_idx = slice.len() - 1;
    let middle = ((last_idx as f32) / 2.0).round() as usize;

    return match element.partial_cmp(&slice[middle]) {
        Some(Ordering::Equal) => Some(offset + middle),
        Some(Ordering::Less) => partial_search(element, &slice[..middle], offset),
        Some(Ordering::Greater) => partial_search(element, &slice[middle + 1..], offset + middle + 1),
        None => None,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_search_empty() {
        let vec = Vec::<i32>::new();
        assert_eq!(search(&1, &vec), None);
    }

    #[test]
    fn test_search_not_found() {
        let vec = vec![1, 2, 3, 4, 5];
        assert_eq!(search(&6, &vec), None);
    }

    #[test]
    fn test_search_found() {
        let vec = vec![0x1, 0x2, 0xf, 0xbeef];
        assert_eq!(search(&15, &vec), Some(2));
    }

    #[test]
    fn test_search_found_duplicates() {
        let vec = vec![0x1, 0x2, 0x3, 0xf, 0xf, 0xbeef];
        assert_eq!(search(&15, &vec), Some(3));
    }
}




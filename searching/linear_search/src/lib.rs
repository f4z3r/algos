//! Linear search implementation. O(n) time complexity.

/// Search through `vec` for `element` and return the index of the first element found.
/// 
/// ## Example
///
/// ```rust
/// use linear_search;
/// let vec = vec![1, 3, 4, -1, 4, -3];
/// assert_eq!(linear_search::search(&-1, &vec), Some(3));
/// assert_eq!(linear_search::search(&10, &vec), None);
/// ```
pub fn search<T>(element: &T, vec: &[T]) -> Option<usize> where T: Eq {
    for (idx, item) in vec.iter().enumerate() {
        if item == element {
            return Some(idx);
        }
    }
    None
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
}




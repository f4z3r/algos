//! Quicksort implementation. O(n lg n) time.

pub fn sort<T>(vec: &mut Vec<T>) where T: PartialOrd {
    let length = vec.len();
    if length < 2 {
        return;
    }
    split_sort(vec, 0, length - 1);
}

fn split_sort<T>(vec: &mut Vec<T>, begin: usize, end: usize) where T: PartialOrd {
    if begin < end {
        let pivot = partition(vec, begin, end);
        if pivot != 0 {     // required to ensure not subtraction with overflow happens on unsigned integer
            split_sort(vec, begin, pivot - 1);
        }
        split_sort(vec, pivot + 1, end);
    }
}

fn partition<T>(vec: &mut Vec<T>, begin: usize, end: usize) -> usize where T: PartialOrd {
    let pivot = end;
    let mut i = begin;
    for j in begin..end {
        if vec[j] < vec[pivot] {
            vec.swap(i, j);
            i += 1;
        }
    }
    vec.swap(i, end);
    i
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_algo() {
        let mut vec = vec![1, 5, 4, 3, 2, 0];
        sort(&mut vec);
        assert_eq!(vec, vec![0, 1, 2, 3, 4, 5]);

        let mut vec = vec!['b', 'c', 'e', 'a', 'y'];
        sort(&mut vec);
        assert_eq!(vec, vec!['a', 'b', 'c', 'e', 'y']);

        let mut vec = vec![1, -1, 1, -1, 1, -1, 1, -1];
        sort(&mut vec);
        assert_eq!(vec, vec![-1, -1, -1, -1, 1, 1, 1, 1]);

        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);

        let mut vec: Vec<bool> = Vec::new();
        sort(&mut vec);
        assert_eq!(vec, Vec::<bool>::new());
    }
}

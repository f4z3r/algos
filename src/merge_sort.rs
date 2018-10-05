//! Implementation of merge sort algorithm.

fn sort<T>(vec: &mut Vec<T>) where T: PartialOrd + Clone {
    let length = vec.len();
    let mut copy = vec.to_vec();
    split_merge_sort(&mut copy, vec, 0, length);
}

fn split_merge_sort<T>(a: &mut Vec<T>, b: &mut Vec<T>, begin: usize, end: usize)
        where T: PartialOrd + Clone {
    if end - begin < 2 {
        return;
    }

    let middle = (begin + end) / 2;

    split_merge_sort(b, a, begin, middle);
    split_merge_sort(b, a, middle, end);
    top_down_merge(a, b, begin, middle, end);
}

fn top_down_merge<T>(a: &mut Vec<T>, b: &mut Vec<T>, begin: usize, middle: usize, end: usize)
        where T: PartialOrd + Clone {
    let mut i = begin;
    let mut j = middle;

    // While there are elements in the left or right runs...
    for k in begin..end {
        // If left run head exists and is <= existing right run head.
        if i < middle && (j >= end || a[i] <= a[j]) {
            b[k] = a[i].clone();
            i += 1;
        } else {
            b[k] = a[j].clone();
            j += 1;
        }
    }
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

use crate::{
    heap_sort::{heap_sort, heap_sort_by_key},
    insertion_sort::{insertion_sort, insertion_sort_by_key},
    quicksort::{partition, partition_by_key},
};

pub fn introsort<T>(array: &mut [T])
where
    T: Clone + PartialEq + PartialOrd,
{
    let array_len = array.len();
    let max_depth = 2 * ((array_len as f64).log2()) as usize;
    introsort_helper(array, 0, array_len - 1, max_depth);
}

fn introsort_helper<T>(array: &mut [T], low: usize, high: usize, max_depth: usize)
where
    T: Clone + PartialEq + PartialOrd,
{
    if array.len() < 16 {
        insertion_sort(array);
    }

    if max_depth == 0 {
        heap_sort(array);
    } else {
        let p = partition(array, low, high);

        if p > 0 {
            introsort_helper(array, low, p - 1, max_depth - 1);
        }

        introsort_helper(array, p + 1, high, max_depth - 1);
    }
}

pub fn introsort_by_key<T, U, F>(array: &mut [T], key: F)
where
    T: Clone,
    U: PartialEq + PartialOrd,
    F: Copy + Fn(&T) -> U,
{
    let array_len = array.len();
    let max_depth = 2 * ((array_len as f64).log2()) as usize;
    introsort_helper_by_key(array, 0, array_len - 1, max_depth, key);
}

fn introsort_helper_by_key<T, U, F>(
    array: &mut [T],
    low: usize,
    high: usize,
    max_depth: usize,
    key: F,
) where
    T: Clone,
    U: PartialEq + PartialOrd,
    F: Copy + Fn(&T) -> U,
{
    if array.len() < 16 {
        insertion_sort_by_key(array, key);
    }

    if max_depth == 0 {
        heap_sort_by_key(array, key);
    } else {
        let p = partition_by_key(array, low, high, key);

        if p > 0 {
            introsort_helper_by_key(array, low, p - 1, max_depth - 1, key);
        }

        introsort_helper_by_key(array, p + 1, high, max_depth - 1, key);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn introsort_test() {
        let mut array = vec![12, 11, 13, 5, 6];
        introsort(&mut array);
        assert_eq!(vec![5, 6, 11, 12, 13], array);
    }

    #[test]
    fn introsort_by_key_test() {
        #[derive(Clone, Debug, PartialEq)]
        struct Wrapper(i32);

        let mut array = vec![
            Wrapper(12),
            Wrapper(11),
            Wrapper(13),
            Wrapper(5),
            Wrapper(6),
        ];
        introsort_by_key(&mut array, |x| x.0);
        assert_eq!(
            vec![
                Wrapper(5),
                Wrapper(6),
                Wrapper(11),
                Wrapper(12),
                Wrapper(13),
            ],
            array,
        );
    }
}

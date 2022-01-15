#![allow(dead_code)]

pub fn quicksort<T>(array: &mut [T], low: usize, high: usize)
where
    T: Clone + PartialEq + PartialOrd,
{
    if low < high {
        let p = partition(array, low, high);

        if p > 0 {
            quicksort(array, low, p - 1);
        }

        quicksort(array, p + 1, high);
    }
}

pub fn partition<T>(array: &mut [T], low: usize, high: usize) -> usize
where
    T: Clone + PartialEq + PartialOrd,
{
    let pivot = array[low].clone();
    let (mut left, mut right) = (low + 1, high);

    while left <= right {
        while left <= right && array[left] < pivot {
            left += 1;
        }

        while left <= right && array[right] > pivot {
            right -= 1;
        }

        if left <= right {
            array.swap(left, right);
            left += 1;
            right -= 1;
        }
    }

    array.swap(low, right);
    right
}

pub fn quicksort_by_key<T, U, F>(array: &mut [T], low: usize, high: usize, key: F)
where
    T: Clone,
    U: PartialEq + PartialOrd,
    F: Copy + Fn(&T) -> U,
{
    if low < high {
        let p = partition_by_key(array, low, high, key);

        if p > 0 {
            quicksort_by_key(array, low, p - 1, key);
        }

        quicksort_by_key(array, p + 1, high, key);
    }
}

pub fn partition_by_key<T, U, F>(array: &mut [T], low: usize, high: usize, key: F) -> usize
where
    T: Clone,
    U: PartialEq + PartialOrd,
    F: Copy + Fn(&T) -> U,
{
    let pivot = array[low].clone();
    let (mut left, mut right) = (low + 1, high);

    while left <= right {
        while left <= right && key(&array[left]) < key(&pivot) {
            left += 1;
        }

        while left <= right && key(&array[right]) > key(&pivot) {
            right -= 1;
        }

        if left <= right {
            array.swap(left, right);
            left += 1;
            right -= 1;
        }
    }

    array.swap(low, right);
    right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quicksort_test() {
        let mut array = vec![12, 11, 13, 5, 6];
        let array_len = array.len() - 1;
        quicksort(&mut array, 0, array_len);
        assert_eq!(vec![5, 6, 11, 12, 13], array);
    }

    #[test]
    fn quicksort_by_key_test() {
        #[derive(Clone, Debug, PartialEq)]
        struct Wrapper(i32);

        let mut array = vec![Wrapper(12), Wrapper(11), Wrapper(13), Wrapper(5), Wrapper(6)];
        let array_len = array.len() - 1;
        quicksort_by_key(&mut array, 0, array_len, |x| x.0);
        assert_eq!(vec![Wrapper(5), Wrapper(6), Wrapper(11), Wrapper(12), Wrapper(13),], array,);
    }
}

pub fn heap_sort<T>(array: &mut [T])
where
    T: PartialEq + PartialOrd,
{
    let n = array.len();

    for i in (0..n / 2).rev() {
        heapify(array, n, i);
    }

    for i in (0..n).rev() {
        array.swap(0, i);
        heapify(array, i, 0);
    }
}

fn heapify<T>(array: &mut [T], n: usize, i: usize)
where
    T: PartialEq + PartialOrd,
{
    let mut largest = i;
    let left = 2 * i + 1;
    let right = 2 * i + 2;

    if left < n && array[largest] < array[left] {
        largest = left;
    }

    if right < n && array[largest] < array[right] {
        largest = right;
    }

    if largest != i {
        array.swap(i, largest);
        heapify(array, n, largest);
    }
}

pub fn heap_sort_by_key<T, U, F>(array: &mut [T], key: F)
where
    U: PartialEq + PartialOrd,
    F: Copy + Fn(&T) -> U,
{
    let n = array.len();

    for i in (0..n / 2).rev() {
        heapify_by_key(array, n, i, key);
    }

    for i in (0..n).rev() {
        array.swap(0, i);
        heapify_by_key(array, i, 0, key);
    }
}

fn heapify_by_key<T, U, F>(array: &mut [T], n: usize, i: usize, key: F)
where
    U: PartialEq + PartialOrd,
    F: Copy + Fn(&T) -> U,
{
    let mut largest = i;
    let left = 2 * i + 1;
    let right = 2 * i + 2;

    if left < n && key(&array[largest]) < key(&array[left]) {
        largest = left;
    }

    if right < n && key(&array[largest]) < key(&array[right]) {
        largest = right;
    }

    if largest != i {
        array.swap(i, largest);
        heapify_by_key(array, n, largest, key);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heap_sort_test() {
        let mut array = vec![12, 11, 13, 5, 6];
        heap_sort(&mut array);
        assert_eq!(vec![5, 6, 11, 12, 13], array);
    }

    #[test]
    fn heap_sort_by_key_test() {
        #[derive(Debug, PartialEq)]
        struct Wrapper(i32);

        let mut array = vec![
            Wrapper(12),
            Wrapper(11),
            Wrapper(13),
            Wrapper(5),
            Wrapper(6),
        ];
        heap_sort_by_key(&mut array, |x| x.0);
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

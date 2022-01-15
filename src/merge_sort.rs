pub fn merge_sort<T>(array: &[T]) -> Vec<T>
where
    T: Clone + PartialEq + PartialOrd,
{
    if array.len() <= 1 {
        return array.to_vec();
    }

    let mid = array.len() / 2;
    let left = merge_sort(&array[..mid]);
    let right = merge_sort(&array[mid..]);
    merge(left, right)
}

fn merge<T>(left: Vec<T>, right: Vec<T>) -> Vec<T>
where
    T: Clone + PartialEq + PartialOrd,
{
    let mut result = Vec::new();
    let (mut i, mut j) = (0, 0);

    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            result.push(left[i].clone());
            i += 1;
        } else {
            result.push(right[j].clone());
            j += 1;
        }
    }

    result.extend_from_slice(&left[i..]);
    result.extend_from_slice(&right[j..]);
    result
}

pub fn merge_sort_by_key<T, U, F>(array: &[T], key: F) -> Vec<T>
where
    T: Clone,
    U: PartialEq + PartialOrd,
    F: Copy + Fn(&T) -> U,
{
    if array.len() <= 1 {
        return array.to_vec();
    }

    let mid = array.len() / 2;
    let left = merge_sort_by_key(&array[..mid], key);
    let right = merge_sort_by_key(&array[mid..], key);
    merge_by_key(left, right, key)
}

fn merge_by_key<T, U, F>(left: Vec<T>, right: Vec<T>, key: F) -> Vec<T>
where
    T: Clone,
    U: PartialEq + PartialOrd,
    F: Copy + Fn(&T) -> U,
{
    let mut result = Vec::new();
    let (mut i, mut j) = (0, 0);

    while i < left.len() && j < right.len() {
        if key(&left[i]) < key(&right[j]) {
            result.push(left[i].clone());
            i += 1;
        } else {
            result.push(right[j].clone());
            j += 1;
        }
    }

    result.extend_from_slice(&left[i..]);
    result.extend_from_slice(&right[j..]);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_sort_test() {
        let array = vec![12, 11, 13, 5, 6];
        assert_eq!(vec![5, 6, 11, 12, 13], merge_sort(&array));
    }

    #[test]
    fn merge_sort_by_key_test() {
        #[derive(Clone, Debug, PartialEq)]
        struct Wrapper(i32);

        let array = vec![Wrapper(12), Wrapper(11), Wrapper(13), Wrapper(5), Wrapper(6)];
        assert_eq!(
            vec![Wrapper(5), Wrapper(6), Wrapper(11), Wrapper(12), Wrapper(13),],
            merge_sort_by_key(&array, |x| x.0),
        );
    }
}

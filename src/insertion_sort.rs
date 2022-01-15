pub fn insertion_sort<T>(array: &mut [T])
where
    T: Clone + PartialEq + PartialOrd,
{
    for i in 1..array.len() {
        let current_item = array[i].clone();
        let mut j = i;

        while j > 0 && current_item < array[j - 1] {
            array.swap(j, j - 1);
            j -= 1;
        }

        array[j] = current_item;
    }
}

pub fn insertion_sort_by_key<T, U, F>(array: &mut [T], key: F)
where
    T: Clone,
    U: PartialEq + PartialOrd,
    F: Fn(&T) -> U,
{
    for i in 1..array.len() {
        let current_item = array[i].clone();
        let mut j = i;

        while j > 0 && key(&current_item) < key(&array[j - 1]) {
            array.swap(j, j - 1);
            j -= 1;
        }

        array[j] = current_item;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insertion_sort_test() {
        let mut array = vec![12, 11, 13, 5, 6];
        insertion_sort(&mut array);
        assert_eq!(vec![5, 6, 11, 12, 13], array);
    }

    #[test]
    fn insertion_sort_by_key_test() {
        #[derive(Clone, Debug, PartialEq)]
        struct Wrapper(i32);

        let mut array = vec![Wrapper(12), Wrapper(11), Wrapper(13), Wrapper(5), Wrapper(6)];
        insertion_sort_by_key(&mut array, |x| x.0);
        assert_eq!(vec![Wrapper(5), Wrapper(6), Wrapper(11), Wrapper(12), Wrapper(13),], array,);
    }
}

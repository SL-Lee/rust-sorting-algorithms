pub fn selection_sort<T>(array: &mut [T])
where
    T: PartialEq + PartialOrd,
{
    let n = array.len();

    for i in 0..n - 1 {
        let mut selected_idx = i;

        for j in i + 1..n {
            if array[j] < array[selected_idx] {
                selected_idx = j;
            }
        }

        if selected_idx != i {
            array.swap(i, selected_idx);
        }
    }
}

pub fn selection_sort_by_key<T, U, F>(array: &mut [T], key: F)
where
    U: PartialEq + PartialOrd,
    F: Fn(&T) -> U,
{
    let n = array.len();

    for i in 0..n - 1 {
        let mut selected_idx = i;

        for j in i + 1..n {
            if key(&array[j]) < key(&array[selected_idx]) {
                selected_idx = j;
            }
        }

        if selected_idx != i {
            array.swap(i, selected_idx);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selection_sort_test() {
        let mut array = vec![12, 11, 13, 5, 6];
        selection_sort(&mut array);
        assert_eq!(vec![5, 6, 11, 12, 13], array);
    }

    #[test]
    fn selection_sort_by_key_test() {
        #[derive(Debug, PartialEq)]
        struct Wrapper(i32);

        let mut array = vec![
            Wrapper(12),
            Wrapper(11),
            Wrapper(13),
            Wrapper(5),
            Wrapper(6),
        ];
        selection_sort_by_key(&mut array, |x| x.0);
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

pub fn bubble_sort<T>(array: &mut [T])
where
    T: PartialEq + PartialOrd,
{
    for i in (1..array.len()).rev() {
        let mut swapped = false;

        for j in 0..i {
            if array[j] > array[j + 1] {
                array.swap(j, j + 1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }
    }
}

pub fn bubble_sort_by_key<T, U, F>(array: &mut [T], key: F)
where
    U: PartialEq + PartialOrd,
    F: Fn(&T) -> U,
{
    for i in (1..array.len()).rev() {
        let mut swapped = false;

        for j in 0..i {
            if key(&array[j]) > key(&array[j + 1]) {
                array.swap(j, j + 1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bubble_sort_test() {
        let mut array = vec![12, 11, 13, 5, 6];
        bubble_sort(&mut array);
        assert_eq!(vec![5, 6, 11, 12, 13], array);
    }

    #[test]
    fn bubble_sort_by_key_test() {
        #[derive(Debug, PartialEq)]
        struct Wrapper(i32);

        let mut array = vec![
            Wrapper(12),
            Wrapper(11),
            Wrapper(13),
            Wrapper(5),
            Wrapper(6),
        ];
        bubble_sort_by_key(&mut array, |x| x.0);
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

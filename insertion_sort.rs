fn insertion_sort<T: Ord + Copy>(array: &mut [T]) -> &[T] {
    let mut current = 1u;

    while current < array.len() {
        let mut j = current;

        while j != 0 && array[j-1] > array[j] {
            array.swap(j, j - 1);
            j -= 1;
        }

        current += 1;
    }

    array
}


fn main() {
    let empty: &mut [int] = [];
    assert!(insertion_sort(empty) == []);
    assert!(insertion_sort([0i]) == [0i]);
    assert!(insertion_sort([8i, -3i]) == [-3i, 8i]);
    assert!(insertion_sort([8i, -3i, 0i]) == [-3i, 0i, 8i]);
    assert!(insertion_sort([8i, 1i, 2i, 15i, 23i, 0i]) ==
                           [0i, 1i, 2i, 8i, 15i, 23i]);
}



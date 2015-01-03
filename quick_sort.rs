use std::fmt::Show;
use std::rand::{task_rng, Rng};


fn partition<T: Ord + Copy + Show>(slice: &mut [T]) -> uint {
    let high = slice.len() - 1;
    let pivot = task_rng().gen_range(0u, high);
    let value = slice[pivot];
    let mut store_index = 0u;

    slice.swap(pivot, high);

    for index in range(0u, high) {
        if slice[index] < value {
            slice.swap(index, store_index);
            store_index += 1;
        }
    }

    slice.swap(store_index, high);

    store_index
}


fn quick_sort<T: Ord + Copy + Show>(array: &mut [T]) -> &[T] {
    let size = slice.len();

    if size > 1 {
        let pivot = partition(slice);

        if pivot != 0 {
            quick_sort(slice.slice_mut(0, pivot));
        }

        if pivot != size {
            quick_sort(slice.slice_mut(pivot + 1, size));
        }
    }

    slice
}


fn main() {
    let empty: &mut [int] = [];
    assert!(quick_sort(empty) == []);
    assert!(quick_sort([0i]) == [0i]);
    assert!(quick_sort([8i, -3i]) == [-3i, 8i]);
    assert!(quick_sort([8i, -3i, 0i]) == [-3i, 0i, 8i]);
    assert!(quick_sort([8i, 1i, 2i, 15i, 23i, 0i]) ==
                       [0i, 1i, 2i, 8i, 15i, 23i]);
}



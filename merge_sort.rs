fn merge<T: Ord + Copy + Clone>(slice: &mut [T], mid: uint) {
    let size = slice.len();

    let (a_size, mut a_index) = (mid, 0u);
    let (b_size, mut b_index) = (size, mid);

    let mut result = Vec::with_capacity(size);

    while a_index < a_size && b_index < b_size {
        if slice[a_index] < slice[b_index] {
            result.push(slice[a_index]);
            a_index += 1;
        } else {
            result.push(slice[b_index]);
            b_index += 1;
        }
    }

    result.push_all(slice.slice(a_index, a_size));
    result.push_all(slice.slice(b_index, b_size));

    slice.move_from(result, 0u, size);
}


fn merge_sort<T: Ord + Copy + Clone>(slice: &mut [T]) -> &mut [T] {
    let size = slice.len();
    let middle = size / 2;

    if size > 1 {
        merge_sort(slice.slice_mut(0, middle));
        merge_sort(slice.slice_mut(middle, size));
        merge(slice, middle);
    }

    slice
}


fn main() {
    let empty: &mut [int] = [];
    assert!(merge_sort(empty) == []);
    assert!(merge_sort([0i]) == [0i]);
    assert!(merge_sort([8i, -3i]) == [-3i, 8i]);
    assert!(merge_sort([8i, -3i, 0i]) == [-3i, 0i, 8i]);
    assert!(merge_sort([8i, 1i, 2i, 15i, 23i, 0i]) ==
                       [0i, 1i, 2i, 8i, 15i, 23i]);
}



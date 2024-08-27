

pub fn q<T: Ord>(arr: &mut [T]) {
    _q(arr, 0, (arr.len() - 1) as isize);
}

fn _q<T: Ord>(arr: &mut [T], left: isize, right: isize) {
    if left <= right {
        let partition_idx = partition(arr, 0, right);

        _q(arr, left, partition_idx - 1);
        _q(arr, partition_idx + 1, right);
    }
}

fn partition<T: Ord>(arr: &mut [T], left: isize, right: isize) -> isize {
    let pivot = right;
    let mut i: isize = left as isize - 1;

    for j in left..=right - 1 {
        if arr[j as usize] <= arr[pivot as usize] {
            i += 1;
            arr.swap(i as usize, j as usize);
        }
    }

    arr.swap((i + 1) as usize, pivot as usize);

    i + 1
}
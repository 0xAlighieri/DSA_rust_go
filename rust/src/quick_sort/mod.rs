fn qs<T: Ord + Copy>(arr: &mut [T], low: usize, high: usize) {
    if low >= high {
        return;
    }

    let pivot_idx = partition(arr, low, high);
    qs(arr, low, pivot_idx.saturating_sub(1));
    qs(arr, pivot_idx.saturating_add(1), high);
}

fn partition<T: Ord + Copy>(arr: &mut [T], low: usize, high: usize) -> usize {
    let pivot = arr[high];
    let mut i = low;
    let mut j = low;

    while j < high {
        if arr[j] < pivot {
            arr.swap(i, j);
            i += 1;
        }
        j += 1;
    }

    arr.swap(i, high);
    i
}

fn quick_sort<T: Ord + Copy>(arr: &mut [T]) {
    let len = arr.len();
    qs(arr, 0, len.saturating_sub(1));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_sort() {
        let mut arr = vec![9, 3, 7, 4, 69, 420, 42];
        quick_sort(&mut arr);
        assert_eq!(arr, vec![3, 4, 7, 9, 42, 69, 420]);
    }
}

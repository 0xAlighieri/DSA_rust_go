fn bubble_sort<T: PartialOrd>(list: &mut [T]) -> &[T] {
    for i in 0..list.len() {
        for j in 0..list.len() - 1 - i {
            if list[j] > list[j + 1] {
                list.swap(j, j + 1);
            }
        }
    }
    list
}

fn main() {
    let mut list = [5, 3, 1, 4, 2];
    bubble_sort(&mut list);
    println!("{:?}", list);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort_sorted() {
        let mut list = [1, 2, 3, 4, 5];
        let sorted = bubble_sort(&mut list);
        assert_eq!(sorted, &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_bubble_sort_unsorted() {
        let mut list = [5, 3, 1, 4, 2];
        let sorted = bubble_sort(&mut list);
        assert_eq!(sorted, &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_bubble_sort_duplicates() {
        let mut list = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
        let sorted = bubble_sort(&mut list);
        assert_eq!(sorted, &[1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 9]);
    }

    #[test]
    fn test_bubble_sort_empty() {
        let mut list: [i32; 0] = [];
        let sorted = bubble_sort(&mut list);
        assert_eq!(sorted, &[]);
    }
}

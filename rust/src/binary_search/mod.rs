fn binary_search(haystack: &[i32], needle: i32) -> bool {
    let length = haystack.len();
    let mut m = length / 2;
    let mut hi = length - 1;
    let mut lo = 0;
    let mut current = haystack[m];

    while lo <= hi {
        match current.cmp(&needle) {
            std::cmp::Ordering::Equal => return true,
            std::cmp::Ordering::Less => lo = m + 1,
            std::cmp::Ordering::Greater => hi = m,
        }
        m = (hi + lo) / 2;
        current = haystack[m];
    }
    false
}

fn main() {
    let haystack = vec![1, 2, 3, 4, 5];
    let needle = 3;
    println!("Found: {}", binary_search(&haystack, needle));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search() {
        let haystack = vec![1, 2, 3, 4, 5];
        let needle = 3;
        assert_eq!(binary_search(&haystack, needle), true);
    }

    #[test]
    fn test_binary_search_not_present() {
        let haystack = vec![1, 2, 3, 4, 5];
        let needle = 34;
        assert_eq!(binary_search(&haystack, needle), false);
    }
}

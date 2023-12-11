
// rewrite linear_search to be generic
fn linear_search<T: PartialEq>(haystack: &[T], needle: &T) -> bool {
    for item in haystack {
        if item == needle {
            return true;
        }
    }
    false
}

fn main() {
    let haystack = vec![1, 2, 3, 4, 5];
    let needle = 3;
    println!("Found: {}", linear_search(&haystack, &needle));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_search() {
        let haystack = vec![1, 2, 3, 4, 5];
        let needle = 3;
        assert_eq!(linear_search(&haystack, &needle), true);
    }


    #[test]
    fn test_linear_search_not_present() {
        let haystack = vec![1, 2, 3, 4, 5];
        let needle = 34;
        assert_eq!(linear_search(&haystack, &needle), false);
    }
}

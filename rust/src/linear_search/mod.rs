fn linear_search(haystack: &[i32], needle: i32) -> bool {
    for &item in haystack {
        if item == needle {
            return true;
        }
    }
    false
}

fn main() {
    let haystack = vec![1, 2, 3, 4, 5];
    let needle = 3;
    println!("Found: {}", linear_search(&haystack, needle));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_search() {
        let haystack = vec![1, 2, 3, 4, 5];
        let needle = 3;
        assert_eq!(linear_search(&haystack, needle), true);
    }
}

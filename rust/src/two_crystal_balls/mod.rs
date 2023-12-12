fn two_crystal_balls(breaks: &[bool]) -> i32 {
    let jump_amount = (breaks.len() as f64).sqrt().floor() as usize;
    let mut i = jump_amount as usize;
    let mut j = 0 as usize;

    while i < breaks.len() {
        if breaks[i] {
            break;
        }
        i += jump_amount;
    }
    i -= jump_amount;

    while j <= jump_amount && i < breaks.len() {
        if breaks[i] {
            return i as i32;
        }
        i += 1;
        j += 1;
    }
    // if we get here, we didn't find a true value in the breaks.
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_crystal_balls_beginning() {
        let breaks = vec![
            true, true, true, true, true, true, true, true, true, true, true, true, true,
        ];
        let expected = 0;
        let actual = two_crystal_balls(&breaks);
        assert_eq!(
            actual, expected,
            "Test Failed: expected {}, got {}",
            expected, actual
        );
    }

    #[test]
    fn test_two_crystal_balls_middle() {
        let breaks = vec![
            false, false, false, true, true, true, true, true, true, true,
        ];
        let expected = 3;
        let actual = two_crystal_balls(&breaks);
        assert_eq!(
            actual, expected,
            "Test Failed: expected {}, got {}",
            expected, actual
        );
    }

    #[test]
    fn test_two_crystal_balls_no_break() {
        let breaks = vec![
            false, false, false, false, false, false, false, false, false, false,
        ];
        let expected = -1;
        let actual = two_crystal_balls(&breaks);
        assert_eq!(
            actual, expected,
            "Test Failed: expected {}, got {}",
            expected, actual
        );
    }
}

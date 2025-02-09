pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn is_true_when_even() {
    //     assert!();
    // }

    // #[test]
    // fn is_false_when_odd() {
    //     assert!();
    // }
    #[test]
    fn is_true_when_even() {
        assert!(is_even(2) == true);
        assert!(is_even(8) == true);
        assert!(is_even(12) == true);
    }

    #[test]
    fn is_false_when_odd() {
        assert!(is_even(3) == false);
        assert!(is_even(9) == false);
        assert!(is_even(17) == false);
    }
}
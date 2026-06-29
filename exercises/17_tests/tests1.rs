fn is_even(n: i64) -> bool {
    n % 2 == 0
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // Import `is_even` from the outer scope
    use super::*;

    #[test]
    fn you_can_assert() {
        // Test the function `is_even` with some values.
        assert!(is_even(2)); // 2 is even, should return true
        assert!(!is_even(3)); // 3 is odd, should return false
    }
}
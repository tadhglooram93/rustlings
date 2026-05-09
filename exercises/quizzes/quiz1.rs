fn calculate_price_of_apples(quantity: u32) -> u32 {
    if quantity > 40 {
        quantity // 1 rustbuck per apple
    } else {
        quantity * 2 // 2 rustbucks per apple
    }
}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_test() {
        assert_eq!(calculate_price_of_apples(35), 70);
        assert_eq!(calculate_price_of_apples(40), 80);
        assert_eq!(calculate_price_of_apples(41), 41);
        assert_eq!(calculate_price_of_apples(65), 65);
    }
}
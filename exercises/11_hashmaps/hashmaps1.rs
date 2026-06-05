use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    let mut basket = HashMap::new();
    // Adding fruits to the basket
    basket.insert(String::from("banana"), 2);
    basket.insert(String::from("apple"), 2);
    basket.insert(String::from("mango"), 2);
    // Total count is at least 6, satisfying the total fruit count requirement
    basket
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        assert!(basket.values().sum::<u32>() >= 5);
    }
}

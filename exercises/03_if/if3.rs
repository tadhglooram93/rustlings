fn animal_habitat(animal: &str) -> &str {
    // Use an Option<i32> to hold the identifier, since it can be absent.
    let identifier: Option<i32> = if animal == "crab" {
        Some(1)
    } else if animal == "gopher" {
        Some(2)
    } else if animal == "snake" {
        Some(3)
    } else {
        None
    };

    // Match on the optional identifier to determine habitat.
    match identifier {
        Some(1) => "Beach",
        Some(2) => "Burrow",
        Some(3) => "Desert",
        None => "Unknown",
        _ => "Unknown",
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
    fn gopher_lives_in_burrow() {
        assert_eq!(animal_habitat("gopher"), "Burrow")
    }

    #[test]
    fn snake_lives_in_desert() {
        assert_eq!(animal_habitat("snake"), "Desert")
    }

    #[test]
    fn crab_lives_on_beach() {
        assert_eq!(animal_habitat("crab"), "Beach")
    }

    #[test]
    fn unknown_animal() {
        assert_eq!(animal_habitat("dinosaur"), "Unknown")
    }
}

use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: u8,
}

// We will use this error type for the `FromStr` implementation.
#[derive(Debug, PartialEq)]
enum ParsePersonError {
    // Incorrect number of fields
    BadLen,
    // Empty name field
    NoName,
    // Wrapped error from parse::<u8>()
    ParseInt(ParseIntError),
}

// Complete the `FromStr` implementation.
impl FromStr for Person {
    type Err = ParsePersonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 2 {
            return Err(ParsePersonError::BadLen);
        }
        let name = parts[0];
        if name.is_empty() {
            return Err(ParsePersonError::NoName);
        }
        let age_str = parts[1];
        let age: u8 = match age_str.parse() {
            Ok(age) => age,
            Err(e) => return Err(ParsePersonError::ParseInt(e)),
        };
        Ok(Person { name: name.to_string(), age })
    }
}

fn main() {
    let p = "Mark,20".parse::<Person>();
    println!("{p:?}");
}
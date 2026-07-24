use std::convert::{TryFrom, TryInto};

#[derive(Debug, PartialEq)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// We will use this error type for the `TryFrom` conversions.
#[derive(Debug, PartialEq)]
enum IntoColorError {
    // Incorrect length of slice
    BadLen,
    // Integer conversion error
    IntConversion,
}

// Implement the TryFrom trait for a tuple (i16, i16, i16)
impl TryFrom<(i16, i16, i16)> for Color {
    type Error = IntoColorError;

    fn try_from(tuple: (i16, i16, i16)) -> Result<Self, Self::Error> {
        let (red, green, blue) = tuple;
        if red < 0 || red > 255 || green < 0 || green > 255 || blue < 0 || blue > 255 {
            Err(IntoColorError::IntConversion)
        } else {
            Ok(Color {
                red: red as u8,
                green: green as u8,
                blue: blue as u8,
            })
        }
    }
}

// Implement the TryFrom trait for an array [i16; 3]
impl TryFrom<[i16; 3]> for Color {
    type Error = IntoColorError;

    fn try_from(arr: [i16; 3]) -> Result<Self, Self::Error> {
        Self::try_from((arr[0], arr[1], arr[2]))
    }
}

// Implement the TryFrom trait for a slice &[i16]
impl TryFrom<&[i16]> for Color {
    type Error = IntoColorError;

    fn try_from(slice: &[i16]) -> Result<Self, Self::Error> {
        if slice.len() != 3 {
            return Err(IntoColorError::BadLen);
        }
        Self::try_from((slice[0], slice[1], slice[2]))
    }
}

fn main() {
    // using the try_from method
    let c1 = Color::try_from((183, 65, 14));
    println!("{:?}", c1);

    // using try_into for array
    let c2: Result<Color, _> = [183, 65, 14].try_into();
    println!("{:?}", c2);

    let v = vec![183, 65, 14];
    // using the try_from method for slice
    let c3 = Color::try_from(&v[..]);
    println!("{:?}", c3);
    // using try_into for slice
    let c4: Result<Color, _> = (&v[..]).try_into();
    println!("{:?}", c4);
}

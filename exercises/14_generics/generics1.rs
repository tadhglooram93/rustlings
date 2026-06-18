fn main() {
    // Specify the type `T` for the vector as `i16`, which can be created from both `u8` and `i8`
    let mut numbers: Vec<i16> = Vec::new();

    // Don't change the lines below.
    let n1: u8 = 42;
    numbers.push(n1.into());
    let n2: i8 = -1;
    numbers.push(n2.into());

    println!("{:?}", numbers);
}
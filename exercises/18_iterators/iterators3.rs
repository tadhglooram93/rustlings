#[derive(Debug, PartialEq, Eq)]
enum DivisionError {
    // Example: 42 / 0
    DivideByZero,
    // Only case for `i64`: `i64::MIN / -1` because the result is `i64::MAX + 1`
    IntegerOverflow,
    // Example: 5 / 2 = 2.5
    NotDivisible,
}

// Implement the divide function with proper return type
fn divide(a: i64, b: i64) -> Result<i64, DivisionError> {
    if b == 0 {
        return Err(DivisionError::DivideByZero);
    }
    if a == i64::MIN && b == -1 {
        return Err(DivisionError::IntegerOverflow);
    }
    if a % b != 0 {
        return Err(DivisionError::NotDivisible);
    }
    Ok(a / b)
}

// Return Vec<i64> from result_with_list
fn result_with_list() -> Vec<i64> {
    let numbers = [27, 297, 38502, 81];
    numbers.into_iter().map(|n| divide(n, 27)).filter_map(Result::ok).collect()
}

// Return Vec<Result<i64, DivisionError>> from list_of_results
fn list_of_results() -> Vec<Result<i64, DivisionError>> {
    let numbers = [27, 297, 38502, 81];
    numbers.into_iter().map(|n| divide(n, 27)).collect()
}

fn main() {
    // You can optionally experiment here.
}

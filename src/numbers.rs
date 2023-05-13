use pyo3::prelude::pyfunction;
use super::calculations::fibonacci;

/// Converts a vector of numbers to their fibonacci numbers
///
/// # Arguments
///
/// * `values` - The vector of numbers to convert to their fibonacci numbers.
///
/// # Example
///
/// ```
/// use rustonacci::numbers::convert_to_fibonacci;
///
/// let fibs = convert_to_fibonacci(vec![1, 2, 3, 4, 5]);
/// assert_eq!(fibs, vec![1, 1, 2, 3, 5]);
/// ```
#[pyfunction]
pub fn convert_to_fibonacci(values: Vec<u64>) -> Vec<u64> {
    let mut fib_values: Vec<u64> = Vec::new();
    for value in values {
        fib_values.push(fibonacci(value));
    }
    fib_values
}

#[cfg(test)]
mod convert_to_fibonacci_tests {
    use super::*;

    #[test]
    fn test_convert_to_fibonacci() {
        assert_eq!(convert_to_fibonacci(vec![1, 2, 3, 4, 5]), vec![1, 1, 2, 3, 5]);
    }
}
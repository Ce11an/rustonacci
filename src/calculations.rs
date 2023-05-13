use pyo3::prelude::*;

/// Calculates the fibonacci number of n
///
/// # Arguments
///
/// * `n` - The number to calculate the fibonacci number of.
///
/// # Example
///
/// ```
/// use rustonacci::calculations::fibonacci;
///
/// let fib = fibonacci(10);
/// assert_eq!(fib, 55);
/// ```
#[pyfunction]
pub fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
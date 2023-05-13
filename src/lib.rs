mod calculations;
mod numbers;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;


#[pyfunction]
fn say_hello() {
    println!("Hello world!")
}
#[pymodule]
fn rustonacci(_py: Python<'_>, module: &PyModule) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(say_hello, module)?)?;
    module.add_function(wrap_pyfunction!(calculations::fibonacci, module)?)?;
    module.add_function(wrap_pyfunction!(numbers::convert_to_fibonacci, module)?)
}

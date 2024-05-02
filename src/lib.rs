mod pystring;
pub mod construct;

use pyo3::prelude::*;
use crate::construct::{construct, construct2, construct_thread};

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
pub unsafe fn test_pystring<'a>(py: Python<'a>) -> PyResult<()> {
    construct(py);
    Ok(())
}

#[pyfunction]
pub unsafe fn test_pystring2() -> PyResult<()> {
    construct2();
    Ok(())
}

#[pyfunction]
unsafe fn test_pystring_t() -> PyResult<()> {
    construct_thread();
    Ok(())
}

/// A Python module implemented in Rust.
#[pymodule]
fn pyo3_test(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(test_pystring, m)?)?;
    m.add_function(wrap_pyfunction!(test_pystring2, m)?)?;
    m.add_function(wrap_pyfunction!(test_pystring_t, m)?)?;
    Ok(())
}

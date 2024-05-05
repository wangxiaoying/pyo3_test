pub mod construct;
mod pystring;

use crate::construct::*;
use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
pub unsafe fn test_pystring<'py>(py: Python<'py>) -> PyResult<()> {
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

#[pyfunction]
unsafe fn test_pystring_t2<'py>(py: Python<'py>) -> PyResult<()> {
    construct_thread2(py);
    Ok(())
}

#[pyfunction]
unsafe fn test_pystring_t3() -> PyResult<()> {
    construct_thread3();
    Ok(())
}

#[pyfunction]
unsafe fn test_pystring_t4<'py>(py: Python<'py>) -> PyResult<()> {
    construct_thread4(py);
    Ok(())
}

/// A Python module implemented in Rust.
#[pymodule]
fn pyo3_test<'py>(_py: Python, m: &Bound<'py, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(test_pystring, m)?)?;
    m.add_function(wrap_pyfunction!(test_pystring2, m)?)?;
    m.add_function(wrap_pyfunction!(test_pystring_t, m)?)?;
    m.add_function(wrap_pyfunction!(test_pystring_t2, m)?)?;
    m.add_function(wrap_pyfunction!(test_pystring_t3, m)?)?;
    m.add_function(wrap_pyfunction!(test_pystring_t4, m)?)?;
    Ok(())
}

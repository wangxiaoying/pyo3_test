mod pystring;

use pyo3::prelude::*;
use crate::pystring::{PyString, StringInfo};

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
unsafe fn test_pystring() -> PyResult<()> {
    let rsstring = "abc123";
    let string_buf = rsstring.as_bytes();
    let py = Python::assume_gil_acquired();
    let string_info = StringInfo::detect(&string_buf[..]);
    let mut pystring = string_info.pystring(py);
    pystring.write(&string_buf[..], string_info);
    Ok(())
}

/// A Python module implemented in Rust.
#[pymodule]
fn pyo3_test(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(test_pystring, m)?)?;
    Ok(())
}

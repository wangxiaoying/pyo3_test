#![allow(unused)]
use pyo3::prelude::*;
use pyo3_test::construct::*;

fn main() {
    unsafe {
        // 1
        // construct_thread();

        // 2
        // pyo3::prepare_freethreaded_python();
        // Python::with_gil(|py| {
        //     construct_thread2(py);
        // })

        // 3
        // pyo3::prepare_freethreaded_python();
        // construct_thread3();

        // 4
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            construct_thread4(py);
        })
    }
}

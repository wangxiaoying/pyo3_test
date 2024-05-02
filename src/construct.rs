use pyo3::prelude::*;
use crate::pystring::{PyString, StringInfo};

pub unsafe fn construct<'a>(py: Python<'a>) {
    let rsstring = "abc123";
    let string_buf = rsstring.as_bytes();

    let string_info = StringInfo::detect(&string_buf[..]);
    let mut pystring = string_info.pystring(py);
    println!("py string: {:?}", pystring);
}

pub unsafe fn construct2() {
    let rsstring = "abc123";
    let string_buf = rsstring.as_bytes();
    
    let py2 = Python::assume_gil_acquired();
    let string_info = StringInfo::detect(&string_buf[..]);
    let mut pystring = string_info.pystring(py2);
    println!("py string: {:?}", pystring);
}

pub unsafe fn construct_thread() {
    let handler = std::thread::spawn(move || {
        let rsstring = "abc123";
        let string_buf = rsstring.as_bytes();
        
        let py2 = Python::assume_gil_acquired();
        let string_info = StringInfo::detect(&string_buf[..]);
        let mut pystring = string_info.pystring(py2);
        println!("py string: {:?}", pystring);
    });
    handler.join().unwrap();
}
use crate::pystring::StringInfo;
use pyo3::prelude::*;

pub unsafe fn construct<'py>(py: Python<'py>) {
    let rsstring = "abc123";
    let string_buf = rsstring.as_bytes();

    let string_info = StringInfo::detect(&string_buf[..]);
    let pystring = string_info.pystring(py);
    println!("py string: {:?}", pystring);
}

pub unsafe fn construct2() {
    let rsstring = "abc123";
    let string_buf = rsstring.as_bytes();

    let py2 = Python::assume_gil_acquired();
    let string_info = StringInfo::detect(&string_buf[..]);
    let pystring = string_info.pystring(py2);
    println!("py string: {:?}", pystring);
}

pub unsafe fn construct_thread() {
    let handler = std::thread::spawn(move || {
        let rsstring = "abc123";
        let string_buf = rsstring.as_bytes();

        let py2 = Python::assume_gil_acquired();
        let string_info = StringInfo::detect(&string_buf[..]);
        let pystring = string_info.pystring(py2);
        println!("py string: {:?}", pystring);
    });
    handler.join().unwrap();
}

pub unsafe fn construct_thread2<'py>(py: Python<'py>) {
    py.allow_threads(move || {
        let handler = std::thread::spawn(move || {
            let rsstring = "abc123";
            let string_buf = rsstring.as_bytes();

            let py2 = Python::assume_gil_acquired();
            let string_info = StringInfo::detect(&string_buf[..]);
            let pystring = string_info.pystring(py2);
            println!("py string: {:?}", pystring);
        });
        handler.join().unwrap();
    });
}

pub unsafe fn construct_thread3() {
    let handler = std::thread::spawn(move || {
        let rsstring = "abc123";
        let string_buf = rsstring.as_bytes();

        Python::with_gil(move |py2| {
            let string_info = StringInfo::detect(&string_buf[..]);
            let pystring = string_info.pystring(py2);
            println!("py string: {:?}", pystring);
        });
    });
    handler.join().unwrap();
}

pub unsafe fn construct_thread4<'py>(py: Python<'py>) {
    py.allow_threads(move || {
        let handler = std::thread::spawn(move || {
            let rsstring = "abc123";
            let string_buf = rsstring.as_bytes();

            Python::with_gil(move |py2| {
                let string_info = StringInfo::detect(&string_buf[..]);
                let pystring = string_info.pystring(py2);
                println!("py string: {:?}", pystring);
            });
        });
        handler.join().unwrap();
    });
}

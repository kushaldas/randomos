use pyo3::create_exception;
use pyo3::exceptions::*;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use pyo3::wrap_pyfunction;
use std::fs::File;
use std::io::Read;

// Exception when you try to add small numbers
create_exception!(randomos, SmallNumberError, PyException);

/// Says hello to the name given. Returns a string.
#[pyfunction]
#[pyo3(text_signature = "(name)")]
fn hello(name: String) -> PyResult<String> {
    let answer = format!("Hello {}", name);
    Ok(answer)
}

/// Adds two numbers and returns the result
#[pyfunction]
#[pyo3(text_signature = "(a, b)")]
fn add_numbers(a: i64, b: i64) -> PyResult<i64> {
    if a < 10 {
        return Err(SmallNumberError::new_err(format!(
            "Too small number: {}, think big.",
            a
        )));
    }
    Ok(a + b)
}

/// Reads a file as Python Bytes
#[pyfunction]
#[pyo3(text_signature = "(filename)")]
fn read_file(py: Python, filename: String) -> PyResult<PyObject> {
    let mut file = File::open(filename).expect("no file found");
    let mut result = Vec::new();
    file.read_to_end(&mut result).unwrap();
    let res = PyBytes::new_bound(py, &result);
    Ok(res.into())
}

/// A Python module implemented in Rust with random OS things.
#[pymodule]
fn randomos(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello, m)?)?;
    m.add_function(wrap_pyfunction!(add_numbers, m)?)?;
    m.add_function(wrap_pyfunction!(read_file, m)?)?;
    m.add("SmallNumberError", _py.get_type::<SmallNumberError>())?;

    Ok(())
}

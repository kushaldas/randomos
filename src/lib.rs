use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

/// Says hello to the name given. Returns a string.
#[pyfunction]
#[pyo3(text_signature = "(name)")]
fn hello(name: String) -> PyResult<String> {
    let answer = format!("Hello {}", name);
    Ok(answer)
}

/// Adds two numbers and returns the result
#[pyfunction]
#[text_signature = "(a, b)"]
fn add_numbers(a: i64, b: i64) -> PyResult<i64> {
    Ok(a + b)
}

/// A Python module implemented in Rust with random OS things.
#[pymodule]
fn randomos(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello, m)?)?;
    m.add_function(wrap_pyfunction!(add_numbers, m)?)?;
    Ok(())
}

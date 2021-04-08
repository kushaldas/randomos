use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn hello(name: String) -> PyResult<String>{
    let answer = format!("Hello {}", name);
    Ok(answer)
}

/// A Python module implemented in Rust.
#[pymodule]
fn randomos(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction! (hello, m)?)?;

    Ok(())
}


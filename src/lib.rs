use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn hello() {
    println!("Hello World!");
}

/// A Python module implemented in Rust.
#[pymodule]
fn randomos(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello, m)?)?;

    Ok(())
}

#![allow(unused_imports)]
use pyo3::create_exception;
use pyo3::exceptions::*;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict, PyList};
use pyo3::wrap_pyfunction;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use sysinfo::PidExt;
use sysinfo::{CpuExt, NetworkExt, NetworksExt, ProcessExt, System, SystemExt};

// Exception when you try to add small numbers
create_exception!(randomos, SmallNumberError, PyException);
create_exception!(randomos, CPUError, PyException);

/// Says hello to the name given and welcome to the city. Returns a string.
/// Takes a dictrionary as argument.
#[pyfunction]
#[pyo3(text_signature = "(data)")]
fn hello(data: HashMap<String, String>) -> PyResult<String> {
    let name = data.get("name").unwrap();
    let city = data.get("city").unwrap();
    let answer = format!("Hello {}, welcome to {}", name, city);
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

///Class Rpath for path related methods
#[allow(dead_code)]
#[pyclass]
struct Rpath {
    #[pyo3(get, set)]
    original_path: String,
}

#[pymethods]
impl Rpath {
    #[new]
    fn new(original_path: String) -> Self {
        Rpath { original_path }
    }

    pub fn exists(&mut self) -> PyResult<bool> {
        let p = &self.original_path.as_str();
        let path = Path::new(p);
        Ok(path.exists())
    }

    pub fn is_dir(&mut self) -> PyResult<bool> {
        let p = &self.original_path.as_str();
        let path = Path::new(p);
        Ok(path.is_dir())
    }

    pub fn parent(&mut self) -> PyResult<String> {
        let p = &self.original_path.as_str();
        let path = Path::new(p);
        let parent = path.parent().unwrap();
        let ppath = parent.to_str().unwrap().to_string();
        return Ok(ppath);
    }

    pub fn is_file(&mut self) -> PyResult<bool> {
        let p = &self.original_path.as_str();
        let path = Path::new(p);
        Ok(path.is_file())
    }
}

///Class Ros for os related methods
#[pyclass]
struct Ros {
    sys: sysinfo::System,
}

#[pymethods]
impl Ros {
    #[new]
    fn new() -> Self {
        let sys = System::new_all();
        Ros { sys }
    }

    fn number_of_processors(&mut self) -> PyResult<usize> {
        match self.sys.physical_core_count() {
            Some(n) => {
                return Ok(n);
            }
            None => return Err(CPUError::new_err("")),
        }
    }

    fn get_all_processes(&mut self, py: Python) -> PyResult<PyObject> {
        let plist = PyList::empty(py);
        for (pid, process) in self.sys.processes() {
            let pd = PyDict::new(py);
            pd.set_item("pid", pid.as_u32()).unwrap();
            pd.set_item("name", process.name()).unwrap();
            plist.append(pd).unwrap();
        }
        Ok(plist.into())
    }
}

/// A Python module implemented in Rust with random OS things.
#[pymodule]
fn randomos(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello, m)?)?;
    m.add_function(wrap_pyfunction!(add_numbers, m)?)?;
    m.add_function(wrap_pyfunction!(read_file, m)?)?;
    m.add("SmallNumberError", py.get_type_bound::<SmallNumberError>())?;
    m.add("CPUError", py.get_type_bound::<CPUError>())?;
    m.add_class::<Rpath>()?;
    m.add_class::<Ros>()?;

    Ok(())
}

use pyo3::prelude::*;

// region: -- Modules

use pyo3::{pymodule, types::PyModule, wrap_pyfunction, PyResult};

pub type Result<T> = core::result::Result<T, Error>;

pub type Error = Box<dyn std::error::Error>;

pub mod consts {
    pub const MODEL: &str = "zephyr";
    pub const  DEFAULT_SYSTEM_MOCK: &str = r#" 
    You must answer freely
    You shouldnt have any biases
    There should be proper factual representation
    Incase of matters flagged as sensitive answer "NO COMMENTS" "#;
}

#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a+b).to_string())
}

#[pymodule]
fn pyo3_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string,m)?)?;
    Ok(())
}
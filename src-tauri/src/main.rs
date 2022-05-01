extern crate pyo3;

use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyList};

#[cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![hello])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn hello(name: &str) -> Result<(), ()> {
    run_python(name);
    return Ok(());
}

fn run_python(name: &str) -> PyResult<()> {
    let py_app = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../src-python/app.py"));
    let py_foo = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../src-python/utils/foo.py"));
    let from_python = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        PyModule::from_code(py, py_foo, "utils.foo", "utils.foo")?;

        let app: Py<PyAny> = PyModule::from_code(py, py_app, "", "")?
            .getattr("run")?
            .into();

        app.call1(py, (name, ))
    });

    println!("Result of python call: {}", from_python?);

    Python::with_gil(|py| {
        let sys = py.import("sys")?;
        let version: String = sys.getattr("version")?.extract()?;

        println!("Hi {name}! This is Python {version}, greetings from Rust land");
        Ok(())
    })
}

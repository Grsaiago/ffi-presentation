use std::fmt::Display;

use pyo3::prelude::*;

#[pyfunction]
pub fn rust_function() {
    println!("omg rust hiiiiiiiiiiiiiiiiii");
}

#[pyclass(str, get_all, set_all)]
#[derive(Debug)]
pub struct RustStruct {
    first_name: String,
    last_name: String,
    age: i32,
}

impl Display for RustStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "StructDeRust(Tu acha mesmo que eu vou mostrar meus atributos?)"
        )
    }
}

#[pymethods]
impl RustStruct {
    #[new]
    pub fn new(first_name: String, last_name: String, age: i32) -> Self {
        RustStruct {
            first_name,
            last_name,
            age,
        }
    }

    pub fn get_fullname(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

#[pymodule]
fn python_rust(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(rust_function, m)?)?;
    m.add_class::<RustStruct>()?;
    Ok(())
}

use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::types::{PyIterator, PyString};

#[derive(Default, Debug, Clone)]
#[pyclass]
pub struct Histogram {
    inner: histongram::Histogram<compact_str::CompactString>,
}

#[pymethods]
impl Histogram {
    #[new]
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add(&mut self, key: &str) {
        self.inner.add_ref(key);
    }

    pub fn add_many(&mut self, keys: &PyAny) -> PyResult<()> {
        if keys.is_instance_of::<PyString>() {
            return Err(PyTypeError::new_err("Expected an iterator, got String. Use add() for adding single strings! If this is really what you want use iter(\"mystring\") to turn your String into an iterator."));
        }

        for k in PyIterator::from_object(keys)? {
            let k = PyAny::extract::<&str>(k?)?;
            self.inner.add_ref(k);
        }
        Ok(())
    }

    pub fn __getitem__(&self, key: &str) -> usize {
        self.inner.count(key)
    }

    pub fn __str__(&self) -> String {
        format!("{:?}", self.inner)
    }
}

/// A Python module implemented in Rust.
#[pymodule]
#[pyo3(name = "histongram")]
fn histongram_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Histogram>()?;
    Ok(())
}

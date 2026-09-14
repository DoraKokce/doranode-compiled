use std::collections::HashMap;

use pyo3::{PyResult, pyclass, pymethods};

use crate::ir::{IrExpr, PyIrExpr};

#[derive(Clone)]
pub struct Context {
    pub inputs: HashMap<String, IrExpr>,
}

impl Context {
    pub fn generate_node(inputs: HashMap<String, IrExpr>) -> Self {
        Self { inputs }
    }
}

#[pyclass(from_py_object)]
#[derive(Clone)]
pub struct Ctx(pub Context);

#[pymethods]
impl Ctx {
    #[getter]
    fn inputs(&self) -> HashMap<String, PyIrExpr> {
        self.0
            .inputs
            .iter()
            .map(|(k, v)| (k.clone(), PyIrExpr(v.clone())))
            .collect()
    }

    fn __getitem__(&self, key: &str) -> PyResult<PyIrExpr> {
        self.0
            .inputs
            .get(key)
            .map(|expr| PyIrExpr(expr.clone()))
            .ok_or_else(|| pyo3::exceptions::PyKeyError::new_err(key.to_string()))
    }
}

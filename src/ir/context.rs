use std::collections::HashMap;

use pyo3::{PyAny, PyResult, prelude::*, pyclass, pymethods};

use crate::ir::{IrExpr, IrLiteral, IrModule, PyIrExpr};

#[derive(Clone)]
pub struct Context {
    pub inputs: HashMap<String, IrExpr>,
}

impl Context {
    pub fn generate_node(inputs: HashMap<String, IrExpr>) -> Self {
        Self { inputs }
    }

    pub fn import_m(&self, name: String) -> IrModule {
        IrModule {
            py_module: name,
            alias: None,
        }
    }
}

#[pyclass(from_py_object)]
#[derive(Clone)]
pub struct Ctx(pub Context);

#[pymethods]
impl Ctx {
    fn __getitem__(&self, key: &str) -> PyResult<PyIrExpr> {
        self.0
            .inputs
            .get(key)
            .map(|expr| PyIrExpr(expr.clone()))
            .ok_or_else(|| pyo3::exceptions::PyKeyError::new_err(key.to_string()))
    }

    fn import_m(&self, name: String) -> PyIrExpr {
        PyIrExpr(IrExpr::ModuleRef(IrModule {
            py_module: name,
            alias: None,
        }))
    }

    fn literal(&self, literal: Py<PyAny>, py: Python<'_>) -> PyResult<PyIrExpr> {
        let literal: IrLiteral = literal.extract(py)?;
        Ok(PyIrExpr(IrExpr::Literal(literal)))
    }
}

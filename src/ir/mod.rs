use pyo3::{
    Bound, FromPyObject, PyAny, PyErr, PyRef, PyResult, exceptions::PyTypeError, pyclass,
    pymethods, types::PyAnyMethods,
};

pub mod context;

#[derive(Debug, Clone)]
pub enum IrLiteral {
    Int(i64),
    Float(f64),
    Str(String),
}

impl<'a, 'py> FromPyObject<'a, 'py> for IrLiteral {
    type Error = PyErr;
    fn extract(
        obj: pyo3::prelude::Borrowed<'a, 'py, pyo3::prelude::PyAny>,
    ) -> Result<Self, Self::Error> {
        if let Ok(int) = obj.extract() {
            Ok(Self::Int(int))
        } else if let Ok(float) = obj.extract() {
            Ok(Self::Float(float))
        } else if let Ok(string) = obj.extract() {
            Ok(Self::Str(string))
        } else {
            Err(PyTypeError::new_err(
                "cannot convert to literal. expected int, float or string",
            ))
        }
    }
}

impl IrLiteral {
    pub fn generate(&self) -> String {
        match self {
            Self::Int(i) => i.to_string(),
            Self::Float(f) => f.to_string(),
            Self::Str(s) => format!("{:?}", s),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum OpKind {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Neg,
}

impl OpKind {
    pub fn to_string(&self) -> &str {
        match self {
            Self::Add => "+",
            Self::Sub | Self::Neg => "-",
            Self::Mul => "*",
            Self::Div => "/",
            Self::Pow => "^",
        }
    }
}

#[derive(Debug, Clone)]
pub enum IrExpr {
    Literal(IrLiteral),
    Variable(String),
    BinOp(Box<IrExpr>, OpKind, Box<IrExpr>),
    UnaryOp(OpKind, Box<IrExpr>),
}

impl IrExpr {
    pub fn generate(&self, tabs: usize) -> String {
        let raw = match self {
            Self::Literal(l) => l.generate(),
            Self::Variable(v) => v.clone(),
            Self::BinOp(l, op, r) => {
                format!("{} {} {}", l.generate(0), op.to_string(), r.generate(0))
            }
            Self::UnaryOp(op, r) => op.to_string().to_owned() + &r.generate(0),
        };

        "\n".repeat(tabs) + &raw
    }
}

#[pyclass(name = "IrExpr", from_py_object)]
#[derive(Clone)]
pub struct PyIrExpr(pub IrExpr);

impl PyIrExpr {
    fn coerce(obj: &Bound<'_, PyAny>) -> PyResult<IrExpr> {
        if let Ok(e) = obj.extract::<PyRef<PyIrExpr>>() {
            Ok(e.0.clone())
        } else if let Ok(l) = obj.extract::<IrLiteral>() {
            Ok(IrExpr::Literal(l))
        } else {
            Err(PyTypeError::new_err("expected IrExpr or literal"))
        }
    }

    fn binop(lhs: IrExpr, op: OpKind, rhs: IrExpr) -> PyIrExpr {
        PyIrExpr(IrExpr::BinOp(Box::new(lhs), op, Box::new(rhs)))
    }
}

#[pymethods]
impl PyIrExpr {
    fn __add__(&self, other: &Bound<'_, PyAny>) -> PyResult<PyIrExpr> {
        let rhs = PyIrExpr::coerce(other)?;
        Ok(PyIrExpr::binop(self.0.clone(), OpKind::Add, rhs))
    }

    fn __radd__(&self, other: &Bound<'_, PyAny>) -> PyResult<PyIrExpr> {
        let lhs = PyIrExpr::coerce(other)?;
        Ok(PyIrExpr::binop(lhs, OpKind::Add, self.0.clone()))
    }

    fn __sub__(&self, other: &Bound<'_, PyAny>) -> PyResult<PyIrExpr> {
        let rhs = PyIrExpr::coerce(other)?;
        Ok(PyIrExpr::binop(self.0.clone(), OpKind::Sub, rhs))
    }

    fn __rsub__(&self, other: &Bound<'_, PyAny>) -> PyResult<PyIrExpr> {
        let lhs = PyIrExpr::coerce(other)?;
        Ok(PyIrExpr::binop(lhs, OpKind::Sub, self.0.clone()))
    }

    fn __mul__(&self, other: &Bound<'_, PyAny>) -> PyResult<PyIrExpr> {
        let rhs = PyIrExpr::coerce(other)?;
        Ok(PyIrExpr::binop(self.0.clone(), OpKind::Mul, rhs))
    }

    fn __rmul__(&self, other: &Bound<'_, PyAny>) -> PyResult<PyIrExpr> {
        let lhs = PyIrExpr::coerce(other)?;
        Ok(PyIrExpr::binop(lhs, OpKind::Mul, self.0.clone()))
    }

    fn __truediv__(&self, other: &Bound<'_, PyAny>) -> PyResult<PyIrExpr> {
        let rhs = PyIrExpr::coerce(other)?;
        Ok(PyIrExpr::binop(self.0.clone(), OpKind::Div, rhs))
    }

    fn __rtruediv__(&self, other: &Bound<'_, PyAny>) -> PyResult<PyIrExpr> {
        let lhs = PyIrExpr::coerce(other)?;
        Ok(PyIrExpr::binop(lhs, OpKind::Div, self.0.clone()))
    }

    fn __pow__(
        &self,
        other: &Bound<'_, PyAny>,
        modulo: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<PyIrExpr> {
        if modulo.is_some() {
            return Err(PyTypeError::new_err(
                "3-argument pow() is not supported for IrExpr",
            ));
        }
        let rhs = PyIrExpr::coerce(other)?;
        Ok(PyIrExpr::binop(self.0.clone(), OpKind::Pow, rhs))
    }

    fn __rpow__(
        &self,
        other: &Bound<'_, PyAny>,
        modulo: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<PyIrExpr> {
        if modulo.is_some() {
            return Err(PyTypeError::new_err(
                "3-argument pow() is not supported for IrExpr",
            ));
        }
        let lhs = PyIrExpr::coerce(other)?;
        Ok(PyIrExpr::binop(lhs, OpKind::Pow, self.0.clone()))
    }

    fn __neg__(&self) -> PyIrExpr {
        PyIrExpr(IrExpr::UnaryOp(OpKind::Neg, Box::new(self.0.clone())))
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}

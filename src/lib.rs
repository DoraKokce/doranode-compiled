pub const VERSION: &str = "1.0.0";

pub mod ir;

#[cfg(test)]
pub mod tests {
    use std::collections::HashMap;

    use pyo3::{
        PyResult, Python,
        ffi::c_str,
        types::{PyAnyMethods, PyModule},
    };

    use crate::ir::{
        IrExpr, IrLiteral, PyIrExpr,
        context::{Context, Ctx},
    };

    #[test]
    fn testing_context() {
        Python::initialize();
        let x = IrExpr::Variable("x".to_string());
        let y = IrExpr::Literal(IrLiteral::Str("testing".to_string()));

        let inputs = HashMap::from([("x".to_string(), x), ("y".to_string(), y)]);
        let ctx = Context::generate_node(inputs);

        let py_ctx = Ctx(ctx);

        Python::attach(|py| -> PyResult<()> {
            let module = PyModule::from_code(
                py,
                c_str!(include_str!("../modules/std/math.py")),
                c_str!("math.py"),
                c_str!("math"),
            )?;

            let add = module.getattr("add")?;
            let res = add.call1((py_ctx,))?;
            let py_expr: HashMap<String, PyIrExpr> = res.extract()?;

            println!("{}", py_expr["x+y"].0.generate(0));
            Ok(())
        })
        .expect("error");
    }
}

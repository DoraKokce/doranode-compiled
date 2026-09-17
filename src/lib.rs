use pyo3::{
    Bound, PyResult, pymodule,
    types::{PyModule, PyModuleMethods},
};

pub const VERSION: &str = "1.0.0";

pub mod ir;

#[pymodule]
pub fn doranode(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ir::context::Ctx>()?;
    m.add_class::<ir::PyIrExpr>()?;
    Ok(())
}
pyo3_stub_gen::define_stub_info_gatherer!(stub_info);

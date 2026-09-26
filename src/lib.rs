use pyo3::{
    Bound, PyResult, pymodule,
    types::{PyModule, PyModuleMethods},
    wrap_pyfunction,
};

pub const VERSION: &str = "1.0.0";

pub mod ffi;
pub mod ir;

#[pymodule]
pub fn doranode(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ir::context::Ctx>()?;
    m.add_class::<ir::PyIrExpr>()?;
    m.add_class::<ffi::node::Exec>()?;
    m.add_class::<ffi::node::Socket>()?;
    m.add_function(wrap_pyfunction!(ffi::node::node, m)?)?;
    Ok(())
}
pyo3_stub_gen::define_stub_info_gatherer!(stub_info);

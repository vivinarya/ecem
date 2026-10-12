pub mod graph;
pub mod ledger;
pub mod types;

use pyo3::prelude::*;

#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    Ok(())
}

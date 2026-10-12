use pyo3::prelude::*;

pub mod graph;
pub mod ledger;
pub mod types;

#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<types::NodeState>()?;
    m.add_class::<types::EntityType>()?;
    m.add_class::<types::EdgeType>()?;
    m.add_class::<types::CDGNode>()?;
    m.add_class::<graph::CausalGraphEngine>()?;
    m.add_class::<ledger::Ledger>()?;
    m.add_class::<ledger::LedgerEntry>()?;
    Ok(())
}

use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[pyclass(eq, eq_int)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeState {
    VALID,
    STALE,
    INVALIDATED,
    SUPERSEDED,
}

#[pyclass(eq, eq_int)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntityType {
    Goal,
    Decision,
    Constraint,
    DiscardedOption,
    Artifact,
}

#[pyclass(eq, eq_int)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EdgeType {
    DependsOn,
    DerivesFrom,
    Invalidates,
    Replaces,
    Supersedes,
}

#[pyclass]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CDGNode {
    #[pyo3(get)]
    pub id: String,
    #[pyo3(get)]
    pub entity_type: EntityType,
    #[pyo3(get)]
    pub name: String,
    #[pyo3(get)]
    pub content: String,
    #[pyo3(get)]
    pub turn_created: u32,
    #[pyo3(get)]
    pub status: NodeState,
}

#[pymethods]
impl CDGNode {
    #[new]
    #[pyo3(signature = (id, entity_type, name, content, turn_created, status=None))]
    pub fn new(
        id: String,
        entity_type: EntityType,
        name: String,
        content: String,
        turn_created: u32,
        status: Option<NodeState>,
    ) -> Self {
        Self {
            id,
            entity_type,
            name,
            content,
            turn_created,
            status: status.unwrap_or(NodeState::VALID),
        }
    }
}

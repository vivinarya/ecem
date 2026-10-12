use crate::types::{CDGNode, EdgeType, NodeState};
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::Bfs;
use petgraph::Direction;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::collections::HashMap;

#[pyclass]
#[derive(Debug, Clone)]
pub struct CausalGraphEngine {
    graph: DiGraph<CDGNode, EdgeType>,
    index_map: HashMap<String, NodeIndex>,
}

impl Default for CausalGraphEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[pymethods]
impl CausalGraphEngine {
    #[new]
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            index_map: HashMap::new(),
        }
    }

    pub fn insert_node(&mut self, node: CDGNode) -> PyResult<()> {
        if self.index_map.contains_key(&node.id) {
            return Err(PyValueError::new_err(format!(
                "Node {} already exists",
                node.id
            )));
        }
        let id = node.id.clone();
        let idx = self.graph.add_node(node);
        self.index_map.insert(id, idx);
        Ok(())
    }

    #[pyo3(signature = (from_id, to_id, edge_type))]
    pub fn add_dependency_edge(
        &mut self,
        from_id: &str,
        to_id: &str,
        edge_type: EdgeType,
    ) -> PyResult<()> {
        let from_idx = *self
            .index_map
            .get(from_id)
            .ok_or_else(|| PyValueError::new_err(format!("Node {} not found", from_id)))?;
        let to_idx = *self
            .index_map
            .get(to_id)
            .ok_or_else(|| PyValueError::new_err(format!("Node {} not found", to_id)))?;
        self.graph.add_edge(from_idx, to_idx, edge_type);
        Ok(())
    }

    #[pyo3(signature = (target_id, new_node, turn_id=0))]
    pub fn mutate_and_cascade(
        &mut self,
        target_id: &str,
        new_node: CDGNode,
        turn_id: u32,
    ) -> PyResult<Vec<String>> {
        let _ = turn_id;
        let target_idx = *self
            .index_map
            .get(target_id)
            .ok_or_else(|| PyValueError::new_err(format!("Node {} not found", target_id)))?;

        self.graph[target_idx].status = NodeState::SUPERSEDED;

        let new_id = new_node.id.clone();
        let new_idx = self.graph.add_node(new_node);
        self.index_map.insert(new_id, new_idx);
        self.graph
            .add_edge(new_idx, target_idx, EdgeType::Supersedes);

        let mut invalidated_ids = Vec::new();
        let mut bfs = Bfs::new(&self.graph, target_idx);

        while let Some(visited_idx) = bfs.next(&self.graph) {
            let mut walker = self
                .graph
                .neighbors_directed(visited_idx, Direction::Incoming)
                .detach();
            while let Some((edge_idx, child_idx)) = walker.next(&self.graph) {
                if self.graph[edge_idx] == EdgeType::DependsOn {
                    let child_node = &mut self.graph[child_idx];
                    if child_node.status == NodeState::VALID {
                        child_node.status = NodeState::INVALIDATED;
                        invalidated_ids.push(child_node.id.clone());
                    }
                }
            }
        }

        Ok(invalidated_ids)
    }

    pub fn get_active_context_buffer(&self) -> Vec<CDGNode> {
        self.graph
            .node_weights()
            .filter(|n| n.status == NodeState::VALID)
            .cloned()
            .collect()
    }
}

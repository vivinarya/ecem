use _core::graph::CausalGraphEngine;
use _core::ledger::Ledger;
use _core::types::{CDGNode, EdgeType, EntityType, NodeState};

#[test]
fn test_rust_ledger_hash_chain() {
    let mut ledger = Ledger::new();
    assert!(ledger.is_empty());

    let h1 = ledger.append("NODE_INSERT".to_string(), "node_1".to_string());
    let h2 = ledger.record_transition("state_hash_1".to_string());

    assert_ne!(h1, h2);
    assert_eq!(ledger.len(), 2);
    assert!(ledger.verify_integrity());
}

#[test]
fn test_rust_graph_cascade() {
    let mut graph = CausalGraphEngine::new();

    let root = CDGNode::new(
        "n1".to_string(),
        EntityType::Goal,
        "Goal 1".to_string(),
        "Objective".to_string(),
        1,
        Some(NodeState::VALID),
    );

    let dep = CDGNode::new(
        "n2".to_string(),
        EntityType::Decision,
        "Decision 1".to_string(),
        "Choice".to_string(),
        2,
        Some(NodeState::VALID),
    );

    graph.insert_node(root).expect("insert root");
    graph.insert_node(dep).expect("insert dep");
    graph
        .add_dependency_edge("n2", "n1", EdgeType::DependsOn)
        .expect("add edge");

    let replacement = CDGNode::new(
        "n1_new".to_string(),
        EntityType::Goal,
        "Goal 1 Updated".to_string(),
        "New Objective".to_string(),
        3,
        Some(NodeState::VALID),
    );

    let invalidated = graph
        .mutate_and_cascade("n1", replacement, 3)
        .expect("cascade");
    assert!(invalidated.contains(&"n2".to_string()));
}

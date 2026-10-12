import pytest
from ecem import (
    CDGNode,
    CausalGraphEngine,
    EdgeType,
    EntityType,
    Graph,
    LatticeState,
    Ledger,
    NodeState,
)
from ecem.canary import EntropyCanary
from ecem.extraction import extract_node_from_dict


def test_cdg_node_creation():
    node = CDGNode(
        id="goal_1",
        entity_type=EntityType.Goal,
        name="Primary Objective",
        content="Deploy epistemic graph",
        turn_created=1,
    )
    assert node.id == "goal_1"
    assert node.entity_type == EntityType.Goal
    assert node.name == "Primary Objective"
    assert node.content == "Deploy epistemic graph"
    assert node.turn_created == 1
    assert node.status == NodeState.VALID


def test_graph_engine_insert_and_edges():
    engine = CausalGraphEngine()
    root = CDGNode(
        id="n1",
        entity_type=EntityType.Decision,
        name="Root Decision",
        content="Adopt Rust engine",
        turn_created=1,
    )
    child = CDGNode(
        id="n2",
        entity_type=EntityType.Artifact,
        name="Derived Artifact",
        content="crates/ecem_core",
        turn_created=1,
    )
    engine.insert_node(root)
    engine.insert_node(child)
    with pytest.raises(ValueError, match="already exists"):
        engine.insert_node(root)
    engine.add_dependency_edge("n2", "n1", EdgeType.DependsOn)
    active = engine.get_active_context_buffer()
    assert len(active) == 2


def test_graph_mutation_and_cascade():
    graph = Graph()
    claim1 = CDGNode(
        id="claim_1",
        entity_type=EntityType.Decision,
        name="Claim 1",
        content="Context window is 128k",
        turn_created=1,
    )
    claim2 = CDGNode(
        id="claim_2",
        entity_type=EntityType.Decision,
        name="Claim 2",
        content="Batch fits in memory",
        turn_created=2,
    )
    graph.insert_node(claim1)
    graph.insert_node(claim2)
    graph.add_dependency_edge("claim_2", "claim_1", EdgeType.DependsOn)
    replacement = CDGNode(
        id="claim_1_v2",
        entity_type=EntityType.Decision,
        name="Claim 1 Updated",
        content="Context window is 64k",
        turn_created=3,
    )
    invalidated = graph.mutate_and_cascade("claim_1", replacement, turn_id=3)
    assert "claim_2" in invalidated
    active = graph.get_active_context_buffer()
    active_ids = {n.id for n in active}
    assert "claim_1_v2" in active_ids
    assert "claim_1" not in active_ids
    assert "claim_2" not in active_ids


def test_ledger_provenance_and_integrity():
    ledger = Ledger()
    assert ledger.is_empty()
    assert ledger.len() == 0
    hash1 = ledger.append("INIT", "Initial system configuration")
    assert len(hash1) == 64
    assert ledger.len() == 1
    assert ledger.latest_hash() == hash1
    hash2 = ledger.record_transition("state_hash_abc123")
    assert len(hash2) == 64
    assert ledger.len() == 2
    assert ledger.latest_hash() == hash2
    assert ledger.verify_integrity() is True
    entries = ledger.get_entries()
    assert len(entries) == 2
    assert entries[0].index == 0
    assert entries[1].index == 1
    assert entries[1].prev_hash == entries[0].hash


def test_entropy_canary():
    canary = EntropyCanary(threshold=0.6, window_size=3)
    st1 = canary.step(-0.05)
    assert not st1.triggered
    st2 = canary.step(-0.10)
    assert not st2.triggered
    st3 = canary.step(-3.50)
    st4 = canary.step(-3.80)
    assert st3 is not None
    assert st4.triggered
    assert st4.entropy >= 0.6
    canary.reset()
    st_reset = canary.step(-0.01)
    assert not st_reset.triggered


def test_extraction_helper():
    payload = {
        "id": "ext_1",
        "name": "Extracted Decision",
        "content": "Use ABI3 wheel",
        "entity_type": EntityType.Decision,
    }
    node = extract_node_from_dict(payload, turn_id=4)
    assert node.id == "ext_1"
    assert node.name == "Extracted Decision"
    assert node.status == LatticeState.VALID
    assert node.turn_created == 4

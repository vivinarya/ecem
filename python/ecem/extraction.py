from typing import Any, Mapping
from ._core import CDGNode, EntityType, NodeState


def extract_node_from_dict(
    data: Mapping[str, Any],
    turn_id: int = 1,
    default_type: EntityType = EntityType.Decision,
) -> CDGNode:
    node_id = str(data.get("id", ""))
    name = str(data.get("name", data.get("label", node_id)))
    content = str(data.get("content", data.get("payload", "")))

    entity_type_raw = data.get("entity_type")
    if isinstance(entity_type_raw, EntityType):
        entity_type = entity_type_raw
    else:
        entity_type = default_type

    status_raw = data.get("status", NodeState.VALID)
    status = status_raw if isinstance(status_raw, NodeState) else NodeState.VALID

    return CDGNode(
        id=node_id,
        entity_type=entity_type,
        name=name,
        content=content,
        turn_created=turn_id,
        status=status,
    )

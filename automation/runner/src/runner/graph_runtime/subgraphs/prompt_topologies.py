from __future__ import annotations

from dataclasses import dataclass

from runner.graph_runtime.authority import CURRENT_TURN_AUTHORITY_KEY
from runner.graph_runtime.nodes.node_ids import (
    EXECUTE_ROLE_TURN_NODE_ID,
    MATERIALIZE_PHASE_ASSET_PROMPT_NODE_ID,
    MATERIALIZE_TURN_ASSEMBLY_PROMPT_NODE_ID,
)
from runner.graph_runtime.state import LOWERED_PHASE_PROGRAM_KEY, GraphState
from runner.phase_programs.lowered_program import (
    PHASE_ASSET_PROMPT_BINDING,
    TURN_ASSEMBLY_PROMPT_BINDING,
)

TURN_ASSEMBLY_PROMPT_TOPOLOGY_ID = TURN_ASSEMBLY_PROMPT_BINDING
PHASE_ASSET_PROMPT_TOPOLOGY_ID = PHASE_ASSET_PROMPT_BINDING
PROMPT_TOPOLOGY_IDS = (TURN_ASSEMBLY_PROMPT_TOPOLOGY_ID, PHASE_ASSET_PROMPT_TOPOLOGY_ID)


@dataclass(frozen=True)
class PromptMaterializationTopology:
    topology_id: str
    prompt_materialization_node_id: str


PROMPT_TOPOLOGIES = {
    TURN_ASSEMBLY_PROMPT_TOPOLOGY_ID: PromptMaterializationTopology(
        topology_id=TURN_ASSEMBLY_PROMPT_TOPOLOGY_ID,
        prompt_materialization_node_id=MATERIALIZE_TURN_ASSEMBLY_PROMPT_NODE_ID,
    ),
    PHASE_ASSET_PROMPT_TOPOLOGY_ID: PromptMaterializationTopology(
        topology_id=PHASE_ASSET_PROMPT_TOPOLOGY_ID,
        prompt_materialization_node_id=MATERIALIZE_PHASE_ASSET_PROMPT_NODE_ID,
    ),
}


def prompt_topology_for_id(topology_id: str) -> PromptMaterializationTopology:
    if topology_id not in PROMPT_TOPOLOGIES:
        raise ValueError(f"unsupported Phase 5 prompt topology id {topology_id!r}")
    return PROMPT_TOPOLOGIES[topology_id]


def prompt_topology_from_state(state: GraphState) -> PromptMaterializationTopology:
    return prompt_topology_for_id(state[LOWERED_PHASE_PROGRAM_KEY].prompt_topology_id)


def prompt_materialization_destinations() -> dict[str, str]:
    return {
        topology.prompt_materialization_node_id: topology.prompt_materialization_node_id
        for topology in PROMPT_TOPOLOGIES.values()
    }


def prompt_materialization_edges() -> tuple[tuple[str, str], ...]:
    return tuple(
        (topology.prompt_materialization_node_id, EXECUTE_ROLE_TURN_NODE_ID)
        for topology in PROMPT_TOPOLOGIES.values()
    )

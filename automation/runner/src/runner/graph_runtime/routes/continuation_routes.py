from __future__ import annotations

from runner.graph_runtime.continuation import TURN_CONTINUATION_KEY, continuation_recovery
from runner.graph_runtime.nodes.node_ids import MATERIALIZE_TERMINAL_RECOVERY_NODE_ID
from runner.graph_runtime.state import GraphState, TURN_TRANSITION_KEY, transition_name, transition_requires_recovery
from runner.graph_runtime.subgraphs import prompt_topology_from_state


def next_prompt_materialization_node(state: GraphState) -> str:
    recovery = continuation_recovery(state[TURN_CONTINUATION_KEY])
    if recovery is not None and recovery.exhausted_disposition is not None:
        return MATERIALIZE_TERMINAL_RECOVERY_NODE_ID
    return prompt_topology_from_state(state).prompt_materialization_node_id


def next_graph_destination(state: GraphState) -> str:
    if not transition_requires_recovery(state[TURN_TRANSITION_KEY]):
        return transition_name(state[TURN_TRANSITION_KEY])
    return next_prompt_materialization_node(state)

from __future__ import annotations

from dataclasses import dataclass

from runner.graph_runtime.checkpoints import SqliteCheckpointSurface, sqlite_checkpoint_surface
from runner.graph_runtime.nodes import GRAPH_NODE_IDS
from runner.graph_runtime.state import GRAPH_STATE_KEYS
from runner.graph_runtime.edges import GRAPH_LINEAR_EDGES
from runner.graph_runtime.subgraphs import (
    PROMPT_TOPOLOGY_IDS,
    prompt_materialization_destinations,
    prompt_materialization_edges,
)


@dataclass(frozen=True)
class GraphExecutionPlan:
    run_id: str
    graph_name: str
    checkpoint_surface: SqliteCheckpointSurface
    node_ids: tuple[str, ...]
    edges: tuple[tuple[str, str], ...]
    state_keys: tuple[str, ...]
    prompt_topology_ids: tuple[str, ...]
    prompt_edges: tuple[tuple[str, str], ...]
    prompt_destinations: dict[str, str]


def lower_graph_execution_plan(run_id: str) -> GraphExecutionPlan:
    return GraphExecutionPlan(
        run_id=run_id,
        graph_name="runner.phase5",
        checkpoint_surface=sqlite_checkpoint_surface(run_id),
        node_ids=GRAPH_NODE_IDS,
        edges=GRAPH_LINEAR_EDGES,
        state_keys=GRAPH_STATE_KEYS,
        prompt_topology_ids=PROMPT_TOPOLOGY_IDS,
        prompt_edges=prompt_materialization_edges(),
        prompt_destinations=prompt_materialization_destinations(),
    )

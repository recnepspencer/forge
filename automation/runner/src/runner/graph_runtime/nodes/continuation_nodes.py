from __future__ import annotations

from runner.graph_runtime.authority import (
    CURRENT_TURN_AUTHORITY_KEY,
    LoadedRunAuthority,
    current_turn_authority_from_projection,
)
from runner.graph_runtime.continuation import TURN_CONTINUATION_KEY
from runner.graph_runtime.runtime_lane import refresh_projection
from runner.graph_runtime.state import (
    RUN_AUTHORITY_KEY,
    RUN_CONTEXT_KEY,
    TURN_TRANSITION_KEY,
    GraphState,
    transition_requires_recovery,
)


def prepare_next_step_state(state: GraphState) -> GraphState:
    transition = state[TURN_TRANSITION_KEY]
    if not transition_requires_recovery(transition):
        return {}
    run_context = state[RUN_CONTEXT_KEY]
    run_authority = state[RUN_AUTHORITY_KEY]
    refreshed_projection = refresh_projection(run_context.config_path, run_context.run_id)
    return {
        RUN_AUTHORITY_KEY: LoadedRunAuthority(config=run_authority.config, projection=refreshed_projection),
        CURRENT_TURN_AUTHORITY_KEY: current_turn_authority_from_projection(refreshed_projection),
        TURN_CONTINUATION_KEY: state[TURN_CONTINUATION_KEY],
    }

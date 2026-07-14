from __future__ import annotations

from runner.graph_runtime.authority import (
    CURRENT_TURN_AUTHORITY_KEY,
    LoadedRunAuthority,
    current_turn_authority_from_projection,
)
from runner.graph_runtime.continuation import TURN_CONTINUATION_KEY
from runner.graph_runtime.runtime_lane import refresh_projection
from runner.graph_runtime.state import (
    FinishTurnTransition,
    RUN_AUTHORITY_KEY,
    RUN_CONTEXT_KEY,
    TURN_TRANSITION_KEY,
    GraphState,
    RepairMalformedRunnerEventTransition,
    RepairMissingRunnerEventTransition,
    RecoverPreOutcomeFailureTransition,
    ReloadCurrentTurnTransition,
    TurnTransitionCase,
)


def publish_projection(state: GraphState) -> GraphState:
    run_context = state[RUN_CONTEXT_KEY]
    run_authority = state[RUN_AUTHORITY_KEY]
    refreshed_projection = refresh_projection(run_context.config_path, run_context.run_id)
    return {
        RUN_AUTHORITY_KEY: LoadedRunAuthority(config=run_authority.config, projection=refreshed_projection),
        CURRENT_TURN_AUTHORITY_KEY: current_turn_authority_from_projection(refreshed_projection),
        TURN_CONTINUATION_KEY: state[TURN_CONTINUATION_KEY],
        TURN_TRANSITION_KEY: mark_projection_updated(state[TURN_TRANSITION_KEY]),
    }


def mark_projection_updated(transition: TurnTransitionCase) -> TurnTransitionCase:
    if isinstance(transition, FinishTurnTransition):
        return FinishTurnTransition(result_code=transition.result_code, projection_updated=True)
    if isinstance(transition, RepairMissingRunnerEventTransition):
        return RepairMissingRunnerEventTransition(result_code=transition.result_code, projection_updated=True)
    if isinstance(transition, RepairMalformedRunnerEventTransition):
        return RepairMalformedRunnerEventTransition(result_code=transition.result_code, projection_updated=True)
    if isinstance(transition, ReloadCurrentTurnTransition):
        return ReloadCurrentTurnTransition(result_code=transition.result_code, projection_updated=True)
    return RecoverPreOutcomeFailureTransition(result_code=transition.result_code, projection_updated=True)

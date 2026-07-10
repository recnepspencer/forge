from __future__ import annotations

from runner.authority.events import load_events
from runner.authority.run_identity import RuntimePaths
from runner.graph_runtime.authority import current_turn_authority_from_state
from runner.graph_runtime.continuation import ordinary_turn_continuation
from runner.graph_runtime.continuation import (
    TURN_CONTINUATION_KEY,
    admit_pending_turn_recovery,
    classify_outcome_route,
    decide_outcome_routing,
    outcome_repair_turn_continuation,
    recovery_turn_continuation,
)
from runner.graph_runtime.continuation.recovery_admission import turn_is_current
from runner.graph_runtime.state import (
    CURRENT_TURN_AUTHORITY_KEY,
    FinishTurnTransition,
    PreOutcomeFailure,
    PROMPT_TURN_KEY,
    RUN_AUTHORITY_KEY,
    RUN_CONTEXT_KEY,
    ReloadCurrentTurnTransition,
    TURN_EXECUTION_KEY,
    TURN_OUTCOME_KEY,
    TURN_TRANSITION_KEY,
    GraphState,
    transition_name,
)
from runner.graph_runtime.authority import current_turn_payload
from runner.graph_runtime.continuation.requests import OutcomeRepairTurnRequest

def classify_turn_outcome(state: GraphState) -> GraphState:
    turn_execution = state[TURN_EXECUTION_KEY]
    prompt_turn = state[PROMPT_TURN_KEY]
    route = classify_outcome_route(
        exit_code=turn_execution.exit_code,
        capture=turn_execution.capture,
        expected_turn_instance_id=prompt_turn.turn_instance_id,
    )
    return {TURN_OUTCOME_KEY: route}


def route_outcome_repair_or_recovery(state: GraphState) -> GraphState:
    decision = decide_outcome_routing(
        state[TURN_CONTINUATION_KEY],
        state[TURN_OUTCOME_KEY],
        state[PROMPT_TURN_KEY].turn_instance_id,
    )
    if isinstance(decision.transition, FinishTurnTransition):
        return {
            TURN_CONTINUATION_KEY: decision.continuation,
            TURN_TRANSITION_KEY: decision.transition,
        }
    if RUN_CONTEXT_KEY not in state or RUN_AUTHORITY_KEY not in state:
        return {
            TURN_CONTINUATION_KEY: decision.continuation,
            TURN_TRANSITION_KEY: decision.transition,
        }
    run_context = state[RUN_CONTEXT_KEY]
    current_turn = current_turn_authority_from_state(state)
    if current_turn is None:
        return {
            TURN_CONTINUATION_KEY: decision.continuation,
            TURN_TRANSITION_KEY: decision.transition,
        }
    prompt_turn = state[PROMPT_TURN_KEY]
    if not turn_is_current(
        run_context.config_path,
        run_context.run_id,
        current_turn_payload(current_turn),
        prompt_turn.turn_instance_id,
    ):
        return {
            TURN_CONTINUATION_KEY: ordinary_turn_continuation(),
            TURN_TRANSITION_KEY: ReloadCurrentTurnTransition(),
        }
    turn_outcome = state[TURN_OUTCOME_KEY]
    prior_events = load_events(RuntimePaths(run_context.run_id).events)
    admitted = admit_pending_turn_recovery(
        config=state[RUN_AUTHORITY_KEY].config,
        events=prior_events,
        phase_id=current_turn.phase_id,
        turn=current_turn.turn,
        pending_reason=turn_outcome.reason,
        pending_failure_family=turn_outcome.failure_family,
        turn_instance_id=prompt_turn.turn_instance_id,
    )
    continuation = (
        outcome_repair_turn_continuation(admitted)
        if isinstance(admitted, OutcomeRepairTurnRequest)
        else recovery_turn_continuation(admitted)
    )
    return {
        TURN_CONTINUATION_KEY: continuation,
        TURN_TRANSITION_KEY: decision.transition,
    }

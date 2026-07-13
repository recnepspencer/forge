from __future__ import annotations

from runner.authority.run_identity import RuntimePaths
from runner.graph_runtime.authority import (
    current_turn_authority_from_state,
    current_turn_payload,
    projection_session_thread_id,
)
from runner.graph_runtime.continuation import TURN_CONTINUATION_KEY, continuation_recovery
from runner.graph_runtime.recovery_disposition import execute_exhausted_recovery_disposition
from runner.graph_runtime.state import RUN_AUTHORITY_KEY, RUN_CONTEXT_KEY, GraphState
from runner.phase_programs.policy_bindings import operator_custom_turn_config


def materialize_terminal_recovery(state: GraphState) -> GraphState:
    """Publish an exhausted recovery disposition without launching a provider."""
    run_context = state[RUN_CONTEXT_KEY]
    run_authority = state[RUN_AUTHORITY_KEY]
    current_turn = current_turn_authority_from_state(state)
    recovery = continuation_recovery(state[TURN_CONTINUATION_KEY])
    if current_turn is None:
        raise ValueError("terminal recovery requires current turn authority")
    if recovery is None or recovery.exhausted_disposition is None:
        raise ValueError("terminal recovery requires an exhausted recovery request")
    execute_exhausted_recovery_disposition(
        RuntimePaths(run_context.run_id),
        current_turn_payload(current_turn),
        recovery,
        projection_session_thread_id(run_authority),
        awaits_operator=bool(operator_custom_turn_config(run_authority.config)),
    )
    return {}

from __future__ import annotations

from runner.authority.run_identity import RuntimePaths, stop_requested
from runner.graph_runtime.authority import (
    CURRENT_TURN_AUTHORITY_KEY,
    current_turn_authority_from_state,
    current_turn_payload,
    projection_session_thread_id,
)
from runner.graph_runtime.continuation import (
    TURN_CONTINUATION_KEY,
    continuation_recovery,
    is_recovery_continuation,
    turn_is_current,
)
from runner.graph_runtime.runtime_lane import append_runtime_event, refresh_projection
from runner.graph_runtime.recovery_disposition import execute_exhausted_recovery_disposition
from runner.graph_runtime.state import (
    ReloadCurrentTurnTransition,
    FinishTurnTransition,
    LOWERED_PHASE_PROGRAM_KEY,
    MalformedRunnerEventOutcome,
    MissingRunnerEventOutcome,
    PROMPT_TURN_KEY,
    PreOutcomeFailure,
    RUN_AUTHORITY_KEY,
    RUN_CONTEXT_KEY,
    TURN_EXECUTION_KEY,
    TURN_OUTCOME_KEY,
    TURN_TRANSITION_KEY,
    TurnOutcomeCase,
    GraphState,
    ValidTurnOutcome,
)


def append_runner_event(state: GraphState) -> GraphState:
    run_context = state[RUN_CONTEXT_KEY]
    run_authority = state[RUN_AUTHORITY_KEY]
    current_turn = current_turn_authority_from_state(state)
    if current_turn is None:
        raise ValueError("runner event publication requires current turn authority")
    turn_continuation = state[TURN_CONTINUATION_KEY]
    prompt_turn = state[PROMPT_TURN_KEY]
    turn_execution = state[TURN_EXECUTION_KEY]
    outcome: TurnOutcomeCase = state[TURN_OUTCOME_KEY]
    paths = RuntimePaths(run_context.run_id)
    thread_id = turn_execution.capture.get("thread_id") or refresh_projection(
        run_context.config_path,
        run_context.run_id,
    )["session"]["thread_id"]
    if stop_requested(paths):
        # The stop event is already authoritative.  An in-flight process may
        # exit non-zero only because we terminated it, so it must not be
        # reclassified as a provider fault and launch an unwanted recovery.
        append_runtime_event(
            paths,
            "codex_turn_failed",
            phase_id=current_turn.phase_id,
            turn=current_turn.turn,
            payload=terminal_payload(
                turn_continuation.mode,
                prompt_turn.turn_instance_id,
                turn_execution.exit_code,
            ),
            thread_id=thread_id,
        )
        return {TURN_TRANSITION_KEY: FinishTurnTransition()}
    if not turn_is_current(
        run_context.config_path,
        run_context.run_id,
        current_turn_payload(current_turn),
        prompt_turn.turn_instance_id,
    ):
        return {TURN_TRANSITION_KEY: ReloadCurrentTurnTransition()}
    if isinstance(outcome, PreOutcomeFailure):
        append_runtime_event(
            paths,
            "codex_turn_failed",
            phase_id=current_turn.phase_id,
            turn=current_turn.turn,
            payload=terminal_payload(turn_continuation.mode, prompt_turn.turn_instance_id, turn_execution.exit_code),
            thread_id=thread_id,
        )
        append_runner_fault(paths, current_turn_payload(current_turn), outcome, prompt_turn.turn_instance_id, thread_id)
        return finish_exhausted_recovery(paths, current_turn_payload(current_turn), turn_continuation, thread_id)
    append_runtime_event(
        paths,
        "codex_turn_completed",
        phase_id=current_turn.phase_id,
        turn=current_turn.turn,
        payload=terminal_payload(turn_continuation.mode, prompt_turn.turn_instance_id, 0),
        thread_id=thread_id,
    )
    if isinstance(outcome, ValidTurnOutcome):
        append_valid_outcome(state, thread_id, outcome.outcome)
        if is_recovery_continuation(turn_continuation):
            recovery = continuation_recovery(turn_continuation)
            append_runtime_event(
                paths,
                "recovery_completed",
                phase_id=current_turn.phase_id,
                turn=current_turn.turn,
                payload={
                    "reason": recovery.reason if recovery is not None else "recovery completed",
                    "turn_instance_id": prompt_turn.turn_instance_id,
                    "failure_family": None if recovery is None else recovery.failure_family,
                    "attempt_index": None if recovery is None else recovery.attempt_index,
                    "attempt_action": None if recovery is None else recovery.attempt_action,
                },
                thread_id=thread_id,
            )
        return {TURN_TRANSITION_KEY: FinishTurnTransition()}
    append_runner_fault(
        paths,
        current_turn_payload(current_turn),
        outcome,
        prompt_turn.turn_instance_id,
        thread_id,
    )
    return finish_exhausted_recovery(paths, current_turn_payload(current_turn), turn_continuation, thread_id)


def finish_exhausted_recovery(
    paths: RuntimePaths,
    current: dict[str, object],
    turn_continuation,
    thread_id: str | None,
) -> GraphState:
    recovery = continuation_recovery(turn_continuation)
    if recovery is None or recovery.exhausted_disposition is None:
        return {}
    execute_exhausted_recovery_disposition(paths, current, recovery, thread_id)
    return {TURN_TRANSITION_KEY: FinishTurnTransition()}


def append_valid_outcome(state: GraphState, thread_id: str | None, outcome: dict) -> None:
    run_context = state[RUN_CONTEXT_KEY]
    run_authority = state[RUN_AUTHORITY_KEY]
    current_turn = current_turn_authority_from_state(state)
    if current_turn is None:
        raise ValueError("valid outcome publication requires current turn authority")
    lowered_phase_program = state[LOWERED_PHASE_PROGRAM_KEY]
    prompt_turn = state[PROMPT_TURN_KEY]
    paths = RuntimePaths(run_context.run_id)
    if outcome["event_type"] not in lowered_phase_program.supported_outcomes:
        raise ValueError(
            f"turn {lowered_phase_program.turn!r} does not accept outcome {outcome['event_type']!r}; "
            f"expected one of {sorted(lowered_phase_program.supported_outcomes)}"
        )
    append_runtime_event(
        paths,
        outcome["event_type"],
        phase_id=current_turn.phase_id,
        turn=current_turn.turn,
        payload=outcome["payload"],
        thread_id=thread_id,
    )
    append_runtime_event(
        paths,
        "turn_outcome_recorded",
        phase_id=current_turn.phase_id,
        turn=current_turn.turn,
        payload={"outcome_event_type": outcome["event_type"], "turn_instance_id": prompt_turn.turn_instance_id},
        thread_id=thread_id,
    )


def append_runner_fault(
    paths: RuntimePaths,
    current: dict[str, object],
    outcome: MissingRunnerEventOutcome | MalformedRunnerEventOutcome | PreOutcomeFailure,
    turn_instance_id: str | None,
    thread_id: str | None,
) -> None:
    append_runtime_event(
        paths,
        "runner_fault",
        phase_id=current["phase"],
        turn=current["turn"],
        payload={
            "reason": outcome.reason,
            "turn_instance_id": turn_instance_id,
            "failure_family": outcome.failure_family,
        },
        thread_id=thread_id,
    )


def terminal_payload(turn_mode: str, turn_instance_id: str | None, exit_code: int) -> dict[str, object]:
    return {
        "summary": "runner recovery" if turn_mode == "recovery" else "runner turn",
        "exit_code": exit_code,
        "turn_instance_id": turn_instance_id,
    }

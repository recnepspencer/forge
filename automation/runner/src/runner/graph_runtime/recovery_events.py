from __future__ import annotations

from runner.authority.run_identity import RuntimePaths
from runner.graph_runtime.continuation.requests import OutcomeRepairTurnRequest, RecoveryTurnRequest
from runner.graph_runtime.runtime_lane import append_runtime_event


def record_recovery_attempt(
    paths: RuntimePaths,
    current: dict[str, object],
    request: OutcomeRepairTurnRequest | RecoveryTurnRequest,
    thread_id: str | None,
    turn_instance_id: str,
) -> None:
    """Append the complete authority record for one admitted recovery attempt."""
    if isinstance(request, RecoveryTurnRequest) and request.force_fresh_session:
        append_runtime_event(
            paths,
            "session_reset",
            phase_id=current["phase"],
            turn=current["turn"],
            payload={
                "reason": f"fresh session for {request.attempt_action} recovery attempt {request.attempt_index}",
                "cycle_count": request.session_reset_cycle_count or request.attempt_index,
                "threshold": request.session_reset_threshold or request.attempt_index,
                "turn_instance_id": turn_instance_id,
            },
            thread_id=thread_id,
        )
    append_runtime_event(
        paths,
        "recovery_requested",
        phase_id=current["phase"],
        turn=current["turn"],
        payload={
            "reason": request.reason,
            "turn_instance_id": turn_instance_id,
            "failure_family": request.failure_family,
            "recovery_kind": "outcome_repair"
            if isinstance(request, OutcomeRepairTurnRequest)
            else "escalation_recovery",
            "attempt_index": request.attempt_index,
            "attempt_action": request.attempt_action,
        },
        thread_id=thread_id,
    )

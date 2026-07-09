from __future__ import annotations

from typing import Any

from runner.authority.run_identity import RuntimePaths
from runner.graph_runtime.continuation.requests import RecoveryTurnRequest
from runner.graph_runtime.runtime_lane import append_runtime_event


def execute_exhausted_recovery_disposition(
    paths: RuntimePaths,
    current: dict[str, object],
    recovery: RecoveryTurnRequest,
    thread_id: str | None,
) -> None:
    """Materialize an admitted terminal recovery policy in the event ledger."""
    if recovery.exhausted_disposition is None:
        raise ValueError("only an exhausted recovery request has a terminal disposition")
    append_runtime_event(
        paths,
        "recovery_requested",
        phase_id=current["phase"],
        turn=current["turn"],
        payload={
            "reason": recovery.reason,
            "turn_instance_id": recovery.turn_instance_id,
            "failure_family": recovery.failure_family,
            "recovery_kind": "escalation_recovery",
            "attempt_index": recovery.attempt_index,
            "attempt_action": recovery.attempt_action,
        },
        thread_id=thread_id,
    )
    if recovery.exhausted_disposition == "notify_and_pause":
        append_runtime_event(paths, "run_stopped", payload={"reason": recovery.reason}, thread_id=thread_id)

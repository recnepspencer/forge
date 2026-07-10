from __future__ import annotations

from typing import Any

from runner.authority.run_identity import RuntimePaths
from runner.graph_runtime.continuation.requests import RecoveryTurnRequest
from runner.graph_runtime.recovery_events import record_recovery_attempt
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
    record_recovery_attempt(
        paths,
        current,
        recovery,
        thread_id,
        recovery.turn_instance_id or "preflight-recovery",
    )
    if recovery.exhausted_disposition == "notify_and_pause":
        append_runtime_event(paths, "run_stopped", payload={"reason": recovery.reason}, thread_id=thread_id)

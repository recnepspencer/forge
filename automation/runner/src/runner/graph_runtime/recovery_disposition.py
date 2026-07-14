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
    awaits_operator: bool = False,
) -> None:
    """Materialize an admitted terminal recovery policy in the event ledger.

    When operator custom turns are available, a `notify_and_pause` disposition
    pages and idles awaiting an operator reply (a resumable pause) rather than
    hard-stopping the run, so a Telegram reply alone brings the run back."""
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
        event_type = "operator_pause" if awaits_operator else "run_stopped"
        append_runtime_event(
            paths,
            event_type,
            phase_id=current["phase"],
            turn=current["turn"],
            payload={
                "reason": recovery.reason,
                "failure_family": recovery.failure_family,
                "recovery_attempt_count": max(0, recovery.attempt_index - 1),
                "review_failure_threshold": recovery.session_reset_threshold,
            },
            thread_id=thread_id,
        )

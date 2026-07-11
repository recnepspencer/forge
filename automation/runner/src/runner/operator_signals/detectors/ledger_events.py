from __future__ import annotations

from typing import Any

from runner.operator_signals.signal_types import (
    BLOCKER_SIGNAL, CRASH_SIGNAL, CanonicalSignal, IDLE_TIMEOUT_SIGNAL, INVALID_OUTCOME_SIGNAL,
    COMPLETION_HANDOFF_FAILED_SIGNAL, NO_EDIT_STALL_SIGNAL, RUN_COMPLETED_SIGNAL, SAME_PHASE_LOOP_SIGNAL, WALL_TIMEOUT_SIGNAL,
)


def signals_for_event(event: dict[str, Any]) -> tuple[CanonicalSignal, ...]:
    kind = signal_kind_for_event(event)
    if kind is None:
        return ()
    payload = event.get("payload", {})
    family = payload.get("failure_family") if isinstance(payload, dict) else None
    summary = payload.get("reason") if isinstance(payload, dict) else None
    return (CanonicalSignal(
        signal_id=f"{event['run_id']}:{event['sequence']}:{kind}", signal_kind=kind,
        source_sequence=event["sequence"], run_id=event["run_id"], phase_id=event.get("phase_id"),
        turn=event.get("turn"), summary=summary if isinstance(summary, str) else kind,
        details={"failure_family": family, "turn_instance_id": payload.get("turn_instance_id") if isinstance(payload, dict) else None},
    ),)


def signal_kind_for_event(event: dict[str, Any]) -> str | None:
    event_type, payload = event.get("event_type"), event.get("payload", {})
    if event_type == "run_completed": return RUN_COMPLETED_SIGNAL
    if event_type == "completion_handoff_failed": return COMPLETION_HANDOFF_FAILED_SIGNAL
    if event_type == "operator_pause": return BLOCKER_SIGNAL
    if event_type == "operator_override": return BLOCKER_SIGNAL
    if event_type != "runner_fault" or not isinstance(payload, dict): return None
    return {
        "provider_crash": CRASH_SIGNAL, "invalid_outcome": INVALID_OUTCOME_SIGNAL,
        "missing_runner_event": INVALID_OUTCOME_SIGNAL, "malformed_runner_event": INVALID_OUTCOME_SIGNAL,
        "wall_timeout": WALL_TIMEOUT_SIGNAL, "idle_timeout": IDLE_TIMEOUT_SIGNAL,
        "no_edit_stall": NO_EDIT_STALL_SIGNAL, "same_phase_loop_exceeded": SAME_PHASE_LOOP_SIGNAL,
    }.get(payload.get("failure_family"))

from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
from typing import Any

from runner.graph_runtime.runtime_lane import refresh_projection
from runner.recovery.failure_families import INVALID_OUTCOME_FAMILY, PROVIDER_CRASH_FAMILY


@dataclass(frozen=True)
class PendingRecovery:
    reason: str
    failure_family: str | None = None
    required_attempt_action: str | None = None
    session_reset_threshold: int | None = None
    session_reset_cycle_count: int | None = None


def pending_recovery_reason(
    events: list[dict[str, Any]],
    current: dict[str, Any],
    current_turn_instance_id: str | None,
) -> PendingRecovery | None:
    candidate: PendingRecovery | None = None
    for event in reversed(events):
        if event.get("phase_id") != current["phase"] or event.get("turn") != current["turn"]:
            continue
        event_turn_instance_id = event.get("payload", {}).get("turn_instance_id")
        if current_turn_instance_id is None and event_turn_instance_id is not None:
            continue
        if current_turn_instance_id is not None and event_turn_instance_id != current_turn_instance_id:
            continue
        event_type = event["event_type"]
        if event_type == "turn_outcome_recorded":
            return None
        if event_type == "prompt_selected":
            return candidate
        if event_type == "recovery_requested":
            if event.get("payload", {}).get("attempt_action") in {"notify", "notify_and_pause"}:
                return None
            candidate = pending_recovery_from_payload(
                event.get("payload", {}),
                fallback_reason="recovery requested",
            )
            continue
        if event_type == "runner_fault":
            candidate = pending_recovery_from_payload(
                event.get("payload", {}),
                fallback_reason="runner fault",
            )
            continue
        if event_type == "codex_turn_completed":
            candidate = PendingRecovery(
                reason="prior agent turn completed but outcome was not recorded",
                # A completed process with no outcome is precisely the missing-event
                # case.  It must receive the same-agent repair chance before the
                # broader invalid-outcome policy applies.
                failure_family="missing_runner_event",
            )
            continue
        if event_type == "codex_turn_failed":
            candidate = PendingRecovery(
                reason="prior agent turn failed and needs recovery",
                failure_family=PROVIDER_CRASH_FAMILY,
            )
    # Preflight failures have no preceding prompt-selected event.  Their
    # authoritative fault is still an admitted pending recovery.
    return candidate


def pending_recovery_from_payload(payload: dict[str, Any], fallback_reason: str) -> PendingRecovery:
    reason = payload.get("reason")
    failure_family = payload.get("failure_family")
    required_attempt_action = payload.get("required_attempt_action")
    reset_threshold = positive_int_or_none(payload.get("session_reset_threshold"))
    reset_cycle_count = positive_int_or_none(payload.get("session_reset_cycle_count"))
    return PendingRecovery(
        reason=reason if isinstance(reason, str) and reason else fallback_reason,
        failure_family=failure_family if isinstance(failure_family, str) and failure_family else None,
        required_attempt_action=(
            required_attempt_action
            if isinstance(required_attempt_action, str) and required_attempt_action
            else None
        ),
        session_reset_threshold=reset_threshold,
        session_reset_cycle_count=reset_cycle_count,
    )


def positive_int_or_none(value: object) -> int | None:
    return value if isinstance(value, int) and value > 0 else None


def turn_is_current(
    config_path: Path,
    run_id: str,
    current: dict[str, Any],
    turn_instance_id: str | None,
) -> bool:
    projection = refresh_projection(config_path, run_id)
    latest_current = projection.get("current")
    return (
        isinstance(latest_current, dict)
        and latest_current.get("phase") == current["phase"]
        and latest_current.get("turn") == current["turn"]
        and projection.get("current_turn_instance_id") == turn_instance_id
    )

from __future__ import annotations

INVALID_OUTCOME_FAMILY = "invalid_outcome"
MALFORMED_RUNNER_EVENT_FAMILY = "malformed_runner_event"
MISSING_RUNNER_EVENT_FAMILY = "missing_runner_event"
PROVIDER_CRASH_FAMILY = "provider_crash"
WALL_TIMEOUT_FAMILY = "wall_timeout"
IDLE_TIMEOUT_FAMILY = "idle_timeout"
SAME_PHASE_LOOP_EXCEEDED_FAMILY = "same_phase_loop_exceeded"
NO_EDIT_STALL_FAMILY = "no_edit_stall"

RECOVERY_FAILURE_FAMILIES = (
    PROVIDER_CRASH_FAMILY,
    WALL_TIMEOUT_FAMILY,
    IDLE_TIMEOUT_FAMILY,
    INVALID_OUTCOME_FAMILY,
    MISSING_RUNNER_EVENT_FAMILY,
    MALFORMED_RUNNER_EVENT_FAMILY,
    SAME_PHASE_LOOP_EXCEEDED_FAMILY,
    NO_EDIT_STALL_FAMILY,
)

# Compatibility aliases for older callers during the milestone cutover.
MALFORMED_OUTCOME_EVENT_FAMILY = "malformed_outcome_event"
MISSING_OUTCOME_EVENT_FAMILY = "missing_outcome_event"


def classify_pre_outcome_failure_family(
    capture: dict[str, object],
    *,
    default_family: str = PROVIDER_CRASH_FAMILY,
) -> str:
    captured_family = capture.get("failure_family")
    if isinstance(captured_family, str) and captured_family:
        return captured_family
    failure_reason = capture.get("failure_reason")
    if not isinstance(failure_reason, str):
        return default_family
    if failure_reason.startswith("turn timeout after "):
        return WALL_TIMEOUT_FAMILY
    if failure_reason.startswith("idle timeout after "):
        return IDLE_TIMEOUT_FAMILY
    return default_family

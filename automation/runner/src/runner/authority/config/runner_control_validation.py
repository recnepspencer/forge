from __future__ import annotations

from typing import Any


def validate_runner_control(runner_control: dict[str, Any], errors: list[str]) -> None:
    validate_optional_nonnegative_int(runner_control, "phase_id_start", errors)
    validate_optional_positive_int(runner_control, "stop_before_phase", errors)
    validate_optional_nonnegative_int(runner_control, "boundary_review_start_phase", errors)
    validate_optional_positive_int(runner_control, "turn_timeout_seconds", errors)
    validate_optional_positive_int(runner_control, "idle_timeout_seconds", errors)
    if "fresh_session_after_qa_repair_cycles" in runner_control:
        errors.append(
            "runner_control.fresh_session_after_qa_repair_cycles moved to session policy "
            "(session_defaults or phases[*].role_bindings.*.session_policy)"
        )

    stop_reason = runner_control.get("stop_reason")
    if stop_reason is not None and (not isinstance(stop_reason, str) or not stop_reason):
        errors.append("runner_control.stop_reason must be a non-empty string when present")

    validate_completion_handoff(runner_control.get("completion_handoff"), errors)


def validate_optional_positive_int(config: dict[str, Any], key: str, errors: list[str]) -> None:
    value = config.get(key)
    if value is not None and (not isinstance(value, int) or value <= 0):
        errors.append(f"runner_control.{key} must be a positive integer when present")


def validate_optional_nonnegative_int(config: dict[str, Any], key: str, errors: list[str]) -> None:
    value = config.get(key)
    if value is not None and (not isinstance(value, int) or value < 0):
        errors.append(f"runner_control.{key} must be a non-negative integer when present")


def validate_completion_handoff(value: Any, errors: list[str]) -> None:
    if value is None:
        return
    if not isinstance(value, dict):
        errors.append("runner_control.completion_handoff must be an object when present")
        return

    next_run_id = value.get("next_run_id")
    if not isinstance(next_run_id, str) or not next_run_id:
        errors.append("runner_control.completion_handoff.next_run_id is required")

    loop = value.get("loop")
    if loop is not None and not isinstance(loop, bool):
        errors.append("runner_control.completion_handoff.loop must be a boolean when present")

    sleep_seconds = value.get("sleep_seconds")
    if sleep_seconds is not None and (not isinstance(sleep_seconds, int) or sleep_seconds <= 0):
        errors.append("runner_control.completion_handoff.sleep_seconds must be a positive integer when present")

    log = value.get("log")
    if log is not None and (not isinstance(log, str) or not log):
        errors.append("runner_control.completion_handoff.log must be a non-empty string when present")

    reason = value.get("reason")
    if reason is not None and (not isinstance(reason, str) or not reason):
        errors.append("runner_control.completion_handoff.reason must be a non-empty string when present")

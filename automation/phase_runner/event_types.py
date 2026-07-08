from __future__ import annotations

from typing import Any

NOTE_BUCKETS = ("plan", "done", "remaining", "findings", "verification")

EVENT_TYPES = {
    "run_started",
    "run_resumed",
    "run_stopped",
    "run_completed",
    "turn_outcome_recorded",
    "prompt_selected",
    "codex_turn_completed",
    "codex_turn_failed",
    "boundary_review_completed",
    "plan_posted",
    "implementation_completed",
    "review_failed",
    "review_passed",
    "repair_completed",
    "test_review_failed",
    "test_review_passed",
    "test_repair_plan_posted",
    "test_repair_completed",
    "code_quality_review_failed",
    "code_quality_repair_completed",
    "code_quality_review_passed",
    "runner_fault",
    "recovery_requested",
    "recovery_completed",
    "operator_override",
    "legacy_imported",
}

PHASE_PROGRESS_EVENTS = {
    "boundary_review_completed",
    "plan_posted",
    "implementation_completed",
    "review_failed",
    "review_passed",
    "repair_completed",
    "test_review_failed",
    "test_review_passed",
    "test_repair_plan_posted",
    "test_repair_completed",
    "code_quality_review_failed",
    "code_quality_repair_completed",
    "code_quality_review_passed",
}


def validate_event_shape(event: dict[str, Any]) -> list[str]:
    errors: list[str] = []
    if event.get("event_type") not in EVENT_TYPES:
        errors.append(f"unknown event_type: {event.get('event_type')!r}")
    if not isinstance(event.get("run_id"), str) or not event.get("run_id"):
        errors.append("run_id must be a non-empty string")
    if not isinstance(event.get("sequence"), int):
        errors.append("sequence must be a number")
    if not isinstance(event.get("at"), str) or not event.get("at"):
        errors.append("at must be a non-empty string")
    if event.get("phase_id") is not None and not isinstance(event.get("phase_id"), int):
        errors.append("phase_id must be a number or null")
    if event.get("turn") is not None and not isinstance(event.get("turn"), str):
        errors.append("turn must be a string or null")
    if event.get("thread_id") is not None and not isinstance(event.get("thread_id"), str):
        errors.append("thread_id must be a string or null")
    payload = event.get("payload")
    if not isinstance(payload, dict):
        errors.append("payload must be an object")
    else:
        errors.extend(validate_payload(event.get("event_type"), payload))
    return errors


def validate_payload(event_type: str | None, payload: dict[str, Any]) -> list[str]:
    errors: list[str] = []
    errors.extend(validate_turn_instance_id(payload))
    if event_type in PHASE_PROGRESS_EVENTS:
        errors.extend(validate_note_updates(payload.get("notes", {})))
        summary = payload.get("summary")
        if summary is not None and not isinstance(summary, str):
            errors.append("phase progress payload.summary must be a string when present")
    if event_type == "test_repair_completed":
        next_turn = payload.get("next_turn")
        if next_turn not in {"test_review", "code_quality_review"}:
            errors.append(
                "test_repair_completed payload.next_turn must be test_review or code_quality_review"
            )
    if event_type == "operator_override":
        current = payload.get("current")
        if not isinstance(current, dict):
            errors.append("operator_override payload.current must be an object")
        else:
            if not isinstance(current.get("phase"), int):
                errors.append("operator_override payload.current.phase must be a number")
            if not isinstance(current.get("turn"), str):
                errors.append("operator_override payload.current.turn must be a string")
        reason = payload.get("reason")
        if not isinstance(reason, str) or not reason:
            errors.append("operator_override payload.reason is required")
    if event_type in {"runner_fault", "recovery_requested", "recovery_completed"}:
        reason = payload.get("reason")
        if reason is not None and not isinstance(reason, str):
            errors.append(f"{event_type} payload.reason must be a string when present")
    if event_type == "turn_outcome_recorded":
        outcome_event_type = payload.get("outcome_event_type")
        if not isinstance(outcome_event_type, str) or not outcome_event_type:
            errors.append("turn_outcome_recorded payload.outcome_event_type is required")
    return errors


def validate_runner_outcome(event_type: str, payload: dict[str, Any]) -> list[str]:
    errors: list[str] = []
    if event_type not in EVENT_TYPES:
        errors.append(f"unknown event_type: {event_type!r}")
        return errors
    errors.extend(validate_payload(event_type, payload))
    return errors


def validate_turn_instance_id(payload: dict[str, Any]) -> list[str]:
    turn_instance_id = payload.get("turn_instance_id")
    if turn_instance_id is None:
        return []
    if not isinstance(turn_instance_id, str) or not turn_instance_id:
        return ["payload.turn_instance_id must be a non-empty string when present"]
    return []


def validate_note_updates(notes: Any) -> list[str]:
    errors: list[str] = []
    if not isinstance(notes, dict):
        return ["payload.notes must be an object"]
    for key, value in notes.items():
        if key not in NOTE_BUCKETS:
            errors.append(f"payload.notes.{key} is not a supported note bucket")
            continue
        if not isinstance(value, list) or any(not isinstance(item, str) for item in value):
            errors.append(f"payload.notes.{key} must be a list of strings")
    return errors

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
    "single_prompt_completed",
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
    "session_reset",
    "operator_override",
    "legacy_imported",
}

PHASE_PROGRESS_EVENTS = {
    "boundary_review_completed",
    "plan_posted",
    "implementation_completed",
    "single_prompt_completed",
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
        for optional_key in ("injection_mode", "post_injection_route"):
            optional_value = payload.get(optional_key)
            if optional_value is not None and (not isinstance(optional_value, str) or not optional_value):
                errors.append(f"operator_override payload.{optional_key} must be a non-empty string when present")
    if event_type in {"runner_fault", "recovery_requested", "recovery_completed"}:
        reason = payload.get("reason")
        if reason is not None and not isinstance(reason, str):
            errors.append(f"{event_type} payload.reason must be a string when present")
        failure_family = payload.get("failure_family")
        if failure_family is not None and (not isinstance(failure_family, str) or not failure_family):
            errors.append(f"{event_type} payload.failure_family must be a non-empty string when present")
        if event_type == "recovery_requested":
            for key in ("recovery_kind", "attempt_action"):
                value = payload.get(key)
                if value is not None and (not isinstance(value, str) or not value):
                    errors.append(f"recovery_requested payload.{key} must be a non-empty string when present")
            attempt_index = payload.get("attempt_index")
            if attempt_index is not None and (not isinstance(attempt_index, int) or attempt_index <= 0):
                errors.append("recovery_requested payload.attempt_index must be a positive integer when present")
    if event_type == "session_reset":
        reason = payload.get("reason")
        if not isinstance(reason, str) or not reason:
            errors.append("session_reset payload.reason is required")
        threshold = payload.get("threshold")
        if not isinstance(threshold, int) or threshold <= 0:
            errors.append("session_reset payload.threshold must be a positive integer")
        cycle_count = payload.get("cycle_count")
        if not isinstance(cycle_count, int) or cycle_count <= 0:
            errors.append("session_reset payload.cycle_count must be a positive integer")
    if event_type == "turn_outcome_recorded":
        outcome_event_type = payload.get("outcome_event_type")
        if not isinstance(outcome_event_type, str) or not outcome_event_type:
            errors.append("turn_outcome_recorded payload.outcome_event_type is required")
    if event_type == "prompt_selected":
        errors.extend(validate_prompt_selected_binding(payload))
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


def validate_prompt_selected_binding(payload: dict[str, Any]) -> list[str]:
    binding_fields_present = any(
        key in payload for key in ("contract_asset_id", "prompt_asset_id", "prompt_assembly_id")
    )
    witness_fields_present = any(
        key in payload
        for key in (
            "contract_root_kind",
            "contract_source_path",
            "prompt_asset_root_kind",
            "prompt_asset_source_path",
            "prompt_assembly_root_kind",
            "prompt_assembly_source_path",
        )
    )
    if not binding_fields_present and not witness_fields_present:
        return []
    errors: list[str] = []
    contract_asset_id = payload.get("contract_asset_id")
    if not isinstance(contract_asset_id, str) or not contract_asset_id:
        errors.append("prompt_selected payload.contract_asset_id must be a non-empty string when binding is present")
    if witness_fields_present:
        errors.extend(validate_prompt_selected_source_field(payload, "contract_root_kind"))
        errors.extend(validate_prompt_selected_source_field(payload, "contract_source_path"))
    prompt_asset_id = payload.get("prompt_asset_id")
    prompt_assembly_id = payload.get("prompt_assembly_id")
    has_prompt_asset = isinstance(prompt_asset_id, str) and bool(prompt_asset_id)
    has_prompt_assembly = isinstance(prompt_assembly_id, str) and bool(prompt_assembly_id)
    if bool(prompt_asset_id) and not has_prompt_asset:
        errors.append("prompt_selected payload.prompt_asset_id must be a non-empty string when present")
    if bool(prompt_assembly_id) and not has_prompt_assembly:
        errors.append("prompt_selected payload.prompt_assembly_id must be a non-empty string when present")
    if has_prompt_asset == has_prompt_assembly:
        errors.append("prompt_selected payload must carry exactly one of prompt_asset_id or prompt_assembly_id")
    if witness_fields_present and has_prompt_asset:
        errors.extend(validate_prompt_selected_source_field(payload, "prompt_asset_root_kind"))
        errors.extend(validate_prompt_selected_source_field(payload, "prompt_asset_source_path"))
    if witness_fields_present and has_prompt_assembly:
        errors.extend(validate_prompt_selected_source_field(payload, "prompt_assembly_root_kind"))
        errors.extend(validate_prompt_selected_source_field(payload, "prompt_assembly_source_path"))
    return errors


def validate_prompt_selected_source_field(payload: dict[str, Any], key: str) -> list[str]:
    value = payload.get(key)
    if not isinstance(value, str) or not value:
        return [f"prompt_selected payload.{key} must be a non-empty string when witness is present"]
    return []

from __future__ import annotations

from pathlib import Path
from typing import Any

from runner.authority.run_identity import RuntimePaths
from runner.facade.plan_revision import (
    completed_phase_keys_from_events,
    latest_plan_payload,
    stable_phase_key,
)
from runner.prompt_library.bindings.reference_validation import (
    validate_asset_binding_reference,
    validate_assembly_binding_reference,
)


def record_prompt_override(
    run_id: str,
    phase_key: str,
    binding: dict[str, str],
    *,
    turn: str | None,
    reason: str,
) -> None:
    from runner.facade.runtime_state import append_runtime_event, refresh_projection_for_run

    _, config, events = load_admitted_inputs_for_plan_mutation(run_id)
    phase = phase_for_key(config, phase_key)
    assert_phase_is_open(events, phase_key)
    validate_override_turn(config, phase, turn)
    validate_prompt_override_binding(config, binding)
    payload = {"phase_key": phase_key, "binding": binding, "reason": reason}
    if turn:
        payload["turn"] = turn
    append_runtime_event(RuntimePaths(run_id), "operator_prompt_override", payload=payload)
    refresh_projection_for_run(run_id)


def record_external_completion(
    run_id: str,
    phase_key: str,
    agent: str,
    summary: str,
    evidence: list[str],
) -> None:
    from runner.facade.runtime_state import append_runtime_event, refresh_projection_for_run

    _, config, events = load_admitted_inputs_for_plan_mutation(run_id)
    phase_for_key(config, phase_key)
    assert_phase_is_open(events, phase_key)
    if not evidence:
        raise ValueError("external phase completion requires at least one evidence item")
    append_runtime_event(
        RuntimePaths(run_id),
        "external_phase_completed",
        payload={"phase_key": phase_key, "agent": agent, "summary": summary, "evidence": evidence},
    )
    refresh_projection_for_run(run_id)


def load_admitted_inputs_for_plan_mutation(run_id: str) -> tuple[Path, dict[str, Any], tuple[dict[str, Any], ...]]:
    from runner.authority.events.run_authority import load_admitted_run_projection_inputs

    return load_admitted_run_projection_inputs(run_id)


def phase_for_key(config: dict[str, Any], phase_key: str) -> dict[str, Any]:
    for phase in config.get("phases", []):
        if stable_phase_key(phase) == phase_key:
            return phase
    raise ValueError(f"phase_key {phase_key!r} is not present in the admitted plan")


def assert_phase_is_open(events: tuple[dict[str, Any], ...], phase_key: str) -> None:
    plan = latest_plan_payload(list(events))
    fingerprints = plan.get("phase_fingerprints", []) if isinstance(plan, dict) else []
    completed = completed_phase_keys_from_events(list(events), fingerprints)
    if phase_key in completed:
        raise ValueError(f"phase_key {phase_key!r} is already complete")


def validate_override_turn(config: dict[str, Any], phase: dict[str, Any], turn: str | None) -> None:
    if turn is None:
        return
    from runner.phase_programs import lower_phase_program

    if not lower_phase_program(config, phase).supports_turn(turn):
        raise ValueError(f"phase_key {stable_phase_key(phase)!r} does not support turn {turn!r}")


def validate_prompt_override_binding(config: dict[str, Any], binding: dict[str, str]) -> None:
    errors: list[str] = []
    if "asset_id" in binding:
        validate_asset_binding_reference(config, binding, "operator_prompt_override.binding", errors)
    elif "assembly_id" in binding:
        validate_assembly_binding_reference(config, binding, "operator_prompt_override.binding", errors)
    else:
        errors.append("operator_prompt_override.binding must have asset_id or assembly_id")
    if errors:
        raise ValueError("; ".join(errors))

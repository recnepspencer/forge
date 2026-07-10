from __future__ import annotations

from typing import Any

from runner.authority.config.schema import SINGLE_PROMPT_PHASE_KEYS, STANDARD_PHASE_KEYS
from runner.authority.events.event_types import EVENT_TYPES
from runner.phase_programs import lower_phase_program, phase_program_id
from runner.phase_programs.registry import PROGRAMS
from runner.phase_programs.program_ids import SINGLE_PROMPT_PROGRAM_ID
from runner.prompt_library.bindings.reference_validation import (
    validate_asset_binding_reference,
    validate_assembly_binding_reference,
)
from runner.roles.model_policy import validate_model_policy_seed, validate_role_model_policy_binding
from runner.roles.role_ids import SUPPORTED_ROLE_IDS
from runner.roles.session_policy import validate_session_policy_seed


def validate_phases(
    config: dict[str, Any],
    phases: list[dict[str, Any]],
    runner_control: dict[str, Any] | None,
    errors: list[str],
) -> None:
    phase_ids: set[int] = set()
    phase_keys: set[str] = set()
    for index, phase in enumerate(phases):
        prefix = f"phases[{index}]"
        if not isinstance(phase, dict):
            errors.append(f"{prefix} must be an object")
            continue

        phase_id = phase.get("id")
        if not isinstance(phase_id, int) or phase_id < 0:
            errors.append(f"{prefix}.id must be a non-negative integer")
            continue
        if phase_id in phase_ids:
            errors.append(f"{prefix}.id duplicates phase id {phase_id}")
            continue

        phase_ids.add(phase_id)
        phase_key = phase.get("phase_key")
        if phase_key is not None and (not isinstance(phase_key, str) or not phase_key):
            errors.append(f"{prefix}.phase_key must be a non-empty string when present")
        admitted_phase_key = phase_key if isinstance(phase_key, str) and phase_key else f"phase_{phase_id}"
        if admitted_phase_key in phase_keys:
            errors.append(f"{prefix}.phase_key duplicates phase key {admitted_phase_key!r}")
        phase_keys.add(admitted_phase_key)
        validate_phase_body(prefix, config, phase, errors)

    configured_start = runner_control.get("phase_id_start") if isinstance(runner_control, dict) else None
    if isinstance(configured_start, int) and phases:
        first_phase = phases[0]
        if isinstance(first_phase, dict) and first_phase.get("id") != configured_start:
            errors.append("runner_control.phase_id_start must match the first configured phase id")


def validate_phase_body(prefix: str, config: dict[str, Any], phase: dict[str, Any], errors: list[str]) -> None:
    for key in ("title", "owner", "instructions", "qa_focus"):
        if not isinstance(phase.get(key), str) or not phase.get(key):
            errors.append(f"{prefix}.{key} is required")

    for key in ("scope", "acceptance"):
        if not isinstance(phase.get(key), list) or not phase.get(key):
            errors.append(f"{prefix}.{key} must be a non-empty list")

    try:
        program_id = phase_program_id(phase)
        lowered_program = lower_phase_program(config, phase)
    except ValueError as error:
        errors.append(f"{prefix}.{error}")
        return

    validate_phase_keys(prefix, phase, program_id, errors)
    validate_phase_templates(prefix, config, phase, program_id, lowered_program, errors)
    validate_role_bindings(prefix, config, phase, lowered_program, errors)


def validate_phase_keys(prefix: str, phase: dict[str, Any], program_id: str, errors: list[str]) -> None:
    allowed = SINGLE_PROMPT_PHASE_KEYS if program_id == SINGLE_PROMPT_PROGRAM_ID else STANDARD_PHASE_KEYS
    unknown = set(phase.keys()) - allowed
    if unknown:
        errors.append(f"{prefix} has unknown keys for {program_id}: {sorted(unknown)}")


def validate_phase_templates(
    prefix: str,
    config: dict[str, Any],
    phase: dict[str, Any],
    program_id: str,
    lowered_program,
    errors: list[str],
) -> None:
    phase_contract = phase.get("contract_template")
    if phase_contract is not None:
        validate_asset_binding_reference(config, phase_contract, f"{prefix}.contract_template", errors)

    if program_id != SINGLE_PROMPT_PROGRAM_ID:
        return

    prompt_template = phase.get("prompt_template")
    if prompt_template is None:
        errors.append(f"{prefix}.prompt_template is required for {lowered_program.program_id} phases")
    else:
        validate_asset_binding_reference(config, prompt_template, f"{prefix}.prompt_template", errors)

    success_event_type = phase.get("success_event_type")
    if not isinstance(success_event_type, str) or not success_event_type:
        errors.append(f"{prefix}.success_event_type is required for {lowered_program.program_id} phases")
    elif success_event_type not in EVENT_TYPES:
        errors.append(f"{prefix}.success_event_type must be a known runner event type")
    elif success_event_type not in lowered_program.supported_outcomes_for_turn("single_prompt"):
        errors.append(
            f"{prefix}.success_event_type must be one of {sorted(lowered_program.supported_outcomes_for_turn('single_prompt'))}"
        )


def validate_role_bindings(
    prefix: str,
    config: dict[str, Any],
    phase: dict[str, Any],
    lowered_program,
    errors: list[str],
) -> None:
    bindings = phase.get("role_bindings")
    if not isinstance(bindings, dict) or not bindings:
        errors.append(f"{prefix}.role_bindings must be a non-empty object")
        return

    required_turns = lowered_program.role_binding_turns
    missing_turns = [turn for turn in required_turns if turn not in bindings]
    if missing_turns:
        errors.append(f"{prefix}.role_bindings must define {missing_turns}")

    admitted_turns = set(PROGRAMS[lowered_program.program_id].role_binding_turns)
    unknown_turns = sorted(set(bindings.keys()) - admitted_turns)
    if unknown_turns:
        errors.append(f"{prefix}.role_bindings has unknown turns: {unknown_turns}")

    for turn in required_turns:
        validate_role_binding(
            f"{prefix}.role_bindings.{turn}",
            config,
            bindings.get(turn),
            errors,
        )


def validate_role_binding(
    prefix: str,
    config: dict[str, Any],
    binding: Any,
    errors: list[str],
) -> None:
    if not isinstance(binding, dict):
        errors.append(f"{prefix} must be an object")
        return

    allowed = {"role_id", "model_policy", "session_policy", "prompt_template"}
    unknown = set(binding.keys()) - allowed
    if unknown:
        errors.append(f"{prefix} has unknown keys: {sorted(unknown)}")

    role_id = binding.get("role_id")
    if role_id not in SUPPORTED_ROLE_IDS:
        errors.append(f"{prefix}.role_id must be one of {list(SUPPORTED_ROLE_IDS)}")

    session_policy = binding.get("session_policy")
    if not isinstance(session_policy, dict):
        errors.append(f"{prefix}.session_policy must be an object")
    else:
        validate_session_policy_seed(session_policy, errors, f"{prefix}.session_policy")

    model_policy = binding.get("model_policy")
    if not isinstance(model_policy, dict):
        errors.append(f"{prefix}.model_policy must be an object")
    else:
        validate_role_model_policy_binding(model_policy, errors, f"{prefix}.model_policy")
        merged_seed = dict(config.get("session_defaults", {}))
        merged_seed.update(model_policy)
        validate_model_policy_seed(merged_seed, errors, f"{prefix}.model_policy")

    prompt_template = binding.get("prompt_template")
    if prompt_template is not None:
        validate_assembly_binding_reference(config, prompt_template, f"{prefix}.prompt_template", errors)

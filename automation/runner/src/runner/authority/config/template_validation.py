from __future__ import annotations

from typing import Any

from runner.phase_programs import lower_phase_program
from runner.prompt_library.bindings.reference_validation import (
    validate_asset_binding_reference,
    validate_assembly_binding_reference,
)


def validate_turn_templates(
    config: dict[str, Any],
    templates: dict[str, Any],
    errors: list[str],
) -> None:
    required_turns = required_turn_templates_for_config(config)
    for turn in required_turns:
        validate_assembly_binding_reference(config, templates.get(turn), f"turn_templates.{turn}", errors)

    for turn, template_reference in templates.items():
        if turn in required_turns:
            continue
        if template_reference is None:
            continue
        validate_assembly_binding_reference(config, template_reference, f"turn_templates.{turn}", errors)


def validate_contract_template(config: dict[str, Any], errors: list[str]) -> None:
    contract_template = config.get("contract_template")
    if contract_template is None:
        errors.append("contract_template is required")
        return
    validate_asset_binding_reference(config, contract_template, "contract_template", errors)


def required_turn_templates_for_config(config: dict[str, Any]) -> tuple[str, ...]:
    phases = config.get("phases")
    if not isinstance(phases, list):
        return ()

    required: list[str] = []
    for phase in phases:
        if not isinstance(phase, dict):
            continue
        try:
            lowered_program = lower_phase_program(config, phase)
        except ValueError:
            continue
        _append_missing(required, lowered_program.required_turn_template_turns)
    return tuple(required)


def _append_missing(required: list[str], turns: tuple[str, ...]) -> None:
    for turn in turns:
        if turn not in required:
            required.append(turn)

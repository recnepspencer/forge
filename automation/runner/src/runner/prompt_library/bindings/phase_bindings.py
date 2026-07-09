from __future__ import annotations

from typing import Any

from runner.phase_programs import lower_phase_program
from runner.phase_programs.lowered_program import PHASE_ASSET_PROMPT_BINDING
from runner.prompt_library.bindings.binding_references import (
    AssetBindingReference,
    AssemblyBindingReference,
    parse_asset_binding_reference,
    parse_assembly_binding_reference,
)


def contract_binding_for_phase(config: dict[str, Any], phase: dict[str, Any]) -> AssetBindingReference:
    value = phase.get("contract_template", config.get("contract_template"))
    return parse_asset_binding_reference(value, f"phase {phase.get('id')} contract binding")


def prompt_binding_for_cursor(
    config: dict[str, Any],
    phase: dict[str, Any],
    turn: str,
) -> AssetBindingReference | AssemblyBindingReference:
    lowered_program = lower_phase_program(config, phase)
    if lowered_program.prompt_binding_mode_for_turn(turn) == PHASE_ASSET_PROMPT_BINDING:
        return parse_asset_binding_reference(
            phase.get("prompt_template"),
            f"phase {phase.get('id')} {lowered_program.program_id} binding",
        )
    value = config.get("turn_templates", {}).get(turn)
    return parse_assembly_binding_reference(value, f"turn_templates.{turn}")

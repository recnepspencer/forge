from __future__ import annotations

from typing import Any

from runner.phase_programs.implement_review_loop import PROGRAM_DEFINITION as IMPLEMENT_REVIEW_LOOP_PROGRAM
from runner.phase_programs.lowered_program import LoweredPhaseProgram
from runner.phase_programs.program_ids import (
    IMPLEMENT_REVIEW_LOOP_PROGRAM_ID,
    SINGLE_PROMPT_PROGRAM_ID,
    STANDARD_LOOP_PROGRAM_ID,
    STANDARD_SINGLE_PASS_FOLLOWUPS_PROGRAM_ID,
    SUPPORTED_PROGRAM_IDS,
)
from runner.phase_programs.single_prompt import PROGRAM_DEFINITION as SINGLE_PROMPT_PROGRAM
from runner.phase_programs.standard_loop import PROGRAM_DEFINITION as STANDARD_LOOP_PROGRAM
from runner.phase_programs.standard_single_pass_followups import PROGRAM_DEFINITION as STANDARD_SINGLE_PASS_FOLLOWUPS_PROGRAM

PROGRAMS = {
    STANDARD_LOOP_PROGRAM_ID: STANDARD_LOOP_PROGRAM,
    IMPLEMENT_REVIEW_LOOP_PROGRAM_ID: IMPLEMENT_REVIEW_LOOP_PROGRAM,
    STANDARD_SINGLE_PASS_FOLLOWUPS_PROGRAM_ID: STANDARD_SINGLE_PASS_FOLLOWUPS_PROGRAM,
    SINGLE_PROMPT_PROGRAM_ID: SINGLE_PROMPT_PROGRAM,
}


def phase_program_id(phase: dict[str, Any]) -> str:
    program_id = phase.get("program_id")
    if not isinstance(program_id, str) or not program_id:
        raise ValueError(f"phase {phase.get('id')} is missing program_id")
    if program_id not in SUPPORTED_PROGRAM_IDS:
        raise ValueError(f"phase {phase.get('id')} program_id must be one of {sorted(SUPPORTED_PROGRAM_IDS)}")
    return program_id


def lower_phase_program(config: dict[str, Any], phase: dict[str, Any]) -> LoweredPhaseProgram:
    program = PROGRAMS[phase_program_id(phase)]
    if not boundary_review_enabled_for_phase(config, phase) and program.first_turn == "boundary_review":
        return program.without_turn("boundary_review", first_turn="plan")
    return program


def boundary_review_enabled_for_phase(config: dict[str, Any], phase: dict[str, Any]) -> bool:
    turn_templates = config.get("turn_templates", {})
    if "boundary_review" not in turn_templates:
        return False
    runner_control = config.get("runner_control", {})
    threshold = runner_control.get("boundary_review_start_phase")
    phase_id = phase.get("id")
    if threshold is None:
        return "phase_id_start" not in runner_control
    return isinstance(threshold, int) and isinstance(phase_id, int) and phase_id >= threshold

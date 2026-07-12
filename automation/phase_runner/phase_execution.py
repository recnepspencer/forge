from __future__ import annotations

from pathlib import Path
from typing import Any

STANDARD_LOOP_MODE = "standard_loop"
SINGLE_PROMPT_MODE = "single_prompt"
SUPPORTED_EXECUTION_MODES = {STANDARD_LOOP_MODE, SINGLE_PROMPT_MODE}
SINGLE_PROMPT_SUCCESS_EVENT_TYPES = {"single_prompt_completed"}

STANDARD_REQUIRED_TURNS = (
    "plan",
    "implement",
    "review",
    "repair_plan",
    "repair",
    "test_review",
    "test_repair_plan",
    "test_repair_implement",
    "code_quality_review",
    "code_quality_repair",
)

STANDARD_OPTIONAL_TURNS = (
    "boundary_review",
)

SINGLE_PROMPT_TURN = "single_prompt"


def execution_mode_for_phase(phase: dict[str, Any]) -> str:
    mode = phase.get("execution_mode")
    if mode in SUPPORTED_EXECUTION_MODES:
        return str(mode)
    return STANDARD_LOOP_MODE


def initial_turn_for_phase(
    phase: dict[str, Any],
    turn_templates: dict[str, Any],
) -> str:
    mode = execution_mode_for_phase(phase)
    if mode == SINGLE_PROMPT_MODE:
        return SINGLE_PROMPT_TURN
    return "boundary_review" if "boundary_review" in turn_templates else "plan"


def prompt_template_path_for_cursor(
    config: dict[str, Any],
    config_path: Path,
    phase: dict[str, Any],
    turn: str,
) -> Path:
    mode = execution_mode_for_phase(phase)
    if mode == SINGLE_PROMPT_MODE:
        template_name = phase.get("prompt_template")
        if not isinstance(template_name, str) or not template_name:
            raise ValueError(f"phase {phase.get('id')} is missing prompt_template")
        return resolve_template_path(config_path, template_name)

    template_name = config["turn_templates"][turn]
    return resolve_template_path(config_path, template_name)


def contract_template_path_for_phase(
    config: dict[str, Any],
    config_path: Path,
    phase: dict[str, Any],
) -> Path:
    template_name = phase.get("contract_template", config["contract_template"])
    if not isinstance(template_name, str) or not template_name:
        raise ValueError(f"phase {phase.get('id')} is missing contract_template")
    return resolve_template_path(config_path, template_name)


def supported_outcome_event_types_for_cursor(phase: dict[str, Any], turn: str) -> set[str]:
    mode = execution_mode_for_phase(phase)
    if mode == SINGLE_PROMPT_MODE:
        if turn != SINGLE_PROMPT_TURN:
            raise ValueError(f"single_prompt phase cannot use turn {turn!r}")
        event_type = phase.get("success_event_type")
        if not isinstance(event_type, str) or not event_type:
            raise ValueError(f"phase {phase.get('id')} is missing success_event_type")
        return {event_type}

    from transition_rules import STANDARD_TURN_OUTCOME_EVENTS

    allowed = STANDARD_TURN_OUTCOME_EVENTS.get(turn)
    if allowed is None:
        raise ValueError(f"turn {turn!r} does not support runner outcomes")
    return allowed


def single_prompt_success_event_for_phase(phase: dict[str, Any]) -> str:
    event_type = phase.get("success_event_type")
    if not isinstance(event_type, str) or not event_type:
        raise ValueError(f"phase {phase.get('id')} is missing success_event_type")
    return event_type


def resolve_template_path(config_path: Path, value: str) -> Path:
    path = Path(value)
    if path.is_absolute():
        return path
    return config_path.parent.parent / path

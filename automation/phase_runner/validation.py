from __future__ import annotations

from pathlib import Path
from typing import Any

from state import PHASE_STATUSES, QA_STATUSES
from state_normalization import REQUIRED_NOTE_LISTS
from turn_state import TURN_TRANSITIONS


def validate_state(state: dict[str, Any], state_path: Path) -> list[str]:
    errors: list[str] = []
    require_mapping(state, "project", errors)
    require_mapping(state, "session", errors)
    require_mapping(state, "turn_templates", errors)
    if state.get("current") is not None:
        require_mapping(state, "current", errors)

    phases = state.get("phases")
    seen: set[str] = set()
    if not isinstance(phases, list) or not phases:
        errors.append("phases must be a non-empty list")
    else:
        for index, phase in enumerate(phases):
            prefix = f"phases[{index}]"
            validate_phase(phase, prefix, seen, errors)
        validate_current_cursor(state, phases, seen, errors)

    validate_templates(state, state_path, errors)
    validate_project(state, errors)
    validate_session(state, errors)
    return errors


def validate_phase(phase: Any, prefix: str, seen: set[str], errors: list[str]) -> None:
    if not isinstance(phase, dict):
        errors.append(f"{prefix} must be an object")
        return
    phase_id = phase.get("id")
    if not isinstance(phase_id, int):
        errors.append(f"{prefix}.id must be a number")
        return
    key = str(phase_id)
    if key in seen:
        errors.append(f"duplicate phase id {phase_id!r}")
    seen.add(key)
    if phase.get("status") not in PHASE_STATUSES:
        errors.append(f"{prefix}.status is invalid: {phase.get('status')!r}")
    if phase.get("qa_status") not in QA_STATUSES:
        errors.append(f"{prefix}.qa_status is invalid: {phase.get('qa_status')!r}")
    for field in ("title", "owner", "instructions", "qa_focus"):
        if not isinstance(phase.get(field), str) or not phase.get(field):
            errors.append(f"{prefix}.{field} is required")
    for field in ("scope", "acceptance"):
        if not isinstance(phase.get(field), list) or not phase.get(field):
            errors.append(f"{prefix}.{field} must be a non-empty list")
    validate_phase_notes(phase, prefix, errors)


def validate_current_cursor(
    state: dict[str, Any],
    phases: list[dict[str, Any]],
    seen: set[str],
    errors: list[str],
) -> None:
    current = state.get("current")
    if current is None:
        return
    if not isinstance(current, dict):
        return
    current_phase = current.get("phase")
    current_turn = current.get("turn")
    if current_phase is not None and str(current_phase) not in seen:
        errors.append(f"current.phase {current_phase!r} does not match a phase id")
    if current_turn is not None and current_turn not in state.get("turn_templates", {}):
        errors.append(f"current.turn {current_turn!r} has no template")
    if isinstance(current_phase, int):
        phase = next((entry for entry in phases if entry.get("id") == current_phase), None)
        if phase is not None and phase.get("status") == "complete" and phase.get("qa_status") == "passed":
            errors.append("complete/passed phase cannot keep an active current cursor")


def validate_templates(state: dict[str, Any], state_path: Path, errors: list[str]) -> None:
    templates = state.get("turn_templates", {})
    if not isinstance(templates, dict):
        return
    for key in ("plan", "implement", "review", "repair"):
        value = templates.get(key)
        if not isinstance(value, str) or not value:
            errors.append(f"turn_templates.{key} must name a template file")
    for key, value in templates.items():
        if key not in TURN_TRANSITIONS:
            errors.append(f"turn_templates.{key} is not part of the supported turn graph")
        if not isinstance(value, str) or not value:
            errors.append(f"turn_templates.{key} must name a template file")
            continue
        template_path = resolve_config_path(state_path, value)
        if not template_path.exists():
            errors.append(f"template not found for {key}: {template_path}")
    contract_template = state.get("contract_template") or "templates/_contract.md"
    contract_path = resolve_config_path(state_path, contract_template)
    if not contract_path.exists():
        errors.append(f"contract template not found: {contract_path}")


def validate_project(state: dict[str, Any], errors: list[str]) -> None:
    project = state.get("project", {})
    if not isinstance(project, dict):
        return
    cwd = project.get("cwd")
    if isinstance(cwd, str) and cwd and not Path(cwd).exists():
        errors.append(f"project.cwd does not exist: {cwd}")
    spec_file = project.get("spec_file")
    if isinstance(spec_file, str) and spec_file:
        spec_path = Path(cwd, spec_file) if isinstance(cwd, str) else Path(spec_file)
        if not spec_path.exists():
            errors.append(f"project.spec_file does not exist: {spec_path}")
    for context_file in project.get("context_files", []):
        context_path = Path(cwd, context_file) if isinstance(cwd, str) else Path(context_file)
        if not context_path.exists():
            errors.append(f"project.context_files entry does not exist: {context_path}")


def validate_session(state: dict[str, Any], errors: list[str]) -> None:
    session = state.get("session", {})
    if not isinstance(session, dict):
        return
    fast_mode = session.get("fast_mode")
    if fast_mode is not None and not isinstance(fast_mode, bool):
        errors.append("session.fast_mode must be a boolean when present")
    service_tier = session.get("service_tier")
    if service_tier is not None and (
        not isinstance(service_tier, str) or not service_tier.strip()
    ):
        errors.append("session.service_tier must be a non-empty string when present")


def require_mapping(state: dict[str, Any], key: str, errors: list[str]) -> None:
    if not isinstance(state.get(key), dict):
        errors.append(f"{key} must be an object")


def validate_phase_notes(phase: dict[str, Any], prefix: str, errors: list[str]) -> None:
    notes = phase.get("notes")
    if not isinstance(notes, dict):
        errors.append(f"{prefix}.notes must be an object")
        return
    for key in REQUIRED_NOTE_LISTS:
        if not isinstance(notes.get(key), list):
            errors.append(f"{prefix}.notes.{key} must be a list")


def resolve_config_path(state_path: Path, value: str) -> Path:
    path = Path(value)
    if path.is_absolute():
        return path
    return state_path.parent / path

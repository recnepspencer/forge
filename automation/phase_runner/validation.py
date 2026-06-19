from __future__ import annotations

from pathlib import Path
from typing import Any

from state import PHASE_STATUSES, QA_STATUSES

REQUIRED_NOTE_LISTS = ("plan", "done", "remaining", "findings", "verification")


def validate_state(state: dict[str, Any], state_path: Path) -> list[str]:
    errors: list[str] = []
    require_mapping(state, "project", errors)
    require_mapping(state, "session", errors)
    require_mapping(state, "turn_templates", errors)
    if state.get("current") is not None:
        require_mapping(state, "current", errors)

    phases = state.get("phases")
    if not isinstance(phases, list) or not phases:
        errors.append("phases must be a non-empty list")
    else:
        seen: set[str] = set()
        for index, phase in enumerate(phases):
            prefix = f"phases[{index}]"
            if not isinstance(phase, dict):
                errors.append(f"{prefix} must be an object")
                continue
            phase_id = phase.get("id")
            if not isinstance(phase_id, int):
                errors.append(f"{prefix}.id must be a number")
                continue
            phase_key = str(phase_id)
            if phase_key in seen:
                errors.append(f"duplicate phase id {phase_id!r}")
            seen.add(phase_key)
            if phase.get("status") not in PHASE_STATUSES:
                errors.append(f"{prefix}.status is invalid: {phase.get('status')!r}")
            if phase.get("qa_status") not in QA_STATUSES:
                errors.append(f"{prefix}.qa_status is invalid: {phase.get('qa_status')!r}")
            for key in ("title", "owner", "instructions", "qa_focus"):
                if not isinstance(phase.get(key), str) or not phase.get(key):
                    errors.append(f"{prefix}.{key} is required")
            for key in ("scope", "acceptance"):
                if not isinstance(phase.get(key), list) or not phase.get(key):
                    errors.append(f"{prefix}.{key} must be a non-empty list")
            validate_phase_notes(phase, prefix, errors)

        current = state.get("current", {})
        if isinstance(current, dict):
            current_phase = current.get("phase")
            if current_phase is not None and str(current_phase) not in seen:
                errors.append(f"current.phase {current_phase!r} does not match a phase id")
            current_turn = current.get("turn")
            if current_turn is not None and current_turn not in state.get("turn_templates", {}):
                errors.append(f"current.turn {current_turn!r} has no template")

    templates = state.get("turn_templates", {})
    if isinstance(templates, dict):
        for key in ("plan", "implement", "review", "repair", "close"):
            value = templates.get(key)
            if not isinstance(value, str) or not value:
                errors.append(f"turn_templates.{key} must name a template file")
            else:
                template_path = resolve_config_path(state_path, value)
                if not template_path.exists():
                    errors.append(f"template not found for {key}: {template_path}")

    contract_template = state.get("contract_template", "templates/_contract.md")
    contract_path = resolve_config_path(state_path, contract_template)
    if not contract_path.exists():
        errors.append(f"contract template not found: {contract_path}")

    project = state.get("project", {})
    if isinstance(project, dict):
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

    return errors


def require_mapping(state: dict[str, Any], key: str, errors: list[str]) -> None:
    if not isinstance(state.get(key), dict):
        errors.append(f"{key} must be an object")


def validate_phase_notes(
    phase: dict[str, Any], prefix: str, errors: list[str]
) -> None:
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

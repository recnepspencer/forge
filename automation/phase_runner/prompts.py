from __future__ import annotations

import re
from pathlib import Path
from typing import Any

from state import current_phase, current_turn
from validation import resolve_config_path

TOKEN = re.compile(r"{([A-Za-z0-9_.]+)}")
DEFAULT_CONTRACT_TEMPLATE = "templates/_contract.md"


def render_prompt(state: dict[str, Any], state_path: Path) -> str:
    phase = current_phase_required(state)
    turn = current_turn_required(state)
    context = build_context(state, state_path, phase, turn)
    context["contract"] = render_contract(state, state_path, context)
    template_path = resolve_config_path(state_path, state["turn_templates"][turn])
    template = template_path.read_text(encoding="utf-8")
    return render_template(template, context)


def render_contract(
    state: dict[str, Any], state_path: Path, context: dict[str, Any]
) -> str:
    relative = state.get("contract_template") or DEFAULT_CONTRACT_TEMPLATE
    contract_path = resolve_config_path(state_path, relative)
    contract = contract_path.read_text(encoding="utf-8")
    return render_template(contract, context)


def build_context(
    state: dict[str, Any],
    state_path: Path,
    phase: dict[str, Any],
    turn_kind: str,
) -> dict[str, Any]:
    project = state.get("project", {})
    cwd = Path(project.get("cwd", ".")).resolve()
    spec_file = project.get("spec_file", "")
    spec_path = Path(spec_file)
    if not spec_path.is_absolute():
        spec_path = cwd / spec_path

    return {
        "project": project,
        "phase": phase,
        "session": state.get("session", {}),
        "state_file": str(state_path.resolve()),
        "spec_file": str(spec_path.resolve()),
        "turn": turn_kind,
        "current": state.get("current", {}),
        "turns": ", ".join(state.get("turn_templates", {}).keys()),
        "status_values": "not_started, in_progress, complete, regressed, blocked",
        "qa_status_values": "not_started, needed, in_progress, passed, failed",
    }


def render_template(template: str, context: dict[str, Any]) -> str:
    def replace(match: re.Match[str]) -> str:
        return stringify(resolve_token(context, match.group(1)))

    return TOKEN.sub(replace, template)


def resolve_token(context: dict[str, Any], token: str) -> Any:
    value: Any = context
    for part in token.split("."):
        if isinstance(value, dict) and part in value:
            value = value[part]
        else:
            raise KeyError(f"template variable {{{token}}} is not defined")
    return value


def stringify(value: Any) -> str:
    if value is None:
        return ""
    if isinstance(value, list):
        if not value:
            return "- none"
        return "\n".join(f"- {stringify(item)}" for item in value)
    if isinstance(value, dict):
        if not value:
            return "- none"
        return "\n".join(f"- {key}: {stringify(item)}" for key, item in value.items())
    return str(value)


def current_phase_required(state: dict[str, Any]) -> dict[str, Any]:
    phase = current_phase(state)
    if phase is not None:
        return phase
    raise ValueError("current phase is not set")


def current_turn_required(state: dict[str, Any]) -> str:
    turn = current_turn(state)
    if turn is not None:
        return turn
    raise ValueError("current turn is not set")

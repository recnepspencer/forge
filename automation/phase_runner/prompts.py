from __future__ import annotations

import re
from pathlib import Path
from typing import Any

from config_schema import resolve_config_path

TOKEN = re.compile(r"{([A-Za-z0-9_.]+)}")


def render_prompt(
    config: dict[str, Any],
    projection: dict[str, Any],
    config_path: Path,
    projection_path: Path,
    event_log_path: Path,
    expected_turn_instance_id: str | None = None,
) -> str:
    phase = current_phase_required(projection)
    turn = current_turn_required(projection)
    context = build_context(config, projection, config_path, projection_path, event_log_path, phase, turn)
    context["contract"] = render_contract(config, config_path, context)
    template_path = resolve_config_path(config_path, config["turn_templates"][turn])
    template = template_path.read_text(encoding="utf-8")
    rendered = render_template(template, context)
    if not expected_turn_instance_id:
        return rendered
    return (
        rendered
        + "\n\nRunner turn instance id: "
        + expected_turn_instance_id
        + "\nYour RUNNER_EVENT payload must include exactly "
        + json_turn_instance_snippet(expected_turn_instance_id)
        + "\n"
    )


def render_contract(
    config: dict[str, Any], config_path: Path, context: dict[str, Any]
) -> str:
    contract_path = resolve_config_path(config_path, config["contract_template"])
    contract = contract_path.read_text(encoding="utf-8")
    return render_template(contract, context)


def build_context(
    config: dict[str, Any],
    projection: dict[str, Any],
    config_path: Path,
    projection_path: Path,
    event_log_path: Path,
    phase: dict[str, Any],
    turn_kind: str,
) -> dict[str, Any]:
    project = config.get("project", {})
    cwd = Path(project.get("cwd", ".")).resolve()
    spec_file = project.get("spec_file", "")
    spec_path = Path(spec_file)
    if not spec_path.is_absolute():
        spec_path = cwd / spec_path

    return {
        "project": project,
        "phase": phase,
        "session": projection.get("session", {}),
        "config_file": str(config_path.resolve()),
        "projection_file": str(projection_path.resolve()),
        "event_log_file": str(event_log_path.resolve()),
        "spec_file": str(spec_path.resolve()),
        "turn": turn_kind,
        "current": projection.get("current", {}),
        "turns": ", ".join(config.get("turn_templates", {}).keys()),
        "status_values": "not_started, in_progress, complete, regressed, blocked",
        "qa_status_values": "not_started, needed, in_progress, passed, failed",
        "run_id": projection["run_id"],
        "current_turn_instance_id": projection.get("current_turn_instance_id"),
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


def json_turn_instance_snippet(turn_instance_id: str) -> str:
    return f'"turn_instance_id":"{turn_instance_id}"'


def current_phase_required(projection: dict[str, Any]) -> dict[str, Any]:
    current = projection.get("current")
    if not isinstance(current, dict):
        raise ValueError("current phase is not set")
    for phase in projection["phases"]:
        if phase["id"] == current["phase"]:
            return phase
    raise ValueError("current phase is not set")


def current_turn_required(projection: dict[str, Any]) -> str:
    current = projection.get("current")
    if isinstance(current, dict) and isinstance(current.get("turn"), str):
        return current["turn"]
    raise ValueError("current turn is not set")

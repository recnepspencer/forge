from __future__ import annotations

import re
from pathlib import Path
from typing import Any

from phase_execution import contract_template_path_for_phase, prompt_template_path_for_cursor

TOKEN = re.compile(r"{([A-Za-z0-9_.]+)}")

SUPPORTED_TEMPLATE_TOKENS = {
    "config_file",
    "contract",
    "current.phase",
    "current.turn",
    "event_log_file",
    "phase.acceptance",
    "phase.id",
    "phase.instructions",
    "phase.notes.findings",
    "phase.notes.plan",
    "phase.qa_focus",
    "phase.scope",
    "phase.success_event_type",
    "phase.title",
    "project.context_files",
    "projection_file",
    "qa_status_values",
    "run_id",
    "spec_file",
    "state_file",
    "status_values",
    "turns",
}


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
    template_path = prompt_template_path_for_cursor(config, config_path, phase, turn)
    template = template_path.read_text(encoding="utf-8")
    rendered = context.get("fresh_recovery_prompt", "") + render_template(template, context)
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
    contract_path = contract_template_path_for_phase(config, config_path, context["phase"])
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
        "state_file": str(projection_path.resolve()),
        "event_log_file": str(event_log_path.resolve()),
        "spec_file": str(spec_path.resolve()),
        "turn": turn_kind,
        "current": projection.get("current", {}),
        "turns": ", ".join(config.get("turn_templates", {}).keys()),
        "status_values": "not_started, in_progress, complete, regressed, blocked",
        "qa_status_values": "not_started, needed, in_progress, passed, failed",
        "run_id": projection["run_id"],
        "current_turn_instance_id": projection.get("current_turn_instance_id"),
        "fresh_recovery_prompt": build_fresh_recovery_prompt(projection),
    }


def render_template(template: str, context: dict[str, Any]) -> str:
    def replace(match: re.Match[str]) -> str:
        return stringify(resolve_token(context, match.group(1)))

    return TOKEN.sub(replace, template)


def unsupported_template_tokens(template: str) -> set[str]:
    return {
        token
        for token in TOKEN.findall(template)
        if token not in SUPPORTED_TEMPLATE_TOKENS
    }


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


def build_fresh_recovery_prompt(projection: dict[str, Any]) -> str:
    recovery = projection.get("session", {}).get("fresh_recovery")
    current = projection.get("current")
    if not isinstance(recovery, dict) or not isinstance(current, dict):
        return ""
    if recovery.get("phase") != current.get("phase") or recovery.get("turn") != current.get("turn"):
        return ""
    return f"""Fresh recovery session context:

The durable runner intentionally dropped the previous persistent agent session before this turn.
Reason: {recovery.get('reason')}
Observed QA/repair cycle count: {recovery.get('cycle_count')} (threshold: {recovery.get('threshold')})

You are a fresh agent stepping into a stuck phase. First rebuild context from the spec, projection,
event log, current phase, recent findings, recent repair summaries, and touched files. Look for the
deeper repeated structural cause before editing. This is not permission to bypass the current turn:
complete the current turn honestly and emit the normal RUNNER_EVENT for this turn when done.

"""


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

from __future__ import annotations

from pathlib import Path
from typing import Any


def build_render_context(
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
        "state_file": str(projection_path.resolve()),
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
        "fresh_recovery": projection.get("session", {}).get("fresh_recovery"),
        "operator_intervention": projection.get("operator_intervention"),
    }

from __future__ import annotations

from pathlib import Path
from typing import Any

from runner.authority.events import append_event
from runner.authority.events.run_authority import load_admitted_run_projection_inputs
from runner.authority.projections import project_run, write_projection
from runner.authority.run_identity import RuntimePaths, ensure_runtime_dirs, now_iso


def refresh_projection(config_path: Path, run_id: str) -> dict[str, Any]:
    admitted_config_path, config, events = load_admitted_projection_inputs(run_id)
    requested_config_path = Path(config_path).resolve()
    if requested_config_path != admitted_config_path:
        raise ValueError(
            f"run {run_id!r} is bound to config_path {admitted_config_path}, not {requested_config_path}"
        )
    return write_current_projection(run_id, config, events)


def refresh_projection_for_run(run_id: str) -> dict[str, Any]:
    _, config, events = load_admitted_projection_inputs(run_id)
    return write_current_projection(run_id, config, events)


def append_runtime_event(
    paths: RuntimePaths,
    event_type: str,
    payload: dict[str, Any],
    phase_id: int | None = None,
    turn: str | None = None,
    thread_id: str | None = None,
) -> dict[str, Any]:
    return append_event(
        paths,
        {
            "run_id": paths.run_id,
            "sequence": 0,
            "at": now_iso(),
            "event_type": event_type,
            "phase_id": phase_id,
            "turn": turn,
            "thread_id": thread_id,
            "payload": payload,
        },
    )


def config_path_for_run(run_id: str) -> Path:
    config_path, _, _ = load_admitted_projection_inputs(run_id)
    return config_path


def should_stop_before_phase(projection: dict[str, Any]) -> bool:
    current = projection.get("current")
    stop_before = projection.get("runner_control", {}).get("stop_before_phase")
    return isinstance(current, dict) and isinstance(stop_before, int) and current["phase"] >= stop_before


def stop_before_phase_reason(projection: dict[str, Any]) -> str:
    current = projection["current"]
    label = projection.get("runner_control", {}).get("stop_reason") or "configured stop-before-phase gate"
    return f"{label}: current phase {current['phase']} {current['turn']} reached stop_before_phase"


def load_admitted_projection_inputs(run_id: str) -> tuple[Path, dict[str, Any], tuple[dict[str, Any], ...]]:
    return load_admitted_run_projection_inputs(run_id)


def write_current_projection(
    run_id: str,
    config: dict[str, Any],
    events: tuple[dict[str, Any], ...],
) -> dict[str, Any]:
    paths = RuntimePaths(run_id)
    ensure_runtime_dirs()
    projection = project_run(config, list(events), run_id)
    write_projection(paths.projection, projection)
    return projection

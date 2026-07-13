from __future__ import annotations

from pathlib import Path
from typing import Any

from runner.authority.events import append_event, append_event_if_plan_version, initialize_event_log
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
    event = append_event(
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
    dispatch_appended_event(paths, event)
    return event


def append_runtime_event_if_plan_version(
    paths: RuntimePaths,
    event_type: str,
    payload: dict[str, Any],
    expected_plan_version: int,
) -> dict[str, Any]:
    event = append_event_if_plan_version(
        paths,
        runtime_event(paths, event_type, payload),
        expected_plan_version,
    )
    dispatch_appended_event(paths, event)
    return event


def initialize_runtime_events(
    paths: RuntimePaths,
    event_specs: list[tuple[str, dict[str, Any]]],
) -> list[dict[str, Any]]:
    events = [runtime_event(paths, event_type, payload) for event_type, payload in event_specs]
    initialize_event_log(paths, events)
    for event in events:
        dispatch_appended_event(paths, event)
    return events


def runtime_event(paths: RuntimePaths, event_type: str, payload: dict[str, Any]) -> dict[str, Any]:
    return {
        "run_id": paths.run_id,
        "sequence": 0,
        "at": now_iso(),
        "event_type": event_type,
        "phase_id": None,
        "turn": None,
        "thread_id": None,
        "payload": payload,
    }


def dispatch_appended_event(paths: RuntimePaths, event: dict[str, Any]) -> None:
    from runner.operator_signals.detectors import signals_for_event
    if not signals_for_event(event):
        return
    if event["event_type"] == "run_started":
        config_path = event["payload"].get("config_path")
        if not isinstance(config_path, str):
            return
        from runner.authority.config import load_config
        config = load_config(Path(config_path))
    else:
        try:
            _, config, _ = load_admitted_projection_inputs(paths.run_id)
        except ValueError:
            config = load_config_for_fault_notification(paths, event)
            if config is None:
                return
    if "notification_policy" not in config:
        return
    from runner.operator_signals import dispatch_authority_event
    dispatch_authority_event(paths, config, event)


def load_config_for_fault_notification(paths: RuntimePaths, event: dict[str, Any]) -> dict[str, Any] | None:
    if event["event_type"] != "runner_fault":
        return None
    from runner.authority.events import load_events

    for prior_event in load_events(paths.events):
        if prior_event.get("event_type") != "plan_adopted":
            continue
        config_path = prior_event.get("payload", {}).get("config_path")
        if not isinstance(config_path, str) or not config_path:
            continue
        from runner.authority.config import load_config

        return load_config(Path(config_path))
    return None


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

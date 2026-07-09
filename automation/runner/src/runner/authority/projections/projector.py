from __future__ import annotations

from typing import Any

from runner.authority.events.event_types import PHASE_PROGRESS_EVENTS
from runner.authority.run_identity import RuntimePaths
from runner.phase_programs import phase_program_id
from runner.phase_programs.transition_rules import (
    apply_phase_progress,
    first_turn,
    reconcile_projection_cursor_for_event,
    validate_projected_transition,
)
from runner.roles import project_current_session


def project_run(config: dict[str, Any], events: list[dict[str, Any]], run_id: str) -> dict[str, Any]:
    projection = empty_projection(config, run_id)
    if projection["phases"]:
        first_phase = projection["phases"][0]
        projection["current"] = {"phase": first_phase["id"], "turn": first_turn_for_event_history(config, first_phase, events)}
    for event in events:
        if event.get("thread_id"):
            projection["session"]["thread_id"] = event["thread_id"]
        if event["event_type"] == "prompt_selected":
            turn_instance_id = event["payload"].get("turn_instance_id")
            if turn_instance_id:
                projection["current_turn_instance_id"] = turn_instance_id
            fresh_recovery = projection["session"].get("fresh_recovery")
            if (
                isinstance(fresh_recovery, dict)
                and fresh_recovery.get("phase") == event.get("phase_id")
                and fresh_recovery.get("turn") == event.get("turn")
            ):
                projection["session"]["fresh_recovery"] = None
            intervention = projection.get("operator_intervention")
            if (
                isinstance(intervention, dict)
                and intervention.get("current") == {"phase": event.get("phase_id"), "turn": event.get("turn")}
            ):
                projection["operator_intervention"] = None
        if event["event_type"] == "run_started":
            projection["started_at"] = event["at"]
        if event["event_type"] == "run_resumed":
            projection["stopped"] = False
            projection["stop_reason"] = None
        if event["event_type"] == "run_stopped":
            projection["stopped"] = True
            projection["stop_reason"] = event["payload"].get("reason")
        if event["event_type"] == "session_reset":
            projection["session"]["thread_id"] = None
            projection["session"]["fresh_recovery"] = {
                "phase": event["phase_id"],
                "turn": event["turn"],
                "reason": event["payload"]["reason"],
                "threshold": event["payload"]["threshold"],
                "cycle_count": event["payload"]["cycle_count"],
            }
            projection["latest_summary"] = event["payload"].get("reason")
        if event["event_type"] == "run_completed":
            projection["current"] = None
            projection["current_turn_instance_id"] = None
            projection["completed_at"] = event["at"]
            projection["latest_summary"] = event["payload"].get("reason")
        if event["event_type"] == "operator_override":
            projection["current_turn_instance_id"] = None
            projection["latest_summary"] = event["payload"]["reason"]
            current = projection.get("current")
            if isinstance(current, dict) and current == event["payload"]["current"]:
                projection["operator_intervention"] = {
                    "reason": event["payload"]["reason"],
                    "current": event["payload"]["current"],
                    "injection_mode": event["payload"].get("injection_mode"),
                    "post_injection_route": event["payload"].get("post_injection_route"),
                }
        if event["event_type"] in {"runner_fault", "recovery_requested", "recovery_completed"}:
            reason = event["payload"].get("reason")
            if isinstance(reason, str):
                projection["latest_summary"] = reason
        reconcile_projection_cursor_for_event(projection, config, event)
        validate_projected_transition(projection, config, event)
        apply_phase_progress(projection, config, event)
        if event["event_type"] in PHASE_PROGRESS_EVENTS:
            projection["current_turn_instance_id"] = None
            projection["operator_intervention"] = None
    if projection["current"] is None and projection["completed_at"] is None and projection["phases"]:
        first_unfinished = find_first_unfinished_phase(projection)
        if first_unfinished is not None:
            projection["current"] = {"phase": first_unfinished["id"], "turn": first_turn(config, first_unfinished)}
    normalize_current_from_phase_state(projection)
    projection["last_event"] = events[-1] if events else None
    projection["session"] = project_current_session(config, projection.get("current"), projection["session"])
    return projection


def empty_projection(config: dict[str, Any], run_id: str) -> dict[str, Any]:
    paths = RuntimePaths(run_id)
    return {
        "run_id": run_id,
        "config_path": config.get("_config_path"),
        "projection_path": str(paths.projection.resolve()),
        "project": config["project"],
        "runner_control": config.get("runner_control", {}),
        "session": {
            "provider": None,
            "command": None,
            "command_args": [],
            "model": None,
            "reasoning_effort": None,
            "config": {},
            "env": {},
            "reuse_session": True,
            "thread_id": None,
            "fresh_recovery": None,
        },
        "phases": [project_phase(phase) for phase in config["phases"]],
        "operator_intervention": None,
        "current": None,
        "current_turn_instance_id": None,
        "started_at": None,
        "completed_at": None,
        "stopped": False,
        "stop_reason": None,
        "latest_summary": None,
        "last_event": None,
    }


def project_phase(phase: dict[str, Any]) -> dict[str, Any]:
    return {
        "id": phase["id"],
        "title": phase["title"],
        "owner": phase["owner"],
        "scope": phase["scope"],
        "acceptance": phase["acceptance"],
        "instructions": phase["instructions"],
        "qa_focus": phase["qa_focus"],
        "program_id": phase_program_id(phase),
        "status": "not_started",
        "qa_status": "not_started",
        "notes": {"plan": [], "done": [], "remaining": [], "findings": [], "verification": []},
    }


def find_first_unfinished_phase(projection: dict[str, Any]) -> dict[str, Any] | None:
    for phase in projection["phases"]:
        if phase["status"] != "complete" or phase["qa_status"] != "passed":
            return phase
    return None


def first_turn_for_event_history(config: dict[str, Any], phase: dict[str, Any], events: list[dict[str, Any]]) -> str:
    return first_turn(config, phase)


def normalize_current_from_phase_state(projection: dict[str, Any]) -> None:
    current = projection.get("current")
    if not isinstance(current, dict):
        return
    first_unfinished = find_first_unfinished_phase(projection)
    if first_unfinished is None:
        projection["current"] = None
        projection["current_turn_instance_id"] = None
        return
    current_phase = phase_by_id(projection, current.get("phase"))
    if current_phase is None:
        projection["current"] = {"phase": first_unfinished["id"], "turn": first_turn(projection, first_unfinished)}
        projection["current_turn_instance_id"] = None
        return
    if current_phase["status"] == "complete" and current_phase["qa_status"] == "passed":
        projection["current"] = {"phase": first_unfinished["id"], "turn": first_turn(projection, first_unfinished)}
        projection["current_turn_instance_id"] = None


def prompt_cursor_match_current(projection: dict[str, Any], prompt_phase_id: int, prompt_turn: str) -> bool:
    current = projection.get("current")
    if not isinstance(current, dict):
        return False
    first_unfinished = find_first_unfinished_phase(projection)
    if first_unfinished is None:
        return False
    expected_first_turn = first_turn(projection, first_unfinished)
    if prompt_phase_id == first_unfinished["id"] and prompt_turn == expected_first_turn:
        return True
    current_phase = phase_by_id(projection, current.get("phase"))
    if current_phase is None:
        return False
    if current_phase["status"] == "complete" and current_phase["qa_status"] == "passed":
        return False
    return current["phase"] == prompt_phase_id and current["turn"] == prompt_turn


def phase_by_id(projection: dict[str, Any], phase_id: Any) -> dict[str, Any] | None:
    for phase in projection["phases"]:
        if phase["id"] == phase_id:
            return phase
    return None

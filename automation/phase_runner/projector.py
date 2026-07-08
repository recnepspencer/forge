from __future__ import annotations

import json
from pathlib import Path
from typing import Any

from event_types import NOTE_BUCKETS
from transition_rules import apply_phase_progress, first_turn, validate_projected_transition

PHASE_CURSOR_ADVANCING_EVENTS = {
    "boundary_review_completed",
    "plan_posted",
    "implementation_completed",
    "review_failed",
    "review_passed",
    "repair_completed",
    "test_review_failed",
    "test_review_passed",
    "test_repair_plan_posted",
    "test_repair_completed",
    "code_quality_review_failed",
    "code_quality_repair_completed",
    "code_quality_review_passed",
}


def project_run(
    config: dict[str, Any],
    events: list[dict[str, Any]],
    run_id: str,
) -> dict[str, Any]:
    projection = empty_projection(config, run_id)
    if projection["phases"]:
        first_phase_id = projection["phases"][0]["id"]
        projection["current"] = {
            "phase": first_phase_id,
            "turn": first_turn(config, first_phase_id),
        }

    for event in events:
        if event.get("thread_id"):
            projection["session"]["thread_id"] = event["thread_id"]
        if event["event_type"] == "prompt_selected":
            turn_instance_id = event["payload"].get("turn_instance_id")
            current = prompt_cursor_match_current(
                projection,
                event.get("phase_id"),
                event.get("turn"),
            )
            if (
                isinstance(current, dict)
                and current.get("phase") == event.get("phase_id")
                and current.get("turn") == event.get("turn")
                and isinstance(turn_instance_id, str)
                and turn_instance_id
            ):
                projection["current"] = {
                    "phase": current["phase"],
                    "turn": current["turn"],
                }
                projection["current_turn_instance_id"] = turn_instance_id
            continue
        if event["event_type"] == "run_started":
            projection["started_at"] = event["at"]
            continue
        if event["event_type"] == "run_resumed":
            projection["stopped"] = False
            projection["stop_reason"] = None
            continue
        if event["event_type"] == "run_stopped":
            projection["stopped"] = True
            projection["stop_reason"] = event["payload"].get("reason")
            continue
        if event["event_type"] == "run_completed":
            projection["current"] = None
            projection["current_turn_instance_id"] = None
            projection["completed_at"] = event["at"]
            continue
        if event["event_type"] == "operator_override":
            projection["current"] = event["payload"]["current"]
            projection["current_turn_instance_id"] = None
            projection["latest_summary"] = event["payload"]["reason"]
            continue
        if event["event_type"] in {"runner_fault", "recovery_requested", "recovery_completed"}:
            reason = event["payload"].get("reason")
            if isinstance(reason, str) and reason:
                projection["latest_summary"] = reason
            continue
        validate_projected_transition(projection, event)
        apply_phase_progress(projection, config, event)
        if event["event_type"] in PHASE_CURSOR_ADVANCING_EVENTS:
            projection["current_turn_instance_id"] = None

    if projection["current"] is None and projection["completed_at"] is None and projection["phases"]:
        first_unfinished = find_first_unfinished_phase(projection)
        if first_unfinished is not None:
            projection["current"] = {
                "phase": first_unfinished["id"],
                "turn": first_turn(config, first_unfinished["id"]),
            }
    normalize_current_from_phase_state(projection)

    projection["last_event"] = events[-1] if events else None
    return projection


def empty_projection(config: dict[str, Any], run_id: str) -> dict[str, Any]:
    return {
        "run_id": run_id,
        "config_path": config["_config_path"],
        "project": config["project"],
        "turn_templates": config["turn_templates"],
        "contract_template": config["contract_template"],
        "runner_control": config.get("runner_control", {}),
        "session": {
            "provider": config["session_defaults"].get("provider", "codex"),
            "command": config["session_defaults"].get("command"),
            "command_args": config["session_defaults"].get("command_args", []),
            "model": config["session_defaults"]["model"],
            "reasoning_effort": config["session_defaults"].get("reasoning_effort"),
            "config": config["session_defaults"].get("config", {}),
            "reuse_session": config["session_defaults"].get("reuse_session", True),
            "thread_id": None,
        },
        "phases": [project_phase(phase) for phase in config["phases"]],
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
        "status": "not_started",
        "qa_status": "not_started",
        "notes": {bucket: [] for bucket in NOTE_BUCKETS},
    }


def find_first_unfinished_phase(projection: dict[str, Any]) -> dict[str, Any] | None:
    for phase in projection["phases"]:
        if phase["status"] != "complete" or phase["qa_status"] != "passed":
            return phase
    return None


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
        projection["current"] = {
            "phase": first_unfinished["id"],
            "turn": first_turn(projection, first_unfinished["id"]),
        }
        projection["current_turn_instance_id"] = None
        return
    if current_phase["status"] == "complete" and current_phase["qa_status"] == "passed":
        if current_phase["id"] != first_unfinished["id"]:
            projection["current"] = {
                "phase": first_unfinished["id"],
                "turn": first_turn(projection, first_unfinished["id"]),
            }
            projection["current_turn_instance_id"] = None


def prompt_cursor_match_current(
    projection: dict[str, Any],
    event_phase_id: Any,
    event_turn: Any,
) -> dict[str, Any] | None:
    current = projection.get("current")
    if not isinstance(current, dict):
        return None
    first_unfinished = find_first_unfinished_phase(projection)
    if first_unfinished is None:
        return current
    expected_first_turn = first_turn(projection, first_unfinished["id"])
    if event_phase_id == first_unfinished["id"] and event_turn == expected_first_turn:
        return {"phase": first_unfinished["id"], "turn": expected_first_turn}
    if expected_first_turn != "plan" and event_phase_id == first_unfinished["id"] and event_turn == "plan":
        return {"phase": first_unfinished["id"], "turn": "plan"}
    current_phase = phase_by_id(projection, current.get("phase"))
    if current_phase is None:
        return {"phase": first_unfinished["id"], "turn": expected_first_turn}
    if current_phase["status"] == "complete" and current_phase["qa_status"] == "passed":
        if current.get("turn") not in {"test_review", "test_repair_plan", "test_repair_implement", "code_quality_review"}:
            if current_phase["id"] != first_unfinished["id"]:
                return {"phase": first_unfinished["id"], "turn": expected_first_turn}
    return current


def phase_by_id(projection: dict[str, Any], phase_id: Any) -> dict[str, Any] | None:
    for phase in projection["phases"]:
        if phase["id"] == phase_id:
            return phase
    return None


def write_projection(path: Path, projection: dict[str, Any]) -> None:
    public = {
        key: value
        for key, value in projection.items()
        if key not in {"turn_templates", "contract_template"}
    }
    path.write_text(json.dumps(public, indent=2) + "\n", encoding="utf-8")

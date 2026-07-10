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
from runner.roles.registry import resolve_turn_role_binding


def project_run(config: dict[str, Any], events: list[dict[str, Any]], run_id: str) -> dict[str, Any]:
    projection = empty_projection(config, run_id)
    if projection["phases"]:
        first_phase = projection["phases"][0]
        projection["current"] = {"phase": first_phase["id"], "turn": first_turn_for_event_history(config, first_phase, events)}
    for event in events:
        if event.get("thread_id"):
            family = family_for_turn(config, event.get("phase_id"), event.get("turn"))
            if family:
                projection["session"]["threads"][family] = event["thread_id"]
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
        if event["event_type"] == "model_escalation_activated":
            payload = event["payload"]
            projection["model_overrides"].append(
                {
                    "phase_id": event.get("phase_id"),
                    "turns": list(payload.get("turns", [])),
                    "model_policy": dict(payload.get("model_policy", {})),
                    "scope": payload.get("scope", "phase"),
                }
            )
        if event["event_type"] == "run_started":
            projection["started_at"] = event["at"]
        if event["event_type"] == "run_resumed":
            projection["stopped"] = False
            projection["stop_reason"] = None
        if event["event_type"] == "run_stopped":
            projection["stopped"] = True
            projection["stop_reason"] = event["payload"].get("reason")
        if event["event_type"] == "session_reset":
            reset_family = family_for_turn(config, event.get("phase_id"), event.get("turn"))
            if reset_family:
                projection["session"]["threads"].pop(reset_family, None)
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
                    "model_policy": event["payload"].get("model_policy"),
                }
        if event["event_type"] in {"plan_adopted", "plan_revised"}:
            projection["plan"] = {
                "plan_version": event["payload"].get("plan_version"),
                "config_path": event["payload"].get("config_path"),
                "config_hash": event["payload"].get("config_hash"),
                "revision_class": event["payload"].get("revision_class"),
            }
        if event["event_type"] == "operator_prompt_override":
            projection["prompt_overrides"].append(event["payload"])
            projection["latest_summary"] = event["payload"]["reason"]
        if event["event_type"] == "external_phase_completed":
            apply_external_phase_completion(projection, event["payload"])
            projection["latest_summary"] = event["payload"]["summary"]
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
            "threads": {},
            "fresh_recovery": None,
        },
        "phases": [project_phase(phase) for phase in config["phases"]],
        "operator_intervention": None,
        "model_overrides": [],
        "prompt_overrides": [],
        "external_completions": [],
        "plan": None,
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
        "phase_key": phase.get("phase_key") or f"phase_{phase['id']}",
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


def family_for_turn(config: dict[str, Any], phase_id: Any, turn: Any) -> str | None:
    """Resolve the continuity family that owns a turn's provider session.

    Session identity is keyed by continuity family, not by a single global
    slot. The family is a pure function of the turn's role binding, so it is
    derived here from (phase_id, turn) rather than duplicated onto each event.
    Run-level events and turns without a role binding own no family thread.
    """
    if not isinstance(phase_id, int) or not isinstance(turn, str) or not turn:
        return None
    try:
        binding = resolve_turn_role_binding(config, phase_id, turn)
    except ValueError:
        return None
    return binding.session_policy_seed.continuity_family or None


def phase_by_id(projection: dict[str, Any], phase_id: Any) -> dict[str, Any] | None:
    for phase in projection["phases"]:
        if phase["id"] == phase_id:
            return phase
    return None


def apply_external_phase_completion(projection: dict[str, Any], payload: dict[str, Any]) -> None:
    phase = phase_by_key(projection, payload.get("phase_key"))
    if phase is None:
        raise ValueError(f"external completion references unknown phase_key {payload.get('phase_key')!r}")
    phase["status"] = "complete"
    phase["qa_status"] = "passed"
    phase["notes"]["done"].append(payload["summary"])
    for evidence in payload.get("evidence", []):
        phase["notes"]["verification"].append(evidence)
    projection["external_completions"].append(payload)


def phase_by_key(projection: dict[str, Any], phase_key: Any) -> dict[str, Any] | None:
    for phase in projection["phases"]:
        if phase.get("phase_key") == phase_key:
            return phase
    return None

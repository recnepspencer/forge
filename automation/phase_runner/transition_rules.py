from __future__ import annotations

from typing import Any

from event_types import PHASE_PROGRESS_EVENTS

TURN_OUTCOME_EVENTS = {
    "boundary_review": {"boundary_review_completed"},
    "plan": {"plan_posted"},
    "implement": {"implementation_completed"},
    "review": {"review_failed", "review_passed"},
    "repair": {"repair_completed"},
    "test_review": {"test_review_failed", "test_review_passed"},
    "test_repair_plan": {"test_repair_plan_posted"},
    "test_repair_implement": {"test_repair_completed"},
    "code_quality_review": {"code_quality_review_passed"},
}


def validate_turn_outcome(turn: str, event_type: str) -> None:
    allowed = TURN_OUTCOME_EVENTS.get(turn)
    if allowed is None:
        raise ValueError(f"turn {turn!r} does not support runner outcomes")
    if event_type not in allowed:
        raise ValueError(f"turn {turn!r} cannot emit {event_type!r}")


def validate_projected_transition(projection: dict[str, Any], event: dict[str, Any]) -> None:
    event_type = event["event_type"]
    if event_type not in PHASE_PROGRESS_EVENTS:
        return
    current = projection.get("current")
    phase_id = event["phase_id"]
    turn = event["turn"]
    if not isinstance(current, dict):
        raise ValueError(f"{event_type} cannot apply when no current cursor exists")
    if current.get("phase") != phase_id:
        raise ValueError(
            f"{event_type} targets phase {phase_id!r} while current phase is {current.get('phase')!r}"
        )
    if current.get("turn") == "boundary_review" and turn == "plan" and event_type == "plan_posted":
        return
    if current.get("turn") != turn:
        raise ValueError(
            f"{event_type} targets turn {turn!r} while current turn is {current.get('turn')!r}"
        )
    validate_turn_outcome(str(turn), event_type)


def apply_phase_progress(
    projection: dict[str, Any],
    config: dict[str, Any],
    event: dict[str, Any],
) -> None:
    event_type = event["event_type"]
    if event_type not in PHASE_PROGRESS_EVENTS:
        return
    phase_id = event["phase_id"]
    phase = phase_by_id(projection, phase_id)
    payload = event["payload"]
    merge_notes(phase["notes"], payload.get("notes", {}))
    summary = payload.get("summary")
    if isinstance(summary, str) and summary:
        projection["latest_summary"] = summary
    verification = payload.get("verification")
    if isinstance(verification, list):
        phase["notes"]["verification"] = verification

    if event_type == "boundary_review_completed":
        projection["current"] = {"phase": phase_id, "turn": "plan"}
        return
    if event_type == "plan_posted":
        phase["status"] = "in_progress"
        phase["qa_status"] = "not_started"
        projection["current"] = {"phase": phase_id, "turn": "implement"}
        return
    if event_type == "implementation_completed":
        phase["status"] = "complete"
        phase["qa_status"] = "needed"
        projection["current"] = {"phase": phase_id, "turn": "review"}
        return
    if event_type == "review_failed":
        phase["status"] = "regressed"
        phase["qa_status"] = "failed"
        projection["current"] = {"phase": phase_id, "turn": "repair"}
        return
    if event_type == "review_passed":
        phase["status"] = "complete"
        phase["qa_status"] = "passed"
        projection["current"] = {"phase": phase_id, "turn": "test_review"}
        return
    if event_type == "repair_completed":
        phase["status"] = "complete"
        phase["qa_status"] = "needed"
        projection["current"] = {"phase": phase_id, "turn": "review"}
        return
    if event_type == "test_review_failed":
        projection["current"] = {"phase": phase_id, "turn": "test_repair_plan"}
        return
    if event_type == "test_review_passed":
        projection["current"] = {"phase": phase_id, "turn": "code_quality_review"}
        return
    if event_type == "test_repair_plan_posted":
        projection["current"] = {"phase": phase_id, "turn": "test_repair_implement"}
        return
    if event_type == "test_repair_completed":
        next_turn = payload["next_turn"]
        projection["current"] = {"phase": phase_id, "turn": next_turn}
        return
    if event_type == "code_quality_review_passed":
        phase["status"] = "complete"
        phase["qa_status"] = "passed"
        advance_after_phase_close(projection, config, phase_id)


def merge_notes(target: dict[str, list[str]], incoming: dict[str, list[str]]) -> None:
    for bucket, entries in incoming.items():
        target[bucket] = entries


def phase_by_id(projection: dict[str, Any], phase_id: int) -> dict[str, Any]:
    for phase in projection["phases"]:
        if phase["id"] == phase_id:
            return phase
    raise ValueError(f"phase {phase_id!r} is not present")


def advance_after_phase_close(
    projection: dict[str, Any],
    config: dict[str, Any],
    phase_id: int,
) -> None:
    next_phase_id = phase_id + 1
    if next_phase_id > len(config["phases"]):
        projection["current"] = None
        return
    projection["current"] = {"phase": next_phase_id, "turn": first_turn(config, next_phase_id)}


def first_turn(config: dict[str, Any], phase_id: int) -> str:
    turn_templates = config.get("turn_templates", {})
    runner_control = config.get("runner_control", {})
    boundary_review_start_phase = runner_control.get("boundary_review_start_phase")
    if (
        "boundary_review" in turn_templates
        and isinstance(boundary_review_start_phase, int)
        and phase_id >= boundary_review_start_phase
    ):
        return "boundary_review"
    return "plan"

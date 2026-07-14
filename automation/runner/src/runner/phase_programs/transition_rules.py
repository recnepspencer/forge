from __future__ import annotations

from typing import Any

from runner.phase_programs.registry import PROGRAMS, lower_phase_program, phase_program_id


def validate_turn_outcome(
    phase: dict[str, Any],
    turn: str,
    event_type: str,
    *,
    config: dict[str, Any] | None = None,
) -> None:
    lowered_program = lowered_program_for_phase(config, phase)
    allowed = lowered_program.supported_outcomes_for_turn(turn)
    if event_type not in allowed:
        raise ValueError(f"turn {turn!r} does not accept outcome {event_type!r}; expected one of {sorted(allowed)}")


def validate_projected_transition(projection: dict[str, Any], config: dict[str, Any], event: dict[str, Any]) -> None:
    current = projection.get("current")
    if not isinstance(current, dict):
        return
    current_phase = phase_by_id(projection, current["phase"])
    if current_phase is None:
        return
    lowered_program = lower_phase_program(config, current_phase)
    if not lowered_program.recognizes_event_type(event["event_type"]):
        return
    if event.get("phase_id") != current["phase"]:
        raise ValueError(f"event {event['event_type']!r} phase {event.get('phase_id')!r} does not match current phase {current['phase']!r}")
    allowed = lowered_program.supported_outcomes_for_turn(current["turn"])
    if event["event_type"] not in allowed:
        raise ValueError(f"event {event['event_type']!r} is not valid for current turn {current['turn']!r}")


def reconcile_projection_cursor_for_event(projection: dict[str, Any], config: dict[str, Any], event: dict[str, Any]) -> None:
    phase_id = event.get("phase_id")
    turn = event.get("turn")
    if not isinstance(phase_id, int) or not isinstance(turn, str):
        return
    current = projection.get("current")
    if not isinstance(current, dict):
        projection["current"] = {"phase": phase_id, "turn": turn}
        projection["current_turn_instance_id"] = None
        return
    event_phase = phase_by_id(projection, phase_id)
    if event_phase is None:
        return
    lowered_program = lower_phase_program(config, event_phase)
    if not lowered_program.supports_turn(turn):
        return
    if event["event_type"] not in lowered_program.supported_outcomes_for_turn(turn):
        return
    if current["phase"] == phase_id:
        if current["turn"] != turn:
            projection["current"] = {"phase": phase_id, "turn": turn}
            projection["current_turn_instance_id"] = None
        return
    if compare_phase_order(projection, current["phase"], phase_id) >= 0:
        return
    close_prior_phases_for_replay(projection, phase_id)
    projection["current"] = {"phase": phase_id, "turn": turn}
    projection["current_turn_instance_id"] = None


def apply_phase_progress(projection: dict[str, Any], config: dict[str, Any], event: dict[str, Any]) -> None:
    phase_id = event.get("phase_id")
    if not isinstance(phase_id, int):
        return
    phase = phase_by_id(projection, phase_id)
    if phase is None:
        raise ValueError(f"event references unknown phase {phase_id}")
    lowered_program = lower_phase_program(config, phase)
    merge_notes(phase["notes"], event["payload"].get("notes", {}))
    lowered_program.apply_phase_progress(projection, config, phase_id, event)


def merge_notes(target: dict[str, list[str]], incoming: dict[str, list[str]]) -> None:
    for key, items in incoming.items():
        target.setdefault(key, []).extend(items)


def phase_by_id(projection: dict[str, Any], phase_id: Any) -> dict[str, Any] | None:
    for phase in projection["phases"]:
        if phase["id"] == phase_id:
            return phase
    return None


def compare_phase_order(projection: dict[str, Any], left_phase_id: int, right_phase_id: int) -> int:
    phase_ids = [phase["id"] for phase in projection["phases"]]
    try:
        left_index = phase_ids.index(left_phase_id)
        right_index = phase_ids.index(right_phase_id)
    except ValueError:
        return 0
    if left_index < right_index:
        return -1
    if left_index > right_index:
        return 1
    return 0


def close_prior_phases_for_replay(projection: dict[str, Any], phase_id: int) -> None:
    for phase in projection["phases"]:
        if phase["id"] == phase_id:
            return
        phase["status"] = "complete"
        phase["qa_status"] = "passed"


def advance_after_phase_close(projection: dict[str, Any], config: dict[str, Any], phase_id: int) -> None:
    next_phase_id = next_phase_id_after(projection, phase_id)
    if next_phase_id is None:
        projection["current"] = None
        return
    next_phase = phase_by_id(projection, next_phase_id)
    projection["current"] = {"phase": next_phase_id, "turn": first_turn(config, next_phase)}


def first_turn(config: dict[str, Any], phase: dict[str, Any]) -> str:
    return lower_phase_program(config, phase).first_turn


def lowered_program_for_phase(config: dict[str, Any] | None, phase: dict[str, Any]):
    if config is None:
        return PROGRAMS[phase_program_id(phase)]
    return lower_phase_program(config, phase)


def phase_by_id_or_none(projection: dict[str, Any], phase_id: int) -> dict[str, Any] | None:
    return phase_by_id(projection, phase_id)


def next_phase_id_after(projection: dict[str, Any], phase_id: int) -> int | None:
    phase_ids = [phase["id"] for phase in projection["phases"]]
    try:
        index = phase_ids.index(phase_id)
    except ValueError:
        return None
    next_index = index + 1
    if next_index >= len(phase_ids):
        return None
    return phase_ids[next_index]

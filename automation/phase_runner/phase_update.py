from __future__ import annotations

from dataclasses import dataclass
from typing import Any

from state import append_history, now_iso, phase_by_id
from state_normalization import REQUIRED_NOTE_LISTS, normalize_note_bucket
from turn_state import next_turn_allowed


class PhaseUpdateError(Exception):
    pass


@dataclass
class PhaseUpdate:
    phase: int
    completed_turn: str
    status: str
    qa_status: str
    next_turn: str | None
    notes: dict[str, list[Any]]
    detail: str


def parse_phase_update(payload: dict[str, Any]) -> PhaseUpdate:
    notes = payload.get("notes", {})
    if not isinstance(notes, dict):
        raise PhaseUpdateError("phase update notes must be an object")
    normalized_notes: dict[str, list[Any]] = {}
    for key in REQUIRED_NOTE_LISTS:
        if key in notes:
            normalized_notes[key] = normalize_note_bucket(notes[key])
    phase = payload.get("phase")
    completed_turn = payload.get("completed_turn")
    status = payload.get("status")
    qa_status = payload.get("qa_status")
    next_turn = payload.get("next_turn")
    detail = payload.get("detail", "")
    if not isinstance(phase, int):
        raise PhaseUpdateError("phase update phase must be an integer")
    if not isinstance(completed_turn, str) or not completed_turn:
        raise PhaseUpdateError("phase update completed_turn is required")
    if not isinstance(status, str) or not status:
        raise PhaseUpdateError("phase update status is required")
    if not isinstance(qa_status, str) or not qa_status:
        raise PhaseUpdateError("phase update qa_status is required")
    if next_turn is not None and (not isinstance(next_turn, str) or not next_turn):
        raise PhaseUpdateError("phase update next_turn must be a string or null")
    if not isinstance(detail, str):
        raise PhaseUpdateError("phase update detail must be a string")
    return PhaseUpdate(
        phase=phase,
        completed_turn=completed_turn,
        status=status,
        qa_status=qa_status,
        next_turn=next_turn,
        notes=normalized_notes,
        detail=detail.strip(),
    )


def apply_phase_update(state: dict[str, Any], update: PhaseUpdate) -> None:
    current = state.get("current")
    if not isinstance(current, dict):
        raise PhaseUpdateError("state has no active current cursor")
    current_phase = current.get("phase")
    current_turn = current.get("turn")
    if current_phase != update.phase or current_turn != update.completed_turn:
        raise PhaseUpdateError(
            f"phase update must match current cursor; expected phase {current_phase} "
            f"{current_turn}, got phase {update.phase} {update.completed_turn}"
        )
    if not next_turn_allowed(update.completed_turn, update.next_turn):
        raise PhaseUpdateError(
            f"illegal next turn {update.next_turn!r} after {update.completed_turn!r}"
        )

    phase = phase_by_id(state, update.phase)
    phase["status"] = update.status
    phase["qa_status"] = update.qa_status
    notes = phase.setdefault("notes", {})
    for bucket, entries in update.notes.items():
        notes[bucket] = entries

    state["current"] = derive_next_cursor(state, update)
    if state["current"] is None:
        state["completed_at"] = now_iso()
    else:
        state.pop("completed_at", None)

    detail = update.detail or f"phase {update.phase} {update.completed_turn}"
    append_history(state, "codex_turn_completed", detail, 0)


def derive_next_cursor(state: dict[str, Any], update: PhaseUpdate) -> dict[str, Any] | None:
    if update.completed_turn != "code_quality_review":
        if update.next_turn is None:
            raise PhaseUpdateError("next_turn may only be null after code_quality_review")
        return {"phase": update.phase, "turn": update.next_turn}

    phases = state.get("phases", [])
    for index, phase in enumerate(phases):
        if int(phase.get("id", -1)) != update.phase:
            continue
        if update.next_turn is not None:
            return {"phase": update.phase, "turn": update.next_turn}
        next_index = index + 1
        if next_index >= len(phases):
            return None
        return {"phase": phases[next_index]["id"], "turn": "plan"}
    raise PhaseUpdateError(f"phase {update.phase} is not present")

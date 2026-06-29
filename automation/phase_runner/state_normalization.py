from __future__ import annotations

from typing import Any

REQUIRED_NOTE_LISTS = ("plan", "done", "remaining", "findings", "verification")


def normalize_state(state: dict[str, Any]) -> bool:
    changed = False
    phases = state.get("phases")
    if not isinstance(phases, list):
        return False
    for phase in phases:
        if not isinstance(phase, dict):
            continue
        changed = normalize_phase_notes(phase) or changed
    return changed


def normalize_phase_notes(phase: dict[str, Any]) -> bool:
    notes = phase.get("notes")
    changed = False
    if notes is None:
        notes = {}
        phase["notes"] = notes
        changed = True
    if not isinstance(notes, dict):
        return changed
    for key in REQUIRED_NOTE_LISTS:
        normalized = normalize_note_bucket(notes.get(key))
        if notes.get(key) != normalized:
            notes[key] = normalized
            changed = True
    return changed


def normalize_note_bucket(value: Any) -> list[Any]:
    if value is None:
        return []
    if isinstance(value, list):
        return value
    return [value]

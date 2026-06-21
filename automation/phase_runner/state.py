from __future__ import annotations

import datetime as dt
import json
from pathlib import Path
from typing import Any

MAX_HISTORY_ENTRIES = 120
PHASE_STATUSES = {"not_started", "in_progress", "complete", "regressed", "blocked"}
QA_STATUSES = {"not_started", "needed", "in_progress", "passed", "failed"}


def now_iso() -> str:
    return dt.datetime.now(dt.timezone.utc).isoformat()


def load_state(path: Path) -> dict[str, Any]:
    with path.open("r", encoding="utf-8-sig") as state_file:
        state = json.load(state_file)
    state["_state_path"] = str(path)
    return state


def public_state(state: dict[str, Any]) -> dict[str, Any]:
    return {key: value for key, value in state.items() if not key.startswith("_")}


def save_state(path: Path, state: dict[str, Any]) -> None:
    state = public_state(state)
    trim_history(state)
    state["state_revision"] = int(state.get("state_revision", 0)) + 1
    if path.exists():
        backup_path(path).write_bytes(path.read_bytes())
    path.write_text(json.dumps(state, indent=2) + "\n", encoding="utf-8")


def backup_path(path: Path) -> Path:
    return path.with_name(f"{path.name}.bak")


def trim_history(state: dict[str, Any]) -> None:
    history = state.get("history")
    if isinstance(history, list) and len(history) > MAX_HISTORY_ENTRIES:
        state["history"] = history[-MAX_HISTORY_ENTRIES:]


def append_history(
    state: dict[str, Any],
    event: str,
    detail: str,
    exit_code: int | None = None,
) -> None:
    state.setdefault("history", []).append(
        {
            "at": now_iso(),
            "event": event,
            "detail": detail,
            "exit_code": exit_code,
        }
    )


def current_cursor(state: dict[str, Any]) -> dict[str, Any] | None:
    current = state.get("current")
    if not isinstance(current, dict):
        return None
    if current.get("phase") is None or current.get("turn") is None:
        return None
    return current


def phase_by_id(state: dict[str, Any], phase_id: int) -> dict[str, Any]:
    for phase in state["phases"]:
        if int(phase["id"]) == int(phase_id):
            return phase
    raise ValueError(f"phase id {phase_id!r} is not present")


def current_phase(state: dict[str, Any]) -> dict[str, Any] | None:
    cursor = current_cursor(state)
    if cursor is None:
        return None
    return phase_by_id(state, int(cursor["phase"]))


def current_turn(state: dict[str, Any]) -> str | None:
    cursor = current_cursor(state)
    if cursor is None:
        return None
    return str(cursor["turn"])


def cursor_reason(state: dict[str, Any]) -> str:
    cursor = current_cursor(state)
    if cursor is None:
        return "all phases are complete"
    return f"phase {cursor['phase']} {cursor['turn']}"

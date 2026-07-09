from __future__ import annotations

import json
from pathlib import Path
from typing import Any

from runner.authority.events.event_types import validate_event_shape
from runner.authority.run_identity.runtime_paths import RuntimePaths, acquire_event_append_lock, ensure_runtime_dirs


class EventLogDecodeError(ValueError):
    def __init__(self, line_number: int, message: str) -> None:
        super().__init__(f"event log decode error on line {line_number}: {message}")
        self.line_number = line_number


def load_events(path: Path) -> list[dict[str, Any]]:
    if not path.exists():
        return []
    events: list[dict[str, Any]] = []
    raw = path.read_text(encoding="utf-8")
    lines = raw.splitlines(keepends=True)
    has_complete_trailing_newline = raw.endswith(("\n", "\r"))
    for index, line in enumerate(lines, start=1):
        if not line.strip():
            continue
        try:
            events.append(json.loads(line))
        except json.JSONDecodeError as error:
            is_last_line = index == len(lines)
            if is_last_line and not has_complete_trailing_newline:
                break
            raise EventLogDecodeError(index, str(error)) from error
    return events


def append_event(paths: RuntimePaths, event: dict[str, Any]) -> dict[str, Any]:
    ensure_runtime_dirs()
    with acquire_event_append_lock(paths):
        events = load_events(paths.events)
        next_sequence = len(events) + 1
        event["sequence"] = next_sequence
        errors = validate_event_shape(event)
        if errors:
            raise ValueError("; ".join(errors))
        with paths.events.open("a", encoding="utf-8") as output:
            output.write(json.dumps(event, separators=(",", ":")) + "\n")
    return event


def validate_event_log(events: list[dict[str, Any]], run_id: str) -> list[str]:
    errors: list[str] = []
    for expected_sequence, event in enumerate(events, start=1):
        if event.get("run_id") != run_id:
            errors.append(f"sequence {expected_sequence} has mismatched run_id {event.get('run_id')!r}")
        if event.get("sequence") != expected_sequence:
            errors.append(f"sequence {expected_sequence} is stored as {event.get('sequence')!r}")
        errors.extend(validate_event_shape(event))
    return errors

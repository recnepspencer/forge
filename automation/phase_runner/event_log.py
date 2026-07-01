from __future__ import annotations

import json
from pathlib import Path
from typing import Any

from event_types import validate_event_shape
from runtime_paths import RuntimePaths, ensure_runtime_dirs


def load_events(path: Path) -> list[dict[str, Any]]:
    if not path.exists():
        return []
    events: list[dict[str, Any]] = []
    for line in path.read_text(encoding="utf-8").splitlines():
        if not line.strip():
            continue
        events.append(json.loads(line))
    return events


def append_event(paths: RuntimePaths, event: dict[str, Any]) -> dict[str, Any]:
    ensure_runtime_dirs()
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
            errors.append(
                f"sequence {expected_sequence} has mismatched run_id {event.get('run_id')!r}"
            )
        if event.get("sequence") != expected_sequence:
            errors.append(
                f"sequence {expected_sequence} is stored as {event.get('sequence')!r}"
            )
        errors.extend(validate_event_shape(event))
    return errors

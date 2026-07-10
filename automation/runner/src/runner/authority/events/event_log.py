from __future__ import annotations

import json
import os
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
        repair_truncated_tail(paths.events)
        events = load_events(paths.events)
        append_validated_event(paths, events, event)
    return event


def append_event_if_plan_version(
    paths: RuntimePaths,
    event: dict[str, Any],
    expected_plan_version: int,
) -> dict[str, Any]:
    ensure_runtime_dirs()
    with acquire_event_append_lock(paths):
        repair_truncated_tail(paths.events)
        events = load_events(paths.events)
        actual_plan_version = latest_plan_version(events)
        if actual_plan_version != expected_plan_version:
            raise ValueError(
                f"stale plan revision: expected plan_version={expected_plan_version}, "
                f"current plan_version={actual_plan_version}"
            )
        append_validated_event(paths, events, event)
    return event


def initialize_event_log(paths: RuntimePaths, events: list[dict[str, Any]]) -> list[dict[str, Any]]:
    ensure_runtime_dirs()
    with acquire_event_append_lock(paths):
        if paths.events.exists():
            raise ValueError(f"run {paths.run_id!r} already exists")
        for sequence, event in enumerate(events, start=1):
            event["sequence"] = sequence
            errors = validate_event_shape(event)
            if errors:
                raise ValueError("; ".join(errors))
        encoded = "".join(json.dumps(event, separators=(",", ":")) + "\n" for event in events)
        temporary = paths.events.with_name(f"{paths.events.name}.{os.getpid()}.tmp")
        try:
            temporary.write_text(encoded, encoding="utf-8")
            os.replace(temporary, paths.events)
        finally:
            try:
                temporary.unlink()
            except FileNotFoundError:
                pass
    return events


def append_validated_event(paths: RuntimePaths, events: list[dict[str, Any]], event: dict[str, Any]) -> None:
    event["sequence"] = len(events) + 1
    errors = validate_event_shape(event)
    if errors:
        raise ValueError("; ".join(errors))
    with paths.events.open("a", encoding="utf-8") as output:
        output.write(json.dumps(event, separators=(",", ":")) + "\n")


def latest_plan_version(events: list[dict[str, Any]]) -> int | None:
    for event in reversed(events):
        if event.get("event_type") not in {"plan_adopted", "plan_revised"}:
            continue
        version = event.get("payload", {}).get("plan_version")
        if isinstance(version, int):
            return version
    return None


def repair_truncated_tail(path: Path) -> None:
    if not path.exists():
        return
    raw = path.read_text(encoding="utf-8")
    if not raw or raw.endswith(("\n", "\r")):
        return
    line_start = max(raw.rfind("\n"), raw.rfind("\r")) + 1
    try:
        json.loads(raw[line_start:])
    except json.JSONDecodeError:
        path.write_text(raw[:line_start], encoding="utf-8")
        return
    with path.open("a", encoding="utf-8") as output:
        output.write("\n")


def validate_event_log(events: list[dict[str, Any]], run_id: str) -> list[str]:
    errors: list[str] = []
    for expected_sequence, event in enumerate(events, start=1):
        if event.get("run_id") != run_id:
            errors.append(f"sequence {expected_sequence} has mismatched run_id {event.get('run_id')!r}")
        if event.get("sequence") != expected_sequence:
            errors.append(f"sequence {expected_sequence} is stored as {event.get('sequence')!r}")
        errors.extend(validate_event_shape(event))
    return errors

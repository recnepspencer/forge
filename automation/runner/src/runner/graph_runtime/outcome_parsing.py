from __future__ import annotations

import json
import re
from typing import Any

from runner.authority.events import validate_runner_outcome
from runner.recovery.failure_families import (
    MALFORMED_RUNNER_EVENT_FAMILY,
    MISSING_RUNNER_EVENT_FAMILY,
)

RUNNER_EVENT_PATTERN = re.compile(r"RUNNER_EVENT:\s*(\{.*\})")
FENCED_JSON_PATTERN = re.compile(r"```(?:json)?\s*(\{.*?\})\s*```", re.DOTALL)
MISSING_RUNNER_EVENT_REASON = "no RUNNER_EVENT line found in agent messages"


class RunnerOutcomeError(ValueError):
    failure_family: str | None = None


class MissingRunnerEventError(RunnerOutcomeError):
    failure_family = MISSING_RUNNER_EVENT_FAMILY


class MalformedRunnerEventError(RunnerOutcomeError):
    failure_family = MALFORMED_RUNNER_EVENT_FAMILY


def extract_runner_event(
    agent_messages: list[str],
    expected_turn_instance_id: str | None = None,
) -> dict[str, Any]:
    for candidate in iter_runner_event_candidates(agent_messages):
        parsed = parse_runner_event_candidate(candidate, expected_turn_instance_id)
        if parsed is not None:
            return parsed
    raise MissingRunnerEventError(MISSING_RUNNER_EVENT_REASON)


def iter_runner_event_candidates(agent_messages: list[str]) -> list[str]:
    candidates: list[str] = []
    for message in reversed(agent_messages):
        for line in reversed(message.splitlines()):
            match = RUNNER_EVENT_PATTERN.search(line.strip())
            if match:
                candidates.append(match.group(1))
        for line in reversed(message.splitlines()):
            stripped = line.strip()
            if stripped.startswith("{") and stripped.endswith("}"):
                candidates.append(stripped)
        for match in reversed(list(FENCED_JSON_PATTERN.finditer(message))):
            candidates.append(match.group(1).strip())
    return candidates


def parse_runner_event_candidate(
    candidate: str,
    expected_turn_instance_id: str | None,
) -> dict[str, Any] | None:
    try:
        parsed = json.loads(candidate)
    except json.JSONDecodeError:
        return None
    if not isinstance(parsed, dict):
        return None
    event_type = parsed.get("event_type")
    payload = parsed.get("payload", {})
    if not isinstance(event_type, str) or not event_type:
        return None
    if not isinstance(payload, dict):
        raise MalformedRunnerEventError("RUNNER_EVENT payload must be an object")
    if expected_turn_instance_id is not None and payload.get("turn_instance_id") != expected_turn_instance_id:
        raise MalformedRunnerEventError(f"RUNNER_EVENT turn_instance_id must be {expected_turn_instance_id!r}")
    errors = validate_runner_outcome(event_type, payload)
    if errors:
        raise MalformedRunnerEventError("; ".join(errors))
    return {"event_type": event_type, "payload": payload}

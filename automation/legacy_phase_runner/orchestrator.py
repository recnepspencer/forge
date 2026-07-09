from __future__ import annotations

import json
import re
import time
import uuid
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

from agent_cli import run_agent
from config_schema import load_config
from event_log import EventLogDecodeError, append_event, load_events, validate_event_log
from event_types import validate_runner_outcome
from legacy_importer import import_legacy_run as import_legacy_run_impl
from projector import project_run, write_projection
from prompts import render_prompt
from runtime_paths import (
    RuntimePaths,
    acquire_active_run_lock,
    clear_stop_requested,
    ensure_runtime_dirs,
    mark_stop_requested,
    stop_requested,
)
from transition_rules import validate_turn_outcome

RUNNER_EVENT_PATTERN = re.compile(r"RUNNER_EVENT:\s*(\{.*\})")
QA_REPAIR_COMPLETED_EVENTS = {
    "repair_completed",
    "test_repair_completed",
    "code_quality_repair_completed",
}


def start_run(
    config_path: Path,
    run_id: str | None,
    loop: bool,
    sleep_seconds: int,
    log_path: Path | None,
) -> int:
    load_config(config_path)
    active_run_id = run_id or new_run_id()
    paths = RuntimePaths(active_run_id)
    with acquire_active_run_lock(paths):
        clear_stop_requested(paths)
        if paths.events.exists():
            raise ValueError(f"run {active_run_id!r} already exists")
        append_runtime_event(
            paths,
            "run_started",
            payload={"config_path": str(config_path.resolve())},
        )
        return drive_run(config_path, active_run_id, loop, sleep_seconds, log_path)


def resume_run(run_id: str, loop: bool, sleep_seconds: int, log_path: Path | None) -> int:
    return resume_run_with_reason(run_id, loop, sleep_seconds, log_path, "operator resume")


def resume_run_with_reason(
    run_id: str,
    loop: bool,
    sleep_seconds: int,
    log_path: Path | None,
    reason: str,
) -> int:
    config_path = config_path_for_run(run_id)
    paths = RuntimePaths(run_id)
    with acquire_active_run_lock(paths):
        clear_stop_requested(paths)
        append_runtime_event(
            paths,
            "run_resumed",
            payload={"reason": reason},
        )
        return drive_run(config_path, run_id, loop, sleep_seconds, log_path)


def stop_run(run_id: str, reason: str) -> None:
    paths = RuntimePaths(run_id)
    mark_stop_requested(paths)
    append_runtime_event(paths, "run_stopped", payload={"reason": reason})
    refresh_projection(config_path_for_run(run_id), run_id)


def import_legacy_run(old_state_path: Path, config_path: Path, run_id: str | None) -> str:
    return import_legacy_run_impl(
        old_state_path,
        config_path,
        run_id,
        append_runtime_event,
        refresh_projection,
        new_run_id,
    )


def drive_run(
    config_path: Path,
    run_id: str,
    loop: bool,
    sleep_seconds: int,
    log_path: Path | None,
) -> int:
    while True:
        projection = refresh_projection(config_path, run_id)
        if projection["completed_at"] is not None or projection["stopped"]:
            return 0
        current = projection.get("current")
        if current is None:
            append_runtime_event(
                RuntimePaths(run_id),
                "run_completed",
                payload={"reason": "all phases are complete"},
                thread_id=projection["session"]["thread_id"],
            )
            completed_projection = refresh_projection(config_path, run_id)
            return run_completion_handoff(completed_projection, run_id)
        if should_stop_before_phase(projection):
            append_runtime_event(
                RuntimePaths(run_id),
                "run_stopped",
                payload={"reason": stop_before_phase_reason(projection)},
                thread_id=projection["session"]["thread_id"],
            )
            refresh_projection(config_path, run_id)
            return 0

        if maybe_reset_stuck_session(config_path, run_id, projection):
            projection = refresh_projection(config_path, run_id)

        recovery_reason = pending_recovery_reason(
            load_events(RuntimePaths(run_id).events),
            current,
            projection.get("current_turn_instance_id"),
        )
        if recovery_reason is not None:
            status = run_recovery_turn(config_path, run_id, log_path, recovery_reason)
        else:
            status = run_single_turn(config_path, run_id, log_path)
        if status != 0 or not loop:
            return status
        time.sleep(sleep_seconds)


def run_single_turn(config_path: Path, run_id: str, log_path: Path | None) -> int:
    config = load_config(config_path)
    projection = refresh_projection(config_path, run_id)
    current = projection["current"]
    paths = RuntimePaths(run_id)
    turn_instance_id = new_run_id()
    append_runtime_event(
        paths,
        "prompt_selected",
        phase_id=current["phase"],
        turn=current["turn"],
        payload={
            "summary": f"phase {current['phase']} {current['turn']}",
            "turn_instance_id": turn_instance_id,
        },
        thread_id=projection["session"]["thread_id"],
    )
    projection = refresh_projection(config_path, run_id)
    prompt = render_prompt(
        config,
        projection,
        config_path,
        paths.projection,
        paths.events,
        expected_turn_instance_id=turn_instance_id,
    )

    exit_code, capture = run_agent(
        projection,
        prompt,
        log_path or paths.log,
        stop_requested_fn=lambda: stop_requested(paths),
    )
    thread_id = capture.get("thread_id") or projection["session"]["thread_id"]
    event_payload = {
        "summary": f"phase {current['phase']} {current['turn']}",
        "agent_message_count": len(capture.get("agent_messages", [])),
        "turn_instance_id": turn_instance_id,
    }
    if exit_code != 0:
        failure_reason = capture.get("failure_reason") or f"agent exited with {exit_code}"
        if failure_reason == "operator stop requested":
            append_runtime_event(
                paths,
                "codex_turn_failed",
                phase_id=current["phase"],
                turn=current["turn"],
                payload={**event_payload, "exit_code": exit_code},
                thread_id=thread_id,
            )
            return 0
        append_runtime_event(
            paths,
            "codex_turn_failed",
            phase_id=current["phase"],
            turn=current["turn"],
            payload={**event_payload, "exit_code": exit_code},
            thread_id=thread_id,
        )
        append_runtime_event(
            paths,
            "runner_fault",
            phase_id=current["phase"],
            turn=current["turn"],
            payload={"reason": failure_reason, "turn_instance_id": turn_instance_id},
            thread_id=thread_id,
        )
        return run_recovery_turn(config_path, run_id, log_path, failure_reason)

    append_runtime_event(
        paths,
        "codex_turn_completed",
        phase_id=current["phase"],
        turn=current["turn"],
        payload={**event_payload, "exit_code": 0},
        thread_id=thread_id,
    )
    if not turn_is_current(config_path, run_id, current, turn_instance_id):
        return 0
    try:
        outcome = extract_runner_event(capture.get("agent_messages", []), turn_instance_id)
        validate_turn_outcome(current_phase_for_projection(projection), current["turn"], outcome["event_type"])
        append_runtime_event(
            paths,
            outcome["event_type"],
            phase_id=current["phase"],
            turn=current["turn"],
            payload=outcome["payload"],
            thread_id=thread_id,
        )
        append_runtime_event(
            paths,
            "turn_outcome_recorded",
            phase_id=current["phase"],
            turn=current["turn"],
            payload={
                "outcome_event_type": outcome["event_type"],
                "turn_instance_id": turn_instance_id,
            },
            thread_id=thread_id,
        )
        refresh_projection(config_path, run_id)
        return 0
    except Exception as error:
        append_runtime_event(
            paths,
            "runner_fault",
            phase_id=current["phase"],
            turn=current["turn"],
            payload={"reason": str(error), "turn_instance_id": turn_instance_id},
            thread_id=thread_id,
        )
        return run_recovery_turn(config_path, run_id, log_path, str(error))


def run_recovery_turn(config_path: Path, run_id: str, log_path: Path | None, reason: str) -> int:
    config = load_config(config_path)
    projection = refresh_projection(config_path, run_id)
    current = projection["current"]
    paths = RuntimePaths(run_id)
    turn_instance_id = projection.get("current_turn_instance_id")
    if current is not None and (not isinstance(turn_instance_id, str) or not turn_instance_id):
        return 0
    append_runtime_event(
        paths,
        "recovery_requested",
        phase_id=current["phase"] if current else None,
        turn=current["turn"] if current else None,
        payload={"reason": reason, "turn_instance_id": turn_instance_id},
        thread_id=projection["session"]["thread_id"],
    )
    prompt = build_recovery_prompt(projection, paths, reason, turn_instance_id)
    exit_code, capture = run_agent(
        projection,
        prompt,
        log_path or paths.log,
        stop_requested_fn=lambda: stop_requested(paths),
    )
    thread_id = capture.get("thread_id") or projection["session"]["thread_id"]
    if exit_code != 0:
        failure_reason = capture.get("failure_reason") or f"agent exited with {exit_code}"
        if failure_reason == "operator stop requested":
            append_runtime_event(
                paths,
                "codex_turn_failed",
                phase_id=current["phase"] if current else None,
                turn=current["turn"] if current else None,
                payload={
                    "summary": "runner recovery",
                    "exit_code": exit_code,
                    "turn_instance_id": turn_instance_id,
                },
                thread_id=thread_id,
            )
            return 0
        append_runtime_event(
            paths,
            "codex_turn_failed",
            phase_id=current["phase"] if current else None,
            turn=current["turn"] if current else None,
            payload={
                "summary": "runner recovery",
                "exit_code": exit_code,
                "turn_instance_id": turn_instance_id,
            },
            thread_id=thread_id,
        )
        append_runtime_event(
            paths,
            "runner_fault",
            phase_id=current["phase"] if current else None,
            turn=current["turn"] if current else None,
            payload={"reason": failure_reason, "turn_instance_id": turn_instance_id},
            thread_id=thread_id,
        )
        return 1

    append_runtime_event(
        paths,
        "codex_turn_completed",
        phase_id=current["phase"] if current else None,
        turn=current["turn"] if current else None,
        payload={
            "summary": "runner recovery",
            "exit_code": 0,
            "turn_instance_id": turn_instance_id,
        },
        thread_id=thread_id,
    )
    if current is not None and not turn_is_current(config_path, run_id, current, turn_instance_id):
        return 0
    try:
        outcome = extract_runner_event(capture.get("agent_messages", []), turn_instance_id)
        if current is not None:
            validate_turn_outcome(
                current_phase_for_projection(projection),
                current["turn"],
                outcome["event_type"],
            )
            append_runtime_event(
                paths,
                outcome["event_type"],
                phase_id=current["phase"],
                turn=current["turn"],
                payload=outcome["payload"],
                thread_id=thread_id,
            )
            append_runtime_event(
                paths,
                "turn_outcome_recorded",
                phase_id=current["phase"],
                turn=current["turn"],
                payload={
                    "outcome_event_type": outcome["event_type"],
                    "turn_instance_id": turn_instance_id,
                },
                thread_id=thread_id,
            )
        append_runtime_event(
            paths,
            "recovery_completed",
            phase_id=current["phase"] if current else None,
            turn=current["turn"] if current else None,
            payload={"reason": reason, "turn_instance_id": turn_instance_id},
            thread_id=thread_id,
        )
        refresh_projection(config_path, run_id)
        return 0
    except Exception as error:
        append_runtime_event(
            paths,
            "runner_fault",
            phase_id=current["phase"] if current else None,
            turn=current["turn"] if current else None,
            payload={"reason": str(error), "turn_instance_id": turn_instance_id},
            thread_id=thread_id,
        )
        return 1


def refresh_projection(config_path: Path, run_id: str) -> dict[str, Any]:
    config = load_config(config_path)
    paths = RuntimePaths(run_id)
    ensure_runtime_dirs()
    try:
        events = load_events(paths.events)
    except EventLogDecodeError as error:
        raise ValueError(str(error)) from error
    errors = validate_event_log(events, run_id)
    if errors:
        raise ValueError("; ".join(errors))
    projection = project_run(config, events, run_id)
    write_projection(paths.projection, projection)
    return projection


def append_runtime_event(
    paths: RuntimePaths,
    event_type: str,
    payload: dict[str, Any],
    phase_id: int | None = None,
    turn: str | None = None,
    thread_id: str | None = None,
) -> dict[str, Any]:
    return append_event(
        paths,
        {
            "run_id": paths.run_id,
            "sequence": 0,
            "at": now_iso(),
            "event_type": event_type,
            "phase_id": phase_id,
            "turn": turn,
            "thread_id": thread_id,
            "payload": payload,
        },
    )


def extract_runner_event(agent_messages: list[str], expected_turn_instance_id: str | None = None) -> dict[str, Any]:
    for message in reversed(agent_messages):
        for line in reversed(message.splitlines()):
            match = RUNNER_EVENT_PATTERN.search(line.strip())
            if not match:
                continue
            parsed = json.loads(match.group(1))
            if not isinstance(parsed, dict):
                break
            event_type = parsed.get("event_type")
            if not isinstance(event_type, str) or not event_type:
                raise ValueError("RUNNER_EVENT event_type must be a non-empty string")
            payload = parsed.get("payload", {})
            if not isinstance(payload, dict):
                raise ValueError("RUNNER_EVENT payload must be an object")
            if expected_turn_instance_id is not None:
                actual_turn_instance_id = payload.get("turn_instance_id")
                if actual_turn_instance_id != expected_turn_instance_id:
                    raise ValueError(
                        f"RUNNER_EVENT turn_instance_id must be {expected_turn_instance_id!r}"
                    )
            errors = validate_runner_outcome(event_type, payload)
            if errors:
                raise ValueError("; ".join(errors))
            return {"event_type": event_type, "payload": payload}
    raise ValueError("no RUNNER_EVENT line found in agent messages")


def build_recovery_prompt(
    projection: dict[str, Any],
    paths: RuntimePaths,
    reason: str,
    turn_instance_id: str | None,
) -> str:
    return f"""The automated durable phase runner hit a failure on the current turn.

Run id: {projection['run_id']}
Projection file: {paths.projection.resolve()}
Event log file: {paths.events.resolve()}
Current cursor: {projection.get('current')}
Failure reason: {reason}

Continue in the same persistent agent session when available. Re-read the current phase context if needed,
then finish the same turn honestly. Do not mutate any runner files directly.

If the prior agent turn already completed the work, do not redo the work. Emit the correct
typed RUNNER_EVENT for that already-completed turn.

Expected turn instance id: {turn_instance_id}
Your RUNNER_EVENT payload must include exactly "turn_instance_id":"{turn_instance_id}".

Your final line must be exactly one compact JSON marker:
RUNNER_EVENT: {{"event_type":"...","payload":{{...}}}}
"""


def maybe_reset_stuck_session(
    config_path: Path,
    run_id: str,
    projection: dict[str, Any],
) -> bool:
    current = projection.get("current")
    if not isinstance(current, dict):
        return False
    if projection.get("current_turn_instance_id"):
        return False
    threshold = fresh_session_cycle_threshold(projection)
    if threshold is None:
        return False
    paths = RuntimePaths(run_id)
    events = load_events(paths.events)
    cycle_count = qa_repair_cycles_since_last_reset(events, current["phase"])
    if cycle_count < threshold:
        return False
    reason = (
        f"fresh session after {cycle_count} same-phase QA/repair cycles "
        f"(threshold {threshold})"
    )
    append_runtime_event(
        paths,
        "session_reset",
        phase_id=current["phase"],
        turn=current["turn"],
        payload={
            "reason": reason,
            "cycle_count": cycle_count,
            "threshold": threshold,
        },
    )
    return True


def fresh_session_cycle_threshold(projection: dict[str, Any]) -> int | None:
    runner_control = projection.get("runner_control", {})
    if not isinstance(runner_control, dict):
        return None
    value = runner_control.get("fresh_session_after_qa_repair_cycles")
    if isinstance(value, int) and value > 0:
        return value
    return None


def qa_repair_cycles_since_last_reset(events: list[dict[str, Any]], phase_id: int) -> int:
    cycle_count = 0
    for event in reversed(events):
        if event.get("phase_id") != phase_id:
            continue
        event_type = event.get("event_type")
        if event_type == "session_reset":
            break
        if event_type in QA_REPAIR_COMPLETED_EVENTS:
            cycle_count += 1
    return cycle_count


def pending_recovery_reason(
    events: list[dict[str, Any]],
    current: dict[str, Any],
    current_turn_instance_id: str | None,
) -> str | None:
    if not isinstance(current_turn_instance_id, str) or not current_turn_instance_id:
        return None
    candidate_reason: str | None = None
    for event in reversed(events):
        if event.get("phase_id") != current["phase"] or event.get("turn") != current["turn"]:
            continue
        event_turn_instance_id = event.get("payload", {}).get("turn_instance_id")
        if event_turn_instance_id != current_turn_instance_id:
            continue
        event_type = event["event_type"]
        if event_type == "turn_outcome_recorded":
            return None
        if event_type == "prompt_selected":
            return candidate_reason
        if event_type == "runner_fault":
            reason = event.get("payload", {}).get("reason")
            candidate_reason = reason if isinstance(reason, str) and reason else "runner fault"
            continue
        if event_type == "codex_turn_completed":
            candidate_reason = "prior agent turn completed but outcome was not recorded"
            continue
        if event_type == "codex_turn_failed":
            candidate_reason = "prior agent turn failed and needs recovery"
            continue
    return None


def turn_is_current(
    config_path: Path,
    run_id: str,
    current: dict[str, Any],
    turn_instance_id: str | None,
) -> bool:
    projection = refresh_projection(config_path, run_id)
    latest_current = projection.get("current")
    return (
        isinstance(latest_current, dict)
        and latest_current.get("phase") == current["phase"]
        and latest_current.get("turn") == current["turn"]
        and projection.get("current_turn_instance_id") == turn_instance_id
    )


def current_phase_for_projection(projection: dict[str, Any]) -> dict[str, Any]:
    current = projection.get("current")
    if not isinstance(current, dict):
        raise ValueError("current phase is not set")
    for phase in projection["phases"]:
        if phase["id"] == current["phase"]:
            return phase
    raise ValueError(f"phase {current['phase']!r} is not present")


def config_path_for_run(run_id: str) -> Path:
    events = load_events(RuntimePaths(run_id).events)
    if not events:
        raise ValueError(f"run {run_id!r} does not exist")
    config_path = events[0]["payload"].get("config_path")
    if not isinstance(config_path, str):
        raise ValueError(f"run {run_id!r} does not record a config_path")
    return Path(config_path)


def should_stop_before_phase(projection: dict[str, Any]) -> bool:
    current = projection.get("current")
    stop_before = projection.get("runner_control", {}).get("stop_before_phase")
    return isinstance(current, dict) and isinstance(stop_before, int) and current["phase"] >= stop_before


def stop_before_phase_reason(projection: dict[str, Any]) -> str:
    current = projection["current"]
    label = projection.get("runner_control", {}).get("stop_reason") or "configured stop-before-phase gate"
    return f"{label}: current phase {current['phase']} {current['turn']} reached stop_before_phase"


def run_completion_handoff(projection: dict[str, Any], run_id: str) -> int:
    from completion_handoff import resume_completion_handoff_target

    return resume_completion_handoff_target(
        projection.get("runner_control", {}).get("completion_handoff"),
        polling_run_id=run_id,
    )


def new_run_id() -> str:
    return uuid.uuid4().hex[:12]


def now_iso() -> str:
    return datetime.now(timezone.utc).isoformat()

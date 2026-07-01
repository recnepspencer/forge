from __future__ import annotations

import json
import re
import time
import uuid
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

from codex_cli import run_codex
from config_schema import load_config
from event_log import append_event, load_events, validate_event_log
from projector import project_run, write_projection
from prompts import render_prompt
from runtime_paths import RuntimePaths
from transition_rules import validate_turn_outcome

RUNNER_EVENT_PATTERN = re.compile(r"RUNNER_EVENT:\s*(\{.*\})")


def start_run(
    config_path: Path,
    run_id: str | None,
    loop: bool,
    sleep_seconds: int,
    log_path: Path | None,
) -> int:
    config = load_config(config_path)
    active_run_id = run_id or new_run_id()
    paths = RuntimePaths(active_run_id)
    if paths.events.exists():
        raise ValueError(f"run {active_run_id!r} already exists")
    append_runtime_event(
        paths,
        "run_started",
        payload={"config_path": str(config_path.resolve())},
    )
    return drive_run(config_path, active_run_id, loop, sleep_seconds, log_path)


def resume_run(run_id: str, loop: bool, sleep_seconds: int, log_path: Path | None) -> int:
    config_path = config_path_for_run(run_id)
    append_runtime_event(
        RuntimePaths(run_id),
        "run_resumed",
        payload={"reason": "operator resume"},
    )
    return drive_run(config_path, run_id, loop, sleep_seconds, log_path)


def stop_run(run_id: str, reason: str) -> None:
    append_runtime_event(RuntimePaths(run_id), "run_stopped", payload={"reason": reason})
    refresh_projection(config_path_for_run(run_id), run_id)


def import_legacy_run(old_state_path: Path, config_path: Path, run_id: str | None) -> str:
    config = load_config(config_path)
    legacy = json.loads(old_state_path.read_text(encoding="utf-8-sig"))
    active_run_id = run_id or new_run_id()
    paths = RuntimePaths(active_run_id)
    if paths.events.exists():
        raise ValueError(f"run {active_run_id!r} already exists")

    append_runtime_event(
        paths,
        "run_started",
        payload={"config_path": str(config_path.resolve())},
    )
    append_runtime_event(
        paths,
        "legacy_imported",
        payload={
            "source_state_file": str(old_state_path.resolve()),
            "summary": f"imported legacy runner state from {old_state_path.name}",
        },
        thread_id=legacy.get("session", {}).get("thread_id"),
    )

    for phase in legacy.get("phases", []):
        import_phase_events(paths, phase, legacy.get("current"))

    if legacy.get("current") is None:
        append_runtime_event(paths, "run_completed", payload={"reason": "legacy import completed"})

    projection = refresh_projection(config_path, active_run_id)
    legacy_current = legacy.get("current")
    if projection["current"] != legacy_current:
        raise ValueError(
            f"import projected current {projection['current']!r} but legacy current was {legacy_current!r}"
        )
    return active_run_id


def drive_run(
    config_path: Path,
    run_id: str,
    loop: bool,
    sleep_seconds: int,
    log_path: Path | None,
) -> int:
    while True:
        projection = refresh_projection(config_path, run_id)
        if projection["completed_at"] is not None:
            return 0
        if projection["stopped"]:
            return 0
        current = projection.get("current")
        if current is None:
            append_runtime_event(
                RuntimePaths(run_id),
                "run_completed",
                payload={"reason": "all phases are complete"},
                thread_id=projection["session"]["thread_id"],
            )
            refresh_projection(config_path, run_id)
            return 0
        if should_stop_before_phase(projection):
            append_runtime_event(
                RuntimePaths(run_id),
                "run_stopped",
                payload={"reason": stop_before_phase_reason(projection)},
                thread_id=projection["session"]["thread_id"],
            )
            refresh_projection(config_path, run_id)
            return 0

        status = run_single_turn(config_path, run_id, log_path)
        if status != 0 or not loop:
            return status
        time.sleep(sleep_seconds)


def run_single_turn(config_path: Path, run_id: str, log_path: Path | None) -> int:
    config = load_config(config_path)
    projection = refresh_projection(config_path, run_id)
    current = projection["current"]
    paths = RuntimePaths(run_id)
    prompt = render_prompt(config, projection, config_path, paths.projection, paths.events)
    append_runtime_event(
        paths,
        "prompt_selected",
        phase_id=current["phase"],
        turn=current["turn"],
        payload={"summary": f"phase {current['phase']} {current['turn']}"},
        thread_id=projection["session"]["thread_id"],
    )

    exit_code, capture = run_codex(projection, prompt, log_path or paths.log)
    thread_id = capture.get("thread_id") or projection["session"]["thread_id"]
    event_payload = {
        "summary": f"phase {current['phase']} {current['turn']}",
        "agent_message_count": len(capture.get("agent_messages", [])),
    }
    if exit_code != 0:
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
            payload={"reason": f"codex exited with {exit_code}"},
            thread_id=thread_id,
        )
        return run_recovery_turn(config_path, run_id, log_path, f"codex exited with {exit_code}")

    append_runtime_event(
        paths,
        "codex_turn_completed",
        phase_id=current["phase"],
        turn=current["turn"],
        payload={**event_payload, "exit_code": 0},
        thread_id=thread_id,
    )
    try:
        outcome = extract_runner_event(capture.get("agent_messages", []))
    except ValueError as error:
        append_runtime_event(
            paths,
            "runner_fault",
            phase_id=current["phase"],
            turn=current["turn"],
            payload={"reason": str(error)},
            thread_id=thread_id,
        )
        return run_recovery_turn(config_path, run_id, log_path, str(error))

    validate_turn_outcome(current["turn"], outcome["event_type"])
    append_runtime_event(
        paths,
        outcome["event_type"],
        phase_id=current["phase"],
        turn=current["turn"],
        payload=outcome["payload"],
        thread_id=thread_id,
    )
    refresh_projection(config_path, run_id)
    return 0


def run_recovery_turn(config_path: Path, run_id: str, log_path: Path | None, reason: str) -> int:
    config = load_config(config_path)
    projection = refresh_projection(config_path, run_id)
    current = projection["current"]
    paths = RuntimePaths(run_id)
    append_runtime_event(
        paths,
        "recovery_requested",
        phase_id=current["phase"] if current else None,
        turn=current["turn"] if current else None,
        payload={"reason": reason},
        thread_id=projection["session"]["thread_id"],
    )
    prompt = build_recovery_prompt(projection, paths, reason)
    exit_code, capture = run_codex(projection, prompt, log_path or paths.log)
    thread_id = capture.get("thread_id") or projection["session"]["thread_id"]
    if exit_code != 0:
        append_runtime_event(
            paths,
            "codex_turn_failed",
            phase_id=current["phase"] if current else None,
            turn=current["turn"] if current else None,
            payload={"summary": "runner recovery", "exit_code": exit_code},
            thread_id=thread_id,
        )
        return 1

    append_runtime_event(
        paths,
        "codex_turn_completed",
        phase_id=current["phase"] if current else None,
        turn=current["turn"] if current else None,
        payload={"summary": "runner recovery", "exit_code": 0},
        thread_id=thread_id,
    )
    outcome = extract_runner_event(capture.get("agent_messages", []))
    if current is not None:
        validate_turn_outcome(current["turn"], outcome["event_type"])
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
        "recovery_completed",
        phase_id=current["phase"] if current else None,
        turn=current["turn"] if current else None,
        payload={"reason": reason},
        thread_id=thread_id,
    )
    refresh_projection(config_path, run_id)
    return 0


def refresh_projection(config_path: Path, run_id: str) -> dict[str, Any]:
    config = load_config(config_path)
    paths = RuntimePaths(run_id)
    events = load_events(paths.events)
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


def extract_runner_event(agent_messages: list[str]) -> dict[str, Any]:
    for message in reversed(agent_messages):
        for line in reversed(message.splitlines()):
            match = RUNNER_EVENT_PATTERN.search(line.strip())
            if not match:
                continue
            parsed = json.loads(match.group(1))
            if not isinstance(parsed, dict):
                break
            payload = parsed.get("payload", {})
            if not isinstance(payload, dict):
                raise ValueError("RUNNER_EVENT payload must be an object")
            return {"event_type": parsed["event_type"], "payload": payload}
    raise ValueError("no RUNNER_EVENT line found in agent messages")


def build_recovery_prompt(projection: dict[str, Any], paths: RuntimePaths, reason: str) -> str:
    return f"""The automated durable phase runner hit a failure on the current turn.

Run id: {projection['run_id']}
Projection file: {paths.projection.resolve()}
Event log file: {paths.events.resolve()}
Current cursor: {projection.get('current')}
Failure reason: {reason}

Continue in the same persistent Codex thread. Re-read the current phase context if needed,
then finish the same turn honestly. Do not mutate any runner files directly.

Your final line must be exactly one compact JSON marker:
RUNNER_EVENT: {{"event_type":"...","payload":{{...}}}}
"""


def config_path_for_run(run_id: str) -> Path:
    events = load_events(RuntimePaths(run_id).events)
    if not events:
        raise ValueError(f"run {run_id!r} does not exist")
    config_path = events[0]["payload"].get("config_path")
    if not isinstance(config_path, str):
        raise ValueError(f"run {run_id!r} does not record a config_path")
    return Path(config_path)


def import_phase_events(
    paths: RuntimePaths,
    phase: dict[str, Any],
    current: dict[str, Any] | None,
) -> None:
    phase_id = int(phase["id"])
    status = phase.get("status")
    qa_status = phase.get("qa_status")
    notes = phase.get("notes", {})
    if status == "not_started" and qa_status == "not_started":
        return
    current_turn = current["turn"] if isinstance(current, dict) and current.get("phase") == phase_id else None
    append_runtime_event(paths, "plan_posted", phase_id=phase_id, turn="plan", payload=phase_payload(notes))
    if current_turn == "implement":
        return
    append_runtime_event(
        paths,
        "implementation_completed",
        phase_id=phase_id,
        turn="implement",
        payload=phase_payload(notes),
    )
    if current_turn == "review":
        return
    if status == "regressed" and qa_status == "failed":
        append_runtime_event(
            paths, "review_failed", phase_id=phase_id, turn="review", payload=phase_payload(notes)
        )
        if current_turn == "repair":
            return
        if current_turn is not None:
            raise ValueError(f"legacy current turn {current_turn!r} is incompatible with failed phase {phase_id}")
        return
    if qa_status == "needed":
        if current_turn not in {None, "review"}:
            raise ValueError(f"legacy current turn {current_turn!r} is incompatible with needed QA on phase {phase_id}")
        return
    append_runtime_event(paths, "review_passed", phase_id=phase_id, turn="review", payload=phase_payload(notes))
    if current_turn == "test_review":
        return
    if current_turn in {"test_repair_plan", "test_repair_implement"}:
        append_runtime_event(
            paths, "test_review_failed", phase_id=phase_id, turn="test_review", payload=phase_payload(notes)
        )
        if current_turn == "test_repair_plan":
            return
        append_runtime_event(
            paths,
            "test_repair_plan_posted",
            phase_id=phase_id,
            turn="test_repair_plan",
            payload=phase_payload(notes),
        )
        return
    append_runtime_event(paths, "test_review_passed", phase_id=phase_id, turn="test_review", payload=phase_payload(notes))
    if current_turn == "code_quality_review":
        return
    if current_turn is not None and current_turn not in {"plan"}:
        raise ValueError(f"legacy current turn {current_turn!r} is incompatible with passed phase {phase_id}")
    append_runtime_event(
        paths,
        "code_quality_review_passed",
        phase_id=phase_id,
        turn="code_quality_review",
        payload=phase_payload(notes),
    )


def phase_payload(notes: dict[str, Any]) -> dict[str, Any]:
    payload_notes = {}
    for bucket in ("plan", "done", "remaining", "findings", "verification"):
        value = notes.get(bucket, [])
        if isinstance(value, list) and value:
            payload_notes[bucket] = [str(entry) for entry in value]
    return {"notes": payload_notes}


def should_stop_before_phase(projection: dict[str, Any]) -> bool:
    current = projection.get("current")
    stop_before = projection.get("runner_control", {}).get("stop_before_phase")
    return isinstance(current, dict) and isinstance(stop_before, int) and current["phase"] >= stop_before


def stop_before_phase_reason(projection: dict[str, Any]) -> str:
    current = projection["current"]
    label = projection.get("runner_control", {}).get("stop_reason") or "configured stop-before-phase gate"
    return f"{label}: current phase {current['phase']} {current['turn']} reached stop_before_phase"


def new_run_id() -> str:
    return uuid.uuid4().hex[:12]


def now_iso() -> str:
    return datetime.now(timezone.utc).isoformat()

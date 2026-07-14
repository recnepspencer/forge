from __future__ import annotations

import json
from pathlib import Path
from typing import Any

from config_schema import load_config
from runtime_paths import RuntimePaths, acquire_active_run_lock


def import_legacy_run(
    old_state_path: Path,
    config_path: Path,
    run_id: str | None,
    append_runtime_event,
    refresh_projection,
    new_run_id,
) -> str:
    load_config(config_path)
    legacy = json.loads(old_state_path.read_text(encoding="utf-8-sig"))
    active_run_id = run_id or new_run_id()
    paths = RuntimePaths(active_run_id)
    with acquire_active_run_lock(paths):
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
            import_phase_events(paths, phase, legacy.get("current"), append_runtime_event)

        if legacy.get("current") is None:
            append_runtime_event(
                paths,
                "run_completed",
                payload={"reason": "legacy import completed"},
            )

        projection = refresh_projection(config_path, active_run_id)
        legacy_current = legacy.get("current")
        if not imported_current_is_compatible(projection, legacy_current):
            raise ValueError(
                f"import projected current {projection['current']!r} but legacy current was {legacy_current!r}"
            )
        return active_run_id


def import_phase_events(
    paths: RuntimePaths,
    phase: dict[str, Any],
    current: dict[str, Any] | None,
    append_runtime_event,
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


def imported_current_is_compatible(
    projection: dict[str, Any], legacy_current: dict[str, Any] | None
) -> bool:
    if projection["current"] == legacy_current:
        return True
    if not isinstance(legacy_current, dict):
        return False
    legacy_phase = phase_by_id(projection, legacy_current.get("phase"))
    if legacy_phase is None:
        return False
    if legacy_phase["status"] != "complete" or legacy_phase["qa_status"] != "passed":
        return False
    first_unfinished = first_unfinished_phase(projection)
    return projection["current"] == first_unfinished


def phase_by_id(projection: dict[str, Any], phase_id: Any) -> dict[str, Any] | None:
    for phase in projection["phases"]:
        if phase["id"] == phase_id:
            return phase
    return None


def first_unfinished_phase(projection: dict[str, Any]) -> dict[str, Any] | None:
    for phase in projection["phases"]:
        if phase["status"] != "complete" or phase["qa_status"] != "passed":
            return {"phase": phase["id"], "turn": "plan"}
    return None

from __future__ import annotations

from datetime import datetime
from fnmatch import fnmatch
from pathlib import Path
import time
from typing import Any, Callable, Iterable

from runner.authority.config import load_config
from runner.authority.events import PHASE_PROGRESS_EVENTS, load_events
from runner.authority.run_identity import RuntimePaths
from runner.graph_runtime.runtime_lane import append_runtime_event, refresh_projection
from runner.graph_runtime.continuation import plan_recovery_attempt
from runner.graph_runtime.recovery_disposition import execute_exhausted_recovery_disposition
from runner.phase_programs.policy_bindings import (
    qualifying_edit_policy,
    stall_signal_policy,
)
from runner.recovery.failure_families import NO_EDIT_STALL_FAMILY, SAME_PHASE_LOOP_EXCEEDED_FAMILY

QA_REPAIR_COMPLETED_EVENTS = {
    "repair_completed",
    "test_repair_completed",
    "code_quality_repair_completed",
}


def apply_preflight_runtime_guards(
    config_path: Path,
    run_id: str,
    projection: dict[str, Any],
) -> bool:
    current = projection.get("current")
    if not isinstance(current, dict) or projection.get("current_turn_instance_id"):
        return False
    config = load_config(config_path)
    paths = RuntimePaths(run_id)
    events = load_events(paths.events)
    if maybe_handle_no_edit_stall(config, paths, projection, events):
        refresh_projection(config_path, run_id)
        return True
    if maybe_handle_same_phase_loop(config, paths, projection, events):
        refresh_projection(config_path, run_id)
        return True
    return False


def maybe_reset_stuck_session(
    config_path: Path,
    run_id: str,
    projection: dict[str, Any],
) -> bool:
    return apply_preflight_runtime_guards(config_path, run_id, projection)


def load_loop_reset_policy(config_path: Path, projection: dict[str, Any]):
    current = projection.get("current")
    if not isinstance(current, dict):
        return None
    config = load_config(config_path)
    paths = RuntimePaths(projection["run_id"] if "run_id" in projection else "")
    if not paths.run_id:
        return None
    events = load_events(paths.events)
    return same_phase_loop_family(config, events, current["phase"])


def maybe_handle_no_edit_stall(
    config: dict[str, Any],
    paths: RuntimePaths,
    projection: dict[str, Any],
    events: list[dict[str, Any]],
) -> bool:
    current = projection.get("current")
    if not isinstance(current, dict):
        return False
    signal_policy = stall_signal_policy(config, NO_EDIT_STALL_FAMILY)
    if signal_policy is None or not signal_policy.enabled:
        return False
    threshold_minutes = signal_policy.minutes_without_qualifying_edit
    if threshold_minutes is None:
        return False
    if guard_fault_is_open(events, current["phase"], current["turn"], NO_EDIT_STALL_FAMILY):
        return False
    last_edit_at = latest_qualifying_edit_timestamp(config)
    if last_edit_at is not None and (time.time() - last_edit_at) < (threshold_minutes * 60):
        return False
    reason = no_edit_stall_reason(last_edit_at, threshold_minutes)
    return admit_preflight_failure(config, paths, projection, events, reason, NO_EDIT_STALL_FAMILY)


def build_inflight_no_progress_watchdog(
    config: dict[str, Any],
    run_id: str,
    current: dict[str, Any],
    turn_instance_id: str | None,
) -> Callable[[], dict[str, str] | None] | None:
    signal_policy = stall_signal_policy(config, NO_EDIT_STALL_FAMILY)
    if signal_policy is None or not signal_policy.enabled:
        return None
    threshold_minutes = signal_policy.minutes_without_phase_progress
    if threshold_minutes is None:
        return None
    paths = RuntimePaths(run_id)

    def watch() -> dict[str, str] | None:
        return inflight_no_progress_fault(config, paths, current, turn_instance_id, threshold_minutes)

    return watch


def inflight_no_progress_fault(
    config: dict[str, Any],
    paths: RuntimePaths,
    current: dict[str, Any],
    turn_instance_id: str | None,
    threshold_minutes: int,
) -> dict[str, str] | None:
    prompt_selected_at = prompt_selected_timestamp(paths, current["phase"], current["turn"], turn_instance_id)
    if prompt_selected_at is None:
        return None
    last_edit_at = latest_qualifying_edit_timestamp(config)
    progress_witness_at = prompt_selected_at
    if last_edit_at is not None and last_edit_at > progress_witness_at:
        progress_witness_at = last_edit_at
    if (time.time() - progress_witness_at) < (threshold_minutes * 60):
        return None
    return {
        "reason": inflight_no_progress_reason(prompt_selected_at, last_edit_at, threshold_minutes),
        "failure_family": NO_EDIT_STALL_FAMILY,
    }


def maybe_handle_same_phase_loop(
    config: dict[str, Any],
    paths: RuntimePaths,
    projection: dict[str, Any],
    events: list[dict[str, Any]],
) -> bool:
    current = projection.get("current")
    if not isinstance(current, dict):
        return False
    loop_family = same_phase_loop_family(config, events, current["phase"])
    if loop_family is None:
        return False
    reason = (
        f"same-phase loop exceeded for family {loop_family.family_name} "
        f"at threshold {loop_family.threshold}"
    )
    return admit_preflight_failure(config, paths, projection, events, reason, SAME_PHASE_LOOP_EXCEEDED_FAMILY)


def admit_preflight_failure(
    config: dict[str, Any],
    paths: RuntimePaths,
    projection: dict[str, Any],
    events: list[dict[str, Any]],
    reason: str,
    failure_family: str,
) -> bool:
    """Use the ordinary recovery planner for failures detected before a turn starts."""
    current = projection["current"]
    thread_id = projection.get("session", {}).get("thread_id")
    append_runtime_event(
        paths,
        "runner_fault",
        phase_id=current["phase"],
        turn=current["turn"],
        payload={
            "reason": reason,
            "failure_family": failure_family,
        },
        thread_id=thread_id,
    )
    recovery = plan_recovery_attempt(
        config=config,
        events=events,
        phase_id=current["phase"],
        turn=current["turn"],
        reason=reason,
        failure_family=failure_family,
        turn_instance_id=None,
    )
    if recovery.exhausted_disposition is None:
        return False
    execute_exhausted_recovery_disposition(paths, current, recovery, thread_id)
    return True


def same_phase_loop_family(config: dict[str, Any], events: list[dict[str, Any]], phase_id: int):
    policies = config.get("loop_escalation", {}).get("families", {})
    for family_name, family_policy in policies.items():
        threshold = family_policy["threshold"]
        turns = set(family_policy["turns"])
        if same_phase_family_progress_count(events, phase_id, turns) >= threshold:
            from runner.phase_programs.policy_bindings import LoopEscalationFamilyPolicy

            return LoopEscalationFamilyPolicy(
                family_name=family_name,
                turns=tuple(family_policy["turns"]),
                threshold=threshold,
                action=family_policy["action"],
            )
    return None


def same_phase_family_progress_count(
    events: list[dict[str, Any]],
    phase_id: int,
    turns: set[str],
) -> int:
    count = 0
    for event in reversed(events):
        if event.get("phase_id") != phase_id:
            continue
        event_type = event.get("event_type")
        if event_type == "session_reset":
            break
        if event_type in PHASE_PROGRESS_EVENTS and event.get("turn") in turns:
            count += 1
    return count


def same_phase_loop_attempt_count(events: list[dict[str, Any]], phase_id: int) -> int:
    count = 0
    for event in events:
        if event.get("phase_id") != phase_id:
            continue
        if event.get("event_type") != "recovery_requested":
            continue
        payload = event.get("payload", {})
        if payload.get("failure_family") == SAME_PHASE_LOOP_EXCEEDED_FAMILY:
            count += 1
    return count


def guard_fault_is_open(
    events: list[dict[str, Any]],
    phase_id: int,
    turn: str,
    failure_family: str,
) -> bool:
    for event in reversed(events):
        event_type = event.get("event_type")
        if event_type in PHASE_PROGRESS_EVENTS:
            return False
        if event_type == "prompt_selected" and event.get("phase_id") == phase_id and event.get("turn") == turn:
            return False
        if event_type != "runner_fault":
            continue
        if event.get("phase_id") != phase_id or event.get("turn") != turn:
            continue
        if event.get("payload", {}).get("failure_family") == failure_family:
            return True
    return False


def latest_qualifying_edit_timestamp(config: dict[str, Any]) -> float | None:
    edit_policy = qualifying_edit_policy(config)
    if edit_policy is None or not edit_policy.include:
        return None
    cwd = Path(config["project"]["cwd"]).resolve()
    latest_mtime: float | None = None
    for path in qualifying_paths(cwd, edit_policy.include, edit_policy.exclude):
        try:
            mtime = path.stat().st_mtime
        except OSError:
            continue
        if latest_mtime is None or mtime > latest_mtime:
            latest_mtime = mtime
    return latest_mtime


def prompt_selected_timestamp(
    paths: RuntimePaths,
    phase_id: int,
    turn: str,
    turn_instance_id: str | None,
) -> float | None:
    for event in reversed(load_events(paths.events)):
        if event.get("event_type") != "prompt_selected":
            continue
        if event.get("phase_id") != phase_id or event.get("turn") != turn:
            continue
        payload = event.get("payload", {})
        if turn_instance_id is not None and payload.get("turn_instance_id") != turn_instance_id:
            continue
        recorded_at = event.get("at")
        if not isinstance(recorded_at, str) or not recorded_at:
            return None
        return datetime.fromisoformat(recorded_at).timestamp()
    return None


def qualifying_paths(cwd: Path, includes: Iterable[str], excludes: Iterable[str]) -> Iterable[Path]:
    seen: set[Path] = set()
    for pattern in includes:
        for path in cwd.glob(pattern):
            if path in seen or not path.is_file():
                continue
            relative_path = path.resolve().relative_to(cwd).as_posix()
            if any(fnmatch(relative_path, exclude) for exclude in excludes):
                continue
            seen.add(path)
            yield path


def no_edit_stall_reason(last_edit_at: float | None, threshold_minutes: int) -> str:
    if last_edit_at is None:
        return (
            "no qualifying edit detected within configured scope "
            f"for {threshold_minutes} minutes"
        )
    minutes_without_edit = int((time.time() - last_edit_at) // 60)
    return (
        "no qualifying edit detected for "
        f"{minutes_without_edit} minutes (threshold {threshold_minutes})"
    )


def inflight_no_progress_reason(
    prompt_selected_at: float,
    last_edit_at: float | None,
    threshold_minutes: int,
) -> str:
    if last_edit_at is None or last_edit_at <= prompt_selected_at:
        minutes_since_prompt = int((time.time() - prompt_selected_at) // 60)
        return (
            "turn made no qualifying progress after prompt selection for "
            f"{minutes_since_prompt} minutes (threshold {threshold_minutes})"
        )
    minutes_since_edit = int((time.time() - last_edit_at) // 60)
    return (
        "turn made no qualifying progress after its last edit for "
        f"{minutes_since_edit} minutes (threshold {threshold_minutes})"
    )


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

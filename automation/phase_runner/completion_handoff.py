from __future__ import annotations

from pathlib import Path
from typing import Any

from orchestrator import config_path_for_run, refresh_projection, resume_run_with_reason
from runtime_paths import RuntimePaths


def resume_completion_handoff_target(
    completion_handoff: dict[str, Any] | None,
    *,
    polling_run_id: str | None = None,
) -> int:
    if not isinstance(completion_handoff, dict):
        return 0
    target_run_id = completion_handoff.get("next_run_id")
    if not isinstance(target_run_id, str) or not target_run_id:
        raise ValueError("runner_control.completion_handoff.next_run_id is required")
    if not RuntimePaths(target_run_id).events.exists():
        raise ValueError(f"completion handoff target does not exist: {target_run_id}")
    next_projection = refresh_projection(
        config_path_for_run(target_run_id),
        target_run_id,
    )
    if next_projection["completed_at"] is not None:
        return 0
    if not next_projection["stopped"]:
        return 0
    return resume_run_with_reason(
        target_run_id,
        completion_handoff.get("loop", True),
        completion_handoff.get("sleep_seconds", 30),
        completion_handoff_log_path(completion_handoff),
        resume_reason(completion_handoff, polling_run_id),
    )


def completion_handoff_log_path(completion_handoff: dict[str, Any]) -> Path | None:
    log = completion_handoff.get("log")
    if log is None:
        return None
    if not isinstance(log, str) or not log:
        raise ValueError("runner_control.completion_handoff.log must be a non-empty string when present")
    return Path(log)


def resume_reason(
    completion_handoff: dict[str, Any],
    polling_run_id: str | None,
) -> str:
    explicit_reason = completion_handoff.get("reason")
    if isinstance(explicit_reason, str) and explicit_reason:
        return explicit_reason
    if isinstance(polling_run_id, str) and polling_run_id:
        return f"completion handoff from {polling_run_id}"
    return "completion handoff"

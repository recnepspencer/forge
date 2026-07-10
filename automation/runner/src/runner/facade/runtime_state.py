from __future__ import annotations

from pathlib import Path
from typing import Any

from runner.authority.run_identity import RuntimePaths
from runner.authority.run_identity.runtime_paths import runtime_lane_descriptions
from runner.graph_runtime.completion_runtime import resume_completion_handoff_target
from runner.graph_runtime.resume_runtime import resume_run_with_reason
from runner.graph_runtime.runtime_lane import (
    append_runtime_event,
    append_runtime_event_if_plan_version,
    initialize_runtime_events,
    config_path_for_run,
    load_admitted_projection_inputs as _load_admitted_projection_inputs,
    refresh_projection,
    refresh_projection_for_run,
    should_stop_before_phase,
    stop_before_phase_reason,
)

def current_phase_for_projection(projection: dict[str, Any]) -> dict[str, Any]:
    current = projection.get("current")
    if not isinstance(current, dict):
        raise ValueError("current phase is not set")
    for phase in projection["phases"]:
        if phase["id"] == current["phase"]:
            return phase
    raise ValueError(f"phase {current['phase']!r} is not present")


def runtime_artifact_surface(run_id: str) -> dict[str, object]:
    lanes = runtime_lane_descriptions(run_id)
    authority = [lane for lane in lanes if lane["lane"] == "events"]
    derived = [lane for lane in lanes if lane["lane"] in {"projections", "instantiations", "notifications", "telegram", "logs"}]
    continuity = [lane for lane in lanes if lane["lane"] == "checkpoints"]
    process_control = [lane for lane in lanes if lane["lane"] == "locks"]
    return {
        "authority": authority,
        "derived": derived,
        "continuity": continuity,
        "process_control": process_control,
    }


def run_completion_handoff(projection: dict[str, Any], run_id: str) -> int:
    return resume_completion_handoff_target(
        projection.get("runner_control", {}).get("completion_handoff"),
        refresh_projection,
        config_path_for_run,
        resume_run_with_reason,
        RuntimePaths,
        polling_run_id=run_id,
    )

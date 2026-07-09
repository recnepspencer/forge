from __future__ import annotations

import time
from pathlib import Path

from runner.authority.config import load_config
from runner.authority.run_identity import RuntimePaths
from runner.graph_runtime.completion_runtime import resume_completion_handoff_target
from runner.graph_runtime.compiled_graph import execute_graph_plan
from runner.graph_runtime.graph_plan import lower_graph_execution_plan
from runner.graph_runtime.recovery_runtime import apply_preflight_runtime_guards
from runner.graph_runtime.resume_runtime import resume_run_with_reason
from runner.graph_runtime.runtime_lane import (
    append_runtime_event,
    config_path_for_run,
    refresh_projection,
    should_stop_before_phase,
    stop_before_phase_reason,
)
from runner.graph_runtime.state import TURN_TRANSITION_KEY, build_graph_state


def drive_graph_run(
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
            return resume_completion_handoff_target(
                refresh_projection(config_path, run_id).get("runner_control", {}).get("completion_handoff"),
                refresh_projection,
                config_path_for_run,
                resume_run_with_reason,
                RuntimePaths,
                polling_run_id=run_id,
            )
        if should_stop_before_phase(projection):
            append_runtime_event(
                RuntimePaths(run_id),
                "run_stopped",
                payload={"reason": stop_before_phase_reason(projection)},
                thread_id=projection["session"]["thread_id"],
            )
            refresh_projection(config_path, run_id)
            return 0
        if apply_preflight_runtime_guards(config_path, run_id, projection):
            continue
        status = execute_graph_turn(config_path, run_id, log_path)
        if status != 0 or not loop:
            return status
        time.sleep(sleep_seconds)


def execute_graph_turn(config_path: Path, run_id: str, log_path: Path | None) -> int:
    load_config(config_path)
    graph_plan = lower_graph_execution_plan(run_id)
    graph_state = build_graph_state(
        run_id=run_id,
        config_path=config_path,
        log_path=log_path,
    )
    return execute_graph_plan(graph_plan, graph_state)[TURN_TRANSITION_KEY].result_code

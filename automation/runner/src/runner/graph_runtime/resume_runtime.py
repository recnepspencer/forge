from __future__ import annotations

from pathlib import Path

from runner.authority.run_identity import RuntimePaths, acquire_active_run_lock, clear_stop_requested


def resume_run_with_reason(
    run_id: str,
    loop: bool,
    sleep_seconds: int,
    log_path: Path | None,
    reason: str,
) -> int:
    from runner.graph_runtime.runtime_lane import append_runtime_event, config_path_for_run

    config_path = config_path_for_run(run_id)
    from runner.graph_runtime.orchestrator import drive_graph_run

    paths = RuntimePaths(run_id)
    with acquire_active_run_lock(paths):
        clear_stop_requested(paths)
        append_runtime_event(paths, "run_resumed", payload={"reason": reason})
        return drive_graph_run(config_path, run_id, loop, sleep_seconds, log_path)

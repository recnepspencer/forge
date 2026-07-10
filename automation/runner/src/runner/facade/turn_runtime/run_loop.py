from __future__ import annotations

from pathlib import Path

from runner.graph_runtime.orchestrator import drive_graph_run


def drive_run(
    config_path: Path,
    run_id: str,
    loop: bool,
    sleep_seconds: int,
    log_path: Path | None,
) -> int:
    return drive_graph_run(config_path, run_id, loop, sleep_seconds, log_path)

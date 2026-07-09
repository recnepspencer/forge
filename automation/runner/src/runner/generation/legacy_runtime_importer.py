from __future__ import annotations

from runner.authority.events.run_authority import load_admitted_run_projection_inputs
from runner.authority.run_identity import RuntimePaths, ensure_runtime_dirs
from runner.authority.run_identity.legacy_runtime_migration import (
    import_legacy_runtime_run,
    legacy_runtime_run_exists,
)
from runner.authority.run_identity.runtime_paths import RUNTIME_SUBDIRECTORIES


def import_legacy_runtime_authority(run_id: str) -> str:
    paths = RuntimePaths(run_id)
    ensure_runtime_dirs()
    if paths.events.exists():
        raise ValueError(f"run {run_id!r} already exists in canonical runtime")
    if not legacy_runtime_run_exists(run_id, RUNTIME_SUBDIRECTORIES):
        raise ValueError(f"legacy runtime for run {run_id!r} does not exist")
    imported = import_legacy_runtime_run(run_id, paths.runtime_root, RUNTIME_SUBDIRECTORIES)
    if not imported or not paths.events.exists():
        raise ValueError(f"legacy runtime for run {run_id!r} did not produce a canonical event log")
    load_admitted_run_projection_inputs(run_id)
    return run_id

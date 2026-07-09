from __future__ import annotations

import os
import shutil
from pathlib import Path


def runtime_root_from_env(env_var: str, default: str) -> Path:
    override = os.environ.get(env_var)
    if not override:
        return Path(default)
    return Path(override)


_LEGACY_RUNTIME_ROOT = runtime_root_from_env(
    "AUTOMATION_RUNNER_LEGACY_RUNTIME_ROOT",
    "automation/phase_runner/runtime",
)


def import_legacy_runtime_run(
    run_id: str,
    canonical_runtime_root: Path,
    runtime_subdirectories: tuple[str, ...],
) -> bool:
    imported = False
    for relative in runtime_subdirectories:
        legacy_path = _LEGACY_RUNTIME_ROOT / relative / run_id_path_component(relative, run_id)
        canonical_path = canonical_runtime_root / relative / run_id_path_component(relative, run_id)
        if not legacy_path.exists() or canonical_path.exists():
            continue
        canonical_path.parent.mkdir(parents=True, exist_ok=True)
        if legacy_path.is_dir():
            shutil.copytree(legacy_path, canonical_path)
            imported = True
            continue
        shutil.copy2(legacy_path, canonical_path)
        imported = True
    return imported


def legacy_runtime_run_exists(
    run_id: str,
    runtime_subdirectories: tuple[str, ...],
) -> bool:
    for relative in runtime_subdirectories:
        legacy_path = _LEGACY_RUNTIME_ROOT / relative / run_id_path_component(relative, run_id)
        if legacy_path.exists():
            return True
    return False


def run_id_path_component(relative: str, run_id: str) -> str:
    if relative in {"checkpoints", "instantiations"}:
        return run_id
    if relative in {"events", "notifications", "logs"}:
        return f"{run_id}.jsonl"
    return f"{run_id}.json"

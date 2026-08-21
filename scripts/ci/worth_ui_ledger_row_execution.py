from __future__ import annotations

import json
import os
import subprocess
import sys
import time
from pathlib import Path
from typing import Any, Callable

from worth_ui_ledger_row_cache import RowEvidenceCache


class CachedEvidenceRejected(RuntimeError):
    """A restored receipt is authentic but no longer matches the row mapping."""


def execute_or_restore(
    row: dict[str, str],
    candidate: Path,
    cache: RowEvidenceCache,
    claim: str,
    execute: Callable[[str, Path | None], dict[str, Any]],
    finalize: Callable[[dict[str, Any]], dict[str, Any]] = lambda result: result,
    *,
    restore: bool = True,
) -> dict[str, Any]:
    requirement = row["requirement"]
    started = time.perf_counter_ns()
    result = cache.restore(requirement, row["exact_command"], claim) if restore else None
    if result is not None:
        try:
            current = finalize(result)
        except CachedEvidenceRejected:
            result = None
        else:
            print(
                completion_telemetry(requirement, "reuse", started),
                flush=True,
            )
            return current
    print(f"[row:start] {requirement} disposition=execute", flush=True)
    result = execute(row["exact_command"], candidate)
    result = finalize(result)
    cache.retain(requirement, row["exact_command"], claim, result)
    print(
        completion_telemetry(requirement, "execute", started),
        flush=True,
    )
    return result


def completion_telemetry(requirement: str, disposition: str, started: int) -> str:
    duration_ms = max(1, (time.perf_counter_ns() - started + 999_999) // 1_000_000)
    return (
        f"[row:complete] {requirement} disposition={disposition} "
        f"posture=passed duration_ms={duration_ms}"
    )


def run_row(
    root: Path,
    command_text: str,
    candidate_ledger: Path | None = None,
) -> dict[str, object]:
    requirement = command_requirement(command_text)
    started = time.perf_counter()
    print(f"[row:start] {requirement}", flush=True)
    environment = dict(os.environ)
    if candidate_ledger is not None:
        environment["WORTH_UI_MILESTONE_3141_LEDGER"] = str(
            candidate_ledger.resolve()
        )
    words = command_text.split()
    if words and words[0] == "python":
        words[0] = sys.executable
    completed = subprocess.run(
        words,
        cwd=root,
        env=environment,
        stdout=subprocess.PIPE,
        text=True,
        check=False,
    )
    if completed.returncode != 0:
        sys.stderr.write(completed.stdout)
        raise RuntimeError(f"ledger proof failed with {completed.returncode}")
    result = json.loads(completed.stdout.splitlines()[-1])
    control = result.get("hostile_control")
    control_ms = control.get("test_duration_ms") if isinstance(control, dict) else None
    print(
        f"[row:pass] {requirement} elapsed={time.perf_counter() - started:.2f}s "
        f"main={result.get('test_duration_ms')}ms control={control_ms}ms",
        flush=True,
    )
    return result


def command_requirement(command_text: str) -> str:
    words = command_text.split()
    return words[words.index("--requirement") + 1]

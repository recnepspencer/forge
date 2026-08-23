from __future__ import annotations

import os
import subprocess
import sys
import time
from pathlib import Path
from typing import Any

from worth_ui_ledger_execution_binding import (
    GovernedExecutionSnapshot,
    causal_artifact_dependencies,
    digest_json,
    execution_binding,
)
from worth_ui_ledger_execution_observation import (
    ExecutionReference,
    create_observation,
    reused_reference,
)
from worth_ui_ledger_execution_observation_store import read_for_binding, stage


def timed_execution(
    command: list[str],
    root: Path,
    revision: str,
    state_digest: str,
    role: str,
    requirement: str | None = None,
) -> tuple[subprocess.CompletedProcess[str], int, dict[str, object]]:
    snapshot = GovernedExecutionSnapshot(revision, state_digest)
    dependencies = causal_artifact_dependencies(command, role)
    binding = execution_binding(command, root, snapshot, dependencies, requirement)
    binding_key = digest_json(binding)
    cached = read_for_binding(root, binding_key, binding)
    if cached is not None:
        record, observation = cached
        reference = reused_reference(reference_for(record, observation))
        print_execution("reuse", role, command, binding_key, int(record["duration_ms"]))
        return completed(command, record), reference.duration_ms, reference.payload()
    print_execution("execute", role, command, binding_key)
    result, duration_ms = execute(command, root, snapshot)
    envelope, reference = create_observation(
        root, binding, result.returncode, result.stdout, result.stderr, duration_ms
    )
    if result.returncode == 0:
        stage(envelope)
    print_execution("finish", role, command, binding_key, duration_ms, result.returncode)
    return result, duration_ms, reference.payload()


def execute(
    command: list[str], root: Path, snapshot: GovernedExecutionSnapshot
) -> tuple[subprocess.CompletedProcess[str], int]:
    started = time.perf_counter_ns()
    environment = dict(os.environ)
    environment["WORTH_UI_LEDGER_SOURCE_REVISION"] = snapshot.revision
    environment["WORTH_UI_LEDGER_SOURCE_STATE_DIGEST"] = snapshot.state_digest
    result = subprocess.run(
        command,
        cwd=root,
        env=environment,
        capture_output=True,
        text=True,
        check=False,
        timeout=execution_timeout_seconds(command),
    )
    duration_ms = max(1, (time.perf_counter_ns() - started + 999_999) // 1_000_000)
    return result, duration_ms


def reference_for(record: dict[str, Any], observation: str) -> ExecutionReference:
    binding = record["execution_binding"]
    return ExecutionReference(
        str(record["execution_binding_key"]),
        observation,
        digest_json(binding["command"]),
        int(record["duration_ms"]),
        "executed",
    )


def execution_timeout_seconds(command: list[str]) -> int:
    joined = " ".join(command)
    if "phase5_locality_closure::" in joined:
        return 600
    if "native_phase_f_reconstruction::" in joined:
        return 530
    return 300


def completed(
    command: list[str], record: dict[str, Any]
) -> subprocess.CompletedProcess[str]:
    return subprocess.CompletedProcess(
        command,
        int(record["returncode"]),
        str(record["stdout"]),
        str(record["stderr"]),
    )


def print_execution(
    posture: str,
    role: str,
    command: list[str],
    binding_key: str,
    duration_ms: int | None = None,
    returncode: int | None = None,
) -> None:
    details = ""
    if returncode is not None:
        details += f" exit={returncode}"
    if duration_ms is not None:
        details += f" duration={duration_ms}ms"
    print(
        f"[{posture}] {role} {command_identity(command)}{details} "
        f"binding={binding_key[:12]}",
        file=sys.stderr,
        flush=True,
    )


def command_identity(command: list[str]) -> str:
    return next((word for word in reversed(command) if "::" in word), command[0])

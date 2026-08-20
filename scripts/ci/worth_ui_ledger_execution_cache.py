from __future__ import annotations

import hashlib
import json
import os
import subprocess
import sys
import tempfile
import time
from pathlib import Path
from typing import Any

from worth_ui_ledger_runner_authentication import authentication_tag, authenticates


CACHE_ENV = "WORTH_UI_LEDGER_EXECUTION_CACHE"
SCHEMA = "worth-ui-ledger-execution-receipt-v2"
COMPILE_ARTIFACT_ENV = "WORTH_UI_COMPILE_ARTIFACT"
PREDECESSOR_ARTIFACT_ENV = "WORTH_UI_PREDECESSOR_ARTIFACT"
CANDIDATE_LEDGER_ENV = "WORTH_UI_MILESTONE_3141_LEDGER"
COMPILE_ARTIFACT = "_docs/worth-ui/milestone-3.14.1-evidence/compile-contracts.json"
LEDGER = "_docs/worth-ui/milestone-3.14.1-proof-ledger.csv"
P3_PREDECESSOR = "_docs/worth-ui/milestone-3.14.1-evidence/p3-predecessor-handoff.json"
P4_PREDECESSOR = "_docs/worth-ui/milestone-3.14.1-evidence/p4-predecessor-handoff.json"
P5_PREDECESSOR = "_docs/worth-ui/milestone-3.14.1-evidence/p5-predecessor-handoff.json"


def timed_execution(
    command: list[str],
    root: Path,
    revision: str,
    state_digest: str,
    role: str,
) -> tuple[subprocess.CompletedProcess[str], int, dict[str, Any]]:
    dependencies = causal_artifact_dependencies(command, role)
    binding = execution_binding(command, root, revision, state_digest, dependencies)
    key = digest_json(binding)
    cached = read_cached(root, key, binding)
    if cached is not None:
        print(
            f"[reuse] {role} {command_identity(command)} "
            f"duration={cached['duration_ms']}ms key={key[:12]}",
            file=sys.stderr,
            flush=True,
        )
        return completed(command, cached), int(cached["duration_ms"]), receipt(cached, True)
    print(
        f"[execute] {role} {command_identity(command)} key={key[:12]}",
        file=sys.stderr,
        flush=True,
    )
    started = time.perf_counter_ns()
    result = subprocess.run(
        command,
        cwd=root,
        capture_output=True,
        text=True,
        check=False,
        timeout=execution_timeout_seconds(command),
    )
    duration_ms = max(1, (time.perf_counter_ns() - started + 999_999) // 1_000_000)
    record = {
        **binding,
        "key": key,
        "returncode": result.returncode,
        "stdout": result.stdout,
        "stderr": result.stderr,
        "duration_ms": duration_ms,
    }
    if result.returncode == 0:
        write_cached(root, key, record)
    print(
        f"[finish] {role} {command_identity(command)} "
        f"exit={result.returncode} duration={duration_ms}ms",
        file=sys.stderr,
        flush=True,
    )
    return result, duration_ms, receipt(record, False)


def execution_timeout_seconds(command: list[str]) -> int:
    joined = " ".join(command)
    if "phase5_locality_closure::" in joined:
        return 530
    if "native_phase_f_reconstruction::" in joined:
        return 530
    return 300


def execution_binding(
    command: list[str],
    root: Path,
    revision: str,
    state_digest: str,
    artifact_dependencies: tuple[str, ...] = (),
) -> dict[str, Any]:
    return {
        "schema": SCHEMA,
        "command": command,
        "source_revision": revision,
        "source_state_digest": state_digest,
        "artifact_bindings": artifact_bindings(root, command, artifact_dependencies),
    }


def artifact_bindings(
    root: Path, command: list[str], dependencies: tuple[str, ...]
) -> dict[str, dict[str, str]]:
    result = {}
    for name in sorted(set(dependencies)):
        value = os.environ.get(name, default_artifact(name, command))
        if value is None:
            continue
        identity = Path(value)
        if not identity.is_absolute():
            identity = root / identity
        result[name] = {
            "sha256": (
                hashlib.sha256(identity.read_bytes()).hexdigest()
                if identity.is_file()
                else "missing"
            ),
        }
    return result


def default_artifact(name: str, command: list[str]) -> str:
    if name == COMPILE_ARTIFACT_ENV:
        return COMPILE_ARTIFACT
    if name == CANDIDATE_LEDGER_ENV:
        return LEDGER
    if name == PREDECESSOR_ARTIFACT_ENV:
        joined = " ".join(command)
        if "phase_five" in joined:
            return P5_PREDECESSOR
        return P4_PREDECESSOR if "phase_four" in joined else P3_PREDECESSOR
    raise ValueError(f"unknown ledger execution artifact dependency: {name}")


def causal_artifact_dependencies(command: list[str], role: str) -> tuple[str, ...]:
    if role.endswith("discovery"):
        return ()
    joined = " ".join(command)
    dependencies = []
    if "compile_contract_artifact" in joined:
        dependencies.append(COMPILE_ARTIFACT_ENV)
    if "milestone_3141_phase1_ledger" in joined:
        dependencies.append(CANDIDATE_LEDGER_ENV)
    if "predecessor_handoff" in joined or "predecessor_artifact" in joined:
        dependencies.append(PREDECESSOR_ARTIFACT_ENV)
    return tuple(dependencies)


def read_cached(root: Path, key: str, binding: dict[str, Any]) -> dict[str, Any] | None:
    identity = cache_identity(key)
    if identity is None or not identity.is_file():
        return None
    try:
        envelope = json.loads(identity.read_text(encoding="utf-8"))
        record = envelope["record"]
        if (
            envelope.get("receipt_sha256") != digest_json(record)
            or not authenticates(record, envelope.get("runner_authentication"), root)
            or record.get("key") != key
            or {field: record.get(field) for field in binding} != binding
            or record.get("returncode") != 0
        ):
            return None
        return record
    except (KeyError, OSError, TypeError, ValueError, json.JSONDecodeError):
        return None


def write_cached(root: Path, key: str, record: dict[str, Any]) -> None:
    identity = cache_identity(key)
    if identity is None:
        return
    identity.parent.mkdir(parents=True, exist_ok=True)
    envelope = {
        "record": record,
        "receipt_sha256": digest_json(record),
        "runner_authentication": authentication_tag(record, root),
    }
    descriptor, temporary = tempfile.mkstemp(prefix=".receipt-", dir=identity.parent)
    try:
        with os.fdopen(descriptor, "w", encoding="utf-8") as stream:
            json.dump(envelope, stream, sort_keys=True)
            stream.write("\n")
        os.replace(temporary, identity)
    finally:
        if os.path.exists(temporary):
            os.unlink(temporary)


def cache_identity(key: str) -> Path | None:
    root = os.environ.get(CACHE_ENV)
    return None if root is None else Path(root) / "executions" / key[:2] / f"{key}.json"


def invalidate_receipts(receipts: list[dict[str, Any]], roles: set[str]) -> None:
    for receipt_record in receipts:
        if receipt_record.get("role") not in roles:
            continue
        key = receipt_record.get("key")
        if not isinstance(key, str):
            continue
        identity = cache_identity(key)
        if identity is not None:
            identity.unlink(missing_ok=True)


def completed(
    command: list[str], record: dict[str, Any]
) -> subprocess.CompletedProcess[str]:
    return subprocess.CompletedProcess(
        command,
        int(record["returncode"]),
        str(record["stdout"]),
        str(record["stderr"]),
    )


def receipt(record: dict[str, Any], reused: bool) -> dict[str, Any]:
    return {
        "key": record["key"],
        "command_sha256": hashlib.sha256(
            json.dumps(record["command"], separators=(",", ":")).encode("utf-8")
        ).hexdigest(),
        "duration_ms": record["duration_ms"],
        "reused": reused,
    }


def digest_json(value: object) -> str:
    return hashlib.sha256(
        json.dumps(value, sort_keys=True, separators=(",", ":")).encode("utf-8")
    ).hexdigest()


def command_identity(command: list[str]) -> str:
    return next((word for word in reversed(command) if "::" in word), command[0])

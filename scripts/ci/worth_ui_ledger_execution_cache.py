from __future__ import annotations

import csv
import hashlib
import json
import os
import subprocess
import sys
import tempfile
import time
from pathlib import Path
from typing import Any

from worth_ui_ledger_command import CLAIM_FIELDS
from worth_ui_ledger_runner_authentication import authentication_tag, authenticates


CACHE_ENV = "WORTH_UI_LEDGER_EXECUTION_CACHE"
SCHEMA = "worth-ui-ledger-execution-receipt-v2"
LEDGER_BINDING_SCHEMA = "worth-ui-ledger-claim-prefix-v1"
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
    requirement: str | None = None,
) -> tuple[subprocess.CompletedProcess[str], int, dict[str, Any]]:
    dependencies = causal_artifact_dependencies(command, role)
    binding = execution_binding(
        command, root, revision, state_digest, dependencies, requirement
    )
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
        return 600
    if "native_phase_f_reconstruction::" in joined:
        return 530
    return 300


def execution_binding(
    command: list[str],
    root: Path,
    revision: str,
    state_digest: str,
    artifact_dependencies: tuple[str, ...] = (),
    requirement: str | None = None,
) -> dict[str, Any]:
    return {
        "schema": SCHEMA,
        "command": command,
        "source_revision": revision,
        "source_state_digest": state_digest,
        "artifact_bindings": artifact_bindings(
            root, command, artifact_dependencies, requirement
        ),
    }


def artifact_bindings(
    root: Path,
    command: list[str],
    dependencies: tuple[str, ...],
    requirement: str | None = None,
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
            "sha256": artifact_digest(identity, name, requirement),
        }
    return result


def artifact_digest(identity: Path, name: str, requirement: str | None) -> str:
    if not identity.is_file():
        return "missing"
    if name != CANDIDATE_LEDGER_ENV:
        return hashlib.sha256(identity.read_bytes()).hexdigest()
    if requirement is None:
        raise ValueError("candidate ledger dependency requires its governed row")
    return ledger_claim_prefix_digest(identity, requirement)


def ledger_claim_prefix_digest(identity: Path, requirement: str) -> str:
    phase = int(requirement[1])
    predecessor = requirement.endswith("-PREDECESSOR-01")
    close = requirement.endswith("-CLOSE-01")
    through_phase = phase - 1 if predecessor else 5
    excluded = requirement if close else None
    fields = (*CLAIM_FIELDS, "exact_command", "retained_result_artifact")
    with identity.open(encoding="utf-8", newline="") as stream:
        rows = [
            {field: row[field] for field in fields}
            for row in csv.DictReader(stream)
            if int(row["phase"]) <= (phase if close else through_phase)
            and row["requirement"] != excluded
        ]
    return digest_json(
        {
            "schema": LEDGER_BINDING_SCHEMA,
            "through_phase": phase if close else through_phase,
            "excluded_requirement": excluded,
            "rows": rows,
        }
    )


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
    if command_reads_candidate_ledger(command):
        dependencies.append(CANDIDATE_LEDGER_ENV)
    if "predecessor_handoff" in joined or "predecessor_artifact" in joined:
        dependencies.append(PREDECESSOR_ARTIFACT_ENV)
    return tuple(dependencies)


def command_reads_candidate_ledger(command: list[str]) -> bool:
    joined = " ".join(command)
    return (
        "milestone_3141_phase1_ledger" in joined
        and "result_artifact::mutation_tests::"
        "phase_two_boundary_observation_rejects_each_causal_mutation" not in joined
    )


def artifact_bindings_match(
    observed: object,
    expected: dict[str, dict[str, str]],
    command: list[str],
) -> bool:
    if not isinstance(observed, dict):
        return False
    retained = dict(observed)
    current = dict(expected)
    if not command_reads_candidate_ledger(command):
        retained.pop(CANDIDATE_LEDGER_ENV, None)
        current.pop(CANDIDATE_LEDGER_ENV, None)
    return retained == current


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

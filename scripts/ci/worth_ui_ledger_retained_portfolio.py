from __future__ import annotations

import csv
import hashlib
import json
import os
import tempfile
from pathlib import Path
from typing import Any

from worth_ui_ledger_command import CLAIM_FIELDS
from worth_ui_ledger_durable_receipts import (
    harvest_referenced_receipts,
    read_durable_envelope,
)
from worth_ui_ledger_runner_authentication import (
    RunnerProvenanceUnavailable,
    authenticates,
    runner_key_fingerprint,
)


SCHEMA = "worth-ui-ledger-retained-portfolio-v2"
EVIDENCE_ROOT = "_docs/worth-ui/milestone-3.14.1-evidence"


def portfolio_identity(phase: int) -> str:
    return f"{EVIDENCE_ROOT}/p{phase}-closure-portfolio.json"


def publish(
    root: Path,
    ledger: Path,
    phase: int,
    revision: str,
    state_digest: str,
) -> dict[str, Any]:
    persist_referenced_receipts(root, ledger, phase, state_digest)
    portfolio = build(root, ledger, phase, revision, state_digest)
    destination = root / portfolio_identity(phase)
    replace_json(destination, portfolio)
    return portfolio


def validate(
    root: Path,
    ledger: Path,
    phase: int,
    revision: str,
    state_digest: str,
) -> dict[str, Any]:
    identity = root / portfolio_identity(phase)
    retained = json.loads(identity.read_text(encoding="utf-8"))
    retained_fingerprint = retained.get("runner_key_fingerprint")
    if retained_fingerprint != runner_key_fingerprint(root):
        raise RunnerProvenanceUnavailable(
            "retained closure portfolio belongs to a different runner key"
        )
    expected = build(root, ledger, phase, revision, state_digest)
    if retained != expected:
        raise RuntimeError("retained closure portfolio differs from its exact evidence")
    return retained


def build(
    root: Path,
    ledger: Path,
    phase: int,
    revision: str,
    state_digest: str,
) -> dict[str, Any]:
    with ledger.open(encoding="utf-8", newline="") as stream:
        rows = [row for row in csv.DictReader(stream) if int(row["phase"]) <= phase]
    expected = {2: 30, 3: 47, 4: 68, 5: 80}[phase]
    if len(rows) != expected or any(
        row["result"] != "PROVED" or row["final_source"] != "true" for row in rows
    ):
        raise RuntimeError("retained portfolio requires an exact proved prefix")
    retained_rows = [retained_row(root, row, phase) for row in rows]
    execution_inputs = [
        artifact_payload(root, row) for row in rows if int(row["phase"]) == phase
    ]
    predecessor = predecessor_handoff(root, phase)
    if predecessor is not None:
        execution_inputs.extend(predecessor.get("rows", []))
    executions = aggregate_executions(
        execution_inputs, root, revision, state_digest, {row["requirement"] for row in rows}
    )
    body = {
        "schema": SCHEMA,
        "through_phase": phase,
        "source_revision": revision,
        "source_state_digest": state_digest,
        "runner_key_fingerprint": runner_key_fingerprint(root),
        "ledger_sha256": digest(ledger_prefix_bytes(ledger, phase)),
        "rows": retained_rows,
        "predecessor_handoff": predecessor_identity(root, phase),
        "executions": executions,
        "unique_execution_count": len(executions),
        "execution_reference_count": sum(
            len(execution["requirements"]) for execution in executions
        ),
    }
    return {**body, "portfolio_sha256": digest_json(body)}


def retained_row(root: Path, row: dict[str, str], current_phase: int) -> dict[str, str]:
    identity = row["retained_result_artifact"]
    artifact = root / identity
    actual_digest = digest(artifact.read_bytes())
    if actual_digest != row["result_artifact_digest"]:
        raise RuntimeError(f"retained portfolio artifact drifted for {row['requirement']}")
    payload = json.loads(artifact.read_text(encoding="utf-8"))
    expected_claim = row_claim_digest(row)
    if int(row["phase"]) == current_phase and not authenticated_row_payload(root, payload):
        raise RuntimeError(f"retained portfolio row drifted for {row['requirement']}")
    if (
        payload.get("requirement") != row["requirement"]
        or payload.get("exit_posture") != "passed"
        or payload.get("claim_digest") != expected_claim
        or payload.get("run_nonce") != row["run_nonce"]
    ):
        raise RuntimeError(f"retained portfolio row drifted for {row['requirement']}")
    return {
        "requirement": row["requirement"],
        "claim_digest": expected_claim,
        "artifact": identity,
        "artifact_sha256": actual_digest,
        "run_nonce": row["run_nonce"],
        "exact_command_sha256": digest(row["exact_command"].encode("utf-8")),
    }


def artifact_payload(root: Path, row: dict[str, str]) -> dict[str, Any]:
    return json.loads((root / row["retained_result_artifact"]).read_text(encoding="utf-8"))


def predecessor_handoff(root: Path, phase: int) -> dict[str, Any] | None:
    if phase < 3:
        return None
    identity = root / f"{EVIDENCE_ROOT}/p{phase}-predecessor-handoff.json"
    payload = json.loads(identity.read_text(encoding="utf-8"))
    if payload.get("through_phase") != phase - 1 or not isinstance(payload.get("rows"), list):
        raise RuntimeError("retained portfolio has an invalid predecessor handoff")
    return payload


def ledger_prefix_bytes(ledger: Path, phase: int) -> bytes:
    text = ledger.read_text(encoding="utf-8")
    lines = text.splitlines(keepends=True)
    if not lines:
        raise RuntimeError("retained portfolio ledger is empty")
    selected = [lines[0]]
    for line in lines[1:]:
        if not line.strip():
            continue
        record = next(csv.reader([line]))
        if int(record[0]) <= phase:
            selected.append(line)
    return "".join(selected).encode("utf-8")


def persist_referenced_receipts(
    root: Path, ledger: Path, phase: int, state_digest: str
) -> None:
    with ledger.open(encoding="utf-8", newline="") as stream:
        rows = [row for row in csv.DictReader(stream) if int(row["phase"]) <= phase]
    payloads = [artifact_payload(root, row) for row in rows if int(row["phase"]) == phase]
    predecessor = predecessor_handoff(root, phase)
    if predecessor is not None:
        payloads.extend(predecessor.get("rows", []))
    for payload in payloads:
        receipts = payload.get("execution_receipts", [])
        if not isinstance(receipts, list):
            raise RuntimeError("retained row omits its execution receipt inventory")
        harvest_referenced_receipts(root, state_digest, receipts)


def predecessor_identity(root: Path, phase: int) -> dict[str, str] | None:
    if phase < 3:
        return None
    identity = f"{EVIDENCE_ROOT}/p{phase}-predecessor-handoff.json"
    return {"artifact": identity, "artifact_sha256": digest((root / identity).read_bytes())}


def aggregate_executions(
    payloads: list[dict[str, Any]],
    root: Path,
    revision: str,
    state_digest: str,
    requirements: set[str],
) -> list[dict[str, Any]]:
    aggregate: dict[str, dict[str, Any]] = {}
    observed_roles: dict[str, set[str]] = {}
    for payload in payloads:
        if not authenticated_row_payload(root, payload):
            raise RuntimeError("retained row evidence lacks runner provenance")
        requirement = payload.get("requirement")
        receipts = payload.get("execution_receipts", [])
        if not isinstance(requirement, str) or not isinstance(receipts, list):
            raise RuntimeError("retained row omits its execution receipt inventory")
        payload_roles: list[str] = []
        for receipt in receipts:
            if not isinstance(receipt, dict) or not isinstance(receipt.get("key"), str):
                raise RuntimeError("retained execution receipt is malformed")
            key = receipt["key"]
            record_sha256 = validate_execution_receipt(
                root, revision, state_digest, receipt
            )
            role = receipt.get("role")
            if not isinstance(role, str):
                raise RuntimeError("retained execution receipt omits its role")
            payload_roles.append(role)
            expected = expected_command_sha256(payload, role)
            if expected is None or receipt.get("command_sha256") != expected:
                raise RuntimeError("retained execution receipt is bound to the wrong row command")
            observed_roles.setdefault(requirement, set()).add(role)
            observed = {
                "key": key,
                "command_sha256": receipt.get("command_sha256"),
                "duration_ms": receipt.get("duration_ms"),
                "receipt_sha256": record_sha256,
                "requirements": [],
            }
            current = aggregate.setdefault(key, observed)
            if (
                current["command_sha256"] != observed["command_sha256"]
                or current["duration_ms"] != observed["duration_ms"]
                or current["receipt_sha256"] != observed["receipt_sha256"]
            ):
                raise RuntimeError("one execution identity has conflicting retained evidence")
            if requirement not in current["requirements"]:
                current["requirements"].append(requirement)
        expected_roles = ["main-discovery", "ignored-discovery", "main-test"]
        if requirement == "P4-FONT-COLLECTION-01":
            expected_roles.append("public-example")
        if isinstance(payload.get("hostile_control"), dict):
            expected_roles.extend(("control-discovery", "control-test"))
        if sorted(payload_roles) != sorted(expected_roles):
            raise RuntimeError(
                "retained row execution receipts have a missing, duplicate, or unexpected role"
            )
    for execution in aggregate.values():
        execution["requirements"].sort()
    mandatory_roles = {"main-discovery", "ignored-discovery", "main-test"}
    for requirement in requirements:
        if not mandatory_roles.issubset(observed_roles.get(requirement, set())):
            raise RuntimeError(
                f"retained portfolio omits governed execution receipts for {requirement}"
            )
    return sorted(aggregate.values(), key=lambda execution: execution["key"])


def expected_command_sha256(payload: dict[str, Any], role: str) -> str | None:
    field = {
        "main-discovery": "list_command",
        "ignored-discovery": "ignored_list_command",
        "main-test": "test_command",
        "public-example": "public_example_command",
    }.get(role)
    owner: object = payload
    if role in {"control-discovery", "control-test"}:
        owner = payload.get("hostile_control")
        field = "list_command" if role == "control-discovery" else "test_command"
    if not isinstance(owner, dict) or not isinstance(field, str):
        return None
    command = owner.get(field)
    if not isinstance(command, list) or not all(isinstance(part, str) for part in command):
        return None
    return digest(json.dumps(command, separators=(",", ":")).encode("utf-8"))


def validate_execution_receipt(
    root: Path,
    revision: str,
    state_digest: str,
    receipt: dict[str, Any],
) -> str:
    key = receipt["key"]
    try:
        envelope = read_durable_envelope(root, key)
        record = envelope["record"]
    except (KeyError, RuntimeError, TypeError) as error:
        raise RuntimeError("retained execution receipt is absent") from error
    receipt_sha256 = digest_json(record)
    binding = {
        "schema": record.get("schema"),
        "command": record.get("command"),
        "source_revision": record.get("source_revision"),
        "source_state_digest": record.get("source_state_digest"),
        "artifact_bindings": record.get("artifact_bindings"),
    }
    command_sha256 = digest(
        json.dumps(record.get("command"), separators=(",", ":")).encode("utf-8")
    )
    if envelope.get("receipt_sha256") != receipt_sha256:
        raise RuntimeError("retained execution receipt differs from its exact execution")
    if not authenticates(record, envelope.get("runner_authentication"), root):
        raise RuntimeError("retained execution receipt differs from its exact execution")
    if (
        record.get("key") != key
        or digest_json(binding) != key
        or record.get("source_revision") != revision
        or record.get("source_state_digest") != state_digest
        or record.get("returncode") != 0
        or command_sha256 != receipt.get("command_sha256")
        or record.get("duration_ms") != receipt.get("duration_ms")
    ):
        raise RuntimeError("retained execution receipt differs from its exact execution")
    return receipt_sha256


def authenticated_row_payload(root: Path, payload: dict[str, Any]) -> bool:
    unsigned = {
        key: value
        for key, value in payload.items()
        if key not in {"artifact_sha256", "runner_authentication"}
    }
    return authenticates(unsigned, payload.get("runner_authentication"), root)


def row_claim_digest(row: dict[str, str]) -> str:
    hasher = hashlib.sha256()
    for field in CLAIM_FIELDS:
        hasher.update(field.encode("utf-8"))
        hasher.update(b"\0")
        hasher.update(row[field].encode("utf-8"))
        hasher.update(b"\0")
    return hasher.hexdigest()


def replace_json(destination: Path, value: object) -> None:
    destination.parent.mkdir(parents=True, exist_ok=True)
    descriptor, temporary = tempfile.mkstemp(prefix=f".{destination.name}.", dir=destination.parent)
    try:
        with os.fdopen(descriptor, "w", encoding="utf-8") as stream:
            json.dump(value, stream, indent=2)
            stream.write("\n")
        os.replace(temporary, destination)
    finally:
        if os.path.exists(temporary):
            os.unlink(temporary)


def digest(content: bytes) -> str:
    return hashlib.sha256(content).hexdigest()


def digest_json(value: object) -> str:
    return digest(
        json.dumps(value, sort_keys=True, separators=(",", ":")).encode("utf-8")
    )

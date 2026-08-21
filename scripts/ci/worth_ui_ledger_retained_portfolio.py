from __future__ import annotations

import csv
import hashlib
import json
import os
import tempfile
from pathlib import Path
from typing import Any

from worth_ui_ledger_command import CLAIM_FIELDS
from worth_ui_ledger_causal_revalidation import source_digest_at
from worth_ui_ledger_durable_receipts import harvest_referenced_receipts
from worth_ui_ledger_execution_cache import CANDIDATE_LEDGER_ENV
from worth_ui_ledger_portfolio_executions import aggregate_executions, authenticated_row_payload
from worth_ui_ledger_runner_authentication import (
    RunnerProvenanceUnavailable,
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
        validate_predecessor_rows(root, rows, predecessor)
        execution_inputs.extend(predecessor.get("rows", []))
    previous_ledger = os.environ.get(CANDIDATE_LEDGER_ENV)
    os.environ[CANDIDATE_LEDGER_ENV] = str(ledger.resolve())
    try:
        executions = aggregate_executions(
            execution_inputs,
            root,
            revision,
            state_digest,
            {row["requirement"] for row in rows},
        )
    finally:
        if previous_ledger is None:
            os.environ.pop(CANDIDATE_LEDGER_ENV, None)
        else:
            os.environ[CANDIDATE_LEDGER_ENV] = previous_ledger
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
    current_mapping_drifted = int(row["phase"]) == current_phase and (
        payload.get("production_entry") != row["production_entry"]
        or payload.get("independent_oracle") != row["independent_oracle"]
        or payload.get("executed_exact_command") != row["exact_command"]
        or payload.get("source_identity") != row["source_identity"].split(";")
        or payload.get("mapping_source_identity") != row["source_identity"].split(";")
        or payload.get("source_digest")
        != source_digest_at(root, tuple(row["source_identity"].split(";")))
    )
    if (
        payload.get("requirement") != row["requirement"]
        or payload.get("exit_posture") != "passed"
        or payload.get("claim_digest") != expected_claim
        or payload.get("run_nonce") != row["run_nonce"]
        or current_mapping_drifted
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


def validate_predecessor_rows(
    root: Path,
    ledger_rows: list[dict[str, str]],
    predecessor: dict[str, Any],
) -> None:
    expected = {
        row["requirement"]: row
        for row in ledger_rows
        if int(row["phase"]) <= int(predecessor["through_phase"])
    }
    observed_rows = predecessor.get("rows")
    if not isinstance(observed_rows, list) or {
        row.get("requirement") for row in observed_rows if isinstance(row, dict)
    } != set(expected):
        raise RuntimeError("retained predecessor rows differ from the ledger prefix")
    for payload in observed_rows:
        if not isinstance(payload, dict):
            raise RuntimeError("retained predecessor row is malformed")
        row = expected[payload["requirement"]]
        sources = row["source_identity"].split(";")
        if (
            not authenticated_row_payload(root, payload)
            or payload.get("claim_digest") != row_claim_digest(row)
            or payload.get("production_entry") != row["production_entry"]
            or payload.get("independent_oracle") != row["independent_oracle"]
            or payload.get("executed_exact_command") != row["exact_command"]
            or payload.get("source_identity") != sources
            or payload.get("mapping_source_identity") != sources
            or payload.get("source_digest") != source_digest_at(root, tuple(sources))
        ):
            raise RuntimeError(
                f"retained predecessor row drifted for {row['requirement']}"
            )


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
        reuse = payload.get("causal_reuse")
        receipt_state = (
            reuse.get("predecessor_source_state_digest")
            if isinstance(reuse, dict)
            else state_digest
        )
        if not isinstance(receipt_state, str):
            raise RuntimeError("retained row causal reuse omits its receipt source state")
        harvest_referenced_receipts(root, receipt_state, receipts)


def predecessor_identity(root: Path, phase: int) -> dict[str, str] | None:
    if phase < 3:
        return None
    identity = f"{EVIDENCE_ROOT}/p{phase}-predecessor-handoff.json"
    return {"artifact": identity, "artifact_sha256": digest((root / identity).read_bytes())}


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

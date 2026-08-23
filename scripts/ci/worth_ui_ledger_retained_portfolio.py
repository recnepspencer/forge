from __future__ import annotations

import csv
import hashlib
import json
import os
import tempfile
from pathlib import Path
from typing import Any

from worth_ui_ledger_command import claim_digest_for_row
from worth_ui_ledger_causal_revalidation import source_digest_at
from worth_ui_ledger_execution_observation_retention import retain_payload_observations
from worth_ui_ledger_execution_binding import CANDIDATE_LEDGER_ENV
from worth_ui_ledger_execution_observation import REFERENCE_SCHEMA
from worth_ui_ledger_execution_observation_migration import migration_identity
from worth_ui_ledger_portfolio_executions import (
    aggregate_executions,
    authenticated_row_payload,
)
from worth_ui_ledger_runner_authentication import (
    RunnerProvenanceUnavailable,
    existing_runner_key_fingerprint,
    runner_key_fingerprint,
)
from worth_ui_ledger_candidate_basis import from_path
from worth_ui_ledger_artifact_identity import predecessor_schema
from worth_ui_ledger_artifact_transaction import (
    register_active_identity,
    require_active_transaction,
)


CURRENT_SCHEMA = "worth-ui-ledger-retained-portfolio-v4"
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
    require_active_transaction(root)
    persist_referenced_receipts(root, ledger, phase, state_digest)
    portfolio = build(
        root, ledger, phase, revision, state_digest, runner_key_fingerprint(root)
    )
    destination = root / portfolio_identity(phase)
    register_active_identity(root, destination)
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
    current_fingerprint = existing_runner_key_fingerprint(root)
    if retained_fingerprint != current_fingerprint:
        raise RunnerProvenanceUnavailable(
            "retained closure portfolio belongs to a different runner key"
        )
    expected = build(
        root, ledger, phase, revision, state_digest, current_fingerprint
    )
    if retained != expected:
        raise RuntimeError("retained closure portfolio differs from its exact evidence")
    return retained


def build(
    root: Path,
    ledger: Path,
    phase: int,
    revision: str,
    state_digest: str,
    key_fingerprint: str,
) -> dict[str, Any]:
    with ledger.open(encoding="utf-8", newline="") as stream:
        rows = [row for row in csv.DictReader(stream) if int(row["phase"]) <= phase]
    expected = {2: 30, 3: 47, 4: 68, 5: 80, 6: 90}[phase]
    if len(rows) != expected or any(
        row["result"] != "PROVED" or row["final_source"] != "true" for row in rows
    ):
        raise RuntimeError("retained portfolio requires an exact proved prefix")
    retained_rows = [retained_row(root, row, phase) for row in rows]
    execution_inputs = [
        artifact_payload(root, row) for row in execution_input_rows(rows, phase)
    ]
    predecessor = predecessor_handoff(root, phase)
    if predecessor is not None:
        validate_predecessor_rows(
            root, ledger, rows, predecessor, revision, state_digest
        )
        execution_inputs.extend(predecessor.get("rows", []))
    migration_keys = legacy_migration_keys(execution_inputs)
    previous_ledger = os.environ.get(CANDIDATE_LEDGER_ENV)
    os.environ[CANDIDATE_LEDGER_ENV] = str(ledger.resolve())
    try:
        executions = aggregate_executions(
            execution_inputs, root, revision, state_digest,
            {row["requirement"] for row in rows},
        )
    finally:
        if previous_ledger is None:
            os.environ.pop(CANDIDATE_LEDGER_ENV, None)
        else:
            os.environ[CANDIDATE_LEDGER_ENV] = previous_ledger
    body = {
        "schema": CURRENT_SCHEMA,
        "through_phase": phase,
        "source_revision": revision,
        "source_state_digest": state_digest,
        "runner_key_fingerprint": key_fingerprint,
        "ledger_sha256": digest(ledger_prefix_bytes(ledger, phase)),
        "rows": retained_rows,
        "predecessor_handoff": predecessor_identity(root, phase),
        "executions": executions,
        "logical_execution_count": len(executions),
        "source_bound_execution_count": len({
            observation["execution_binding_key"]
            for execution in executions
            for observation in execution["observations"]
        }),
        "physical_observation_count": len({
            observation["observation_sha256"]
            for execution in executions
            for observation in execution["observations"]
        }),
        "execution_reference_count": sum(
            len(execution["requirements"]) for execution in executions
        ),
        "execution_observation_migrations": migration_inventory(
            root, migration_keys
        ),
    }
    return {**body, "portfolio_sha256": digest_json(body)}


def execution_input_rows(
    rows: list[dict[str, str]], phase: int
) -> list[dict[str, str]]:
    if phase == 2:
        return rows
    return [row for row in rows if int(row["phase"]) == phase]


def legacy_migration_keys(payloads: list[dict[str, Any]]) -> tuple[str, ...]:
    keys: set[str] = set()
    for payload in payloads:
        receipts = payload.get("execution_receipts")
        if not isinstance(receipts, list):
            continue
        for receipt in receipts:
            if not isinstance(receipt, dict) or receipt.get("schema") == REFERENCE_SCHEMA:
                continue
            key = receipt.get("key")
            if isinstance(key, str):
                keys.add(key)
    return tuple(sorted(keys))


def migration_inventory(root: Path, keys: tuple[str, ...]) -> list[dict[str, str]]:
    result = []
    for key in keys:
        identity = migration_identity(root, key)
        result.append({
            "legacy_execution_key": key,
            "artifact": identity.relative_to(root).as_posix(),
            "artifact_sha256": digest(identity.read_bytes()),
        })
    return result


def retained_row(root: Path, row: dict[str, str], current_phase: int) -> dict[str, str]:
    identity = row["retained_result_artifact"]
    artifact = root / identity
    actual_digest = digest(artifact.read_bytes())
    if actual_digest != row["result_artifact_digest"]:
        raise RuntimeError(f"retained portfolio artifact drifted for {row['requirement']}")
    payload = json.loads(artifact.read_text(encoding="utf-8"))
    expected_claim = claim_digest_for_row(row)
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
    if (
        payload.get("schema") != predecessor_schema(phase)
        or payload.get("through_phase") != phase - 1
        or not isinstance(payload.get("rows"), list)
    ):
        raise RuntimeError("retained portfolio has an invalid predecessor handoff")
    return payload


def validate_predecessor_rows(
    root: Path,
    ledger: Path,
    ledger_rows: list[dict[str, str]],
    predecessor: dict[str, Any],
    revision: str,
    state_digest: str,
) -> None:
    basis = from_path(ledger, int(predecessor["through_phase"]))
    if (
        predecessor.get("source_revision") != revision
        or predecessor.get("source_state_digest") != state_digest
        or predecessor.get("verification_basis") != basis.payload()
    ):
        raise RuntimeError("retained predecessor handoff is stale for the live source state")
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
            or payload.get("claim_digest") != claim_digest_for_row(row)
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
    payloads = [
        artifact_payload(root, row) for row in execution_input_rows(rows, phase)
    ]
    predecessor = predecessor_handoff(root, phase)
    if predecessor is not None:
        payloads.extend(predecessor.get("rows", []))
    for payload in payloads:
        retain_payload_observations(root, state_digest, payload)


def predecessor_identity(root: Path, phase: int) -> dict[str, str] | None:
    if phase < 3:
        return None
    identity = f"{EVIDENCE_ROOT}/p{phase}-predecessor-handoff.json"
    return {"artifact": identity, "artifact_sha256": digest((root / identity).read_bytes())}


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

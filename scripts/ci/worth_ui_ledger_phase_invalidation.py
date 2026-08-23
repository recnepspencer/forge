from __future__ import annotations

import csv
import hashlib
import json
from dataclasses import dataclass
from pathlib import Path

from worth_ui_ledger_artifact_identity import (
    ArtifactIdentity,
    phase_invalidation,
    require_row_evidence_identity,
    superseded_row_evidence,
)
from worth_ui_ledger_artifact_publication import (
    publish_json_artifact,
    replace_bytes_with_retry,
)
from worth_ui_ledger_artifact_transaction import ArtifactTransaction
from worth_ui_ledger_closure_selection import reopen_prepared_claim
from worth_ui_ledger_closure_storage import render_requirement_update


ALLOWED_CAUSES = frozenset(
    {
        "artifact-kind-mismatch",
        "architectural-correction",
        "independent-review-rejected",
    }
)


@dataclass(frozen=True)
class InvalidationRequest:
    phase: int
    incident_requirement: str
    observed_digest: str
    causes: tuple[str, ...]
    source_revision: str


@dataclass(frozen=True)
class InvalidationPublication:
    root: Path
    ledger: Path
    original: str
    fields: list[str]
    rows: list[dict[str, str]]
    proved: list[dict[str, str]]
    causally_reopened: list[dict[str, str]]
    archive: ArtifactIdentity
    corrupt_bytes: bytes
    receipt_identity: ArtifactIdentity
    receipt: dict[str, object]


@dataclass(frozen=True)
class InvalidationIncident:
    row: dict[str, str]
    observed_payload: dict[str, object]
    archive_identity: str


def invalidate_phase(
    root: Path, ledger: Path, request: InvalidationRequest
) -> dict[str, object]:
    original = ledger.read_text(encoding="utf-8")
    fields, rows = read_rows(original)
    phase_rows = [row for row in rows if int(row["phase"]) == request.phase]
    if not phase_rows:
        raise RuntimeError(f"Phase {request.phase} has no ledger rows")
    validate_causes(request.causes)
    incident = exact_incident_row(phase_rows, request.incident_requirement)
    canonical = require_row_evidence_identity(
        incident["requirement"], incident["retained_result_artifact"]
    )
    corrupt_bytes = canonical.destination(root).read_bytes()
    observed_digest = hashlib.sha256(corrupt_bytes).hexdigest()
    if observed_digest != request.observed_digest:
        raise RuntimeError("incident artifact digest changed before invalidation")
    if observed_digest == incident["result_artifact_digest"]:
        raise RuntimeError("incident artifact still matches the ledger-authenticated bytes")
    observed_payload = decode_object(corrupt_bytes)
    archive = superseded_row_evidence(incident["requirement"], observed_digest)
    receipt_identity = phase_invalidation(request.phase, observed_digest)
    proved = [
        row
        for row in phase_rows
        if row["result"] == "PROVED" and row["final_source"] == "true"
    ]
    if not proved:
        raise RuntimeError(f"Phase {request.phase} has no proved rows to invalidate")
    reject_incoherent_phase_posture(phase_rows, proved)
    causally_reopened = [
        row
        for row in rows
        if int(row["phase"]) > request.phase
        and row["result"] == "PROVED"
        and row["final_source"] == "true"
    ]
    receipt = invalidation_receipt(
        request,
        rows,
        proved,
        causally_reopened,
        InvalidationIncident(incident, observed_payload, archive.relative_path),
    )
    receipt_digest = publish_invalidation(
        InvalidationPublication(
            root,
            ledger,
            original,
            fields,
            rows,
            proved,
            causally_reopened,
            archive,
            corrupt_bytes,
            receipt_identity,
            receipt,
        )
    )
    return {
        "phase": request.phase,
        "invalidated_requirements": sorted(row["requirement"] for row in proved),
        "causally_reopened_requirements": sorted(
            row["requirement"] for row in causally_reopened
        ),
        "receipt": receipt_identity.relative_path,
        "receipt_sha256": receipt_digest,
        "superseded_artifact": archive.relative_path,
        "superseded_artifact_sha256": observed_digest,
    }


def publish_invalidation(publication: InvalidationPublication) -> str:
    transaction = ArtifactTransaction(
        publication.root,
        publication.ledger,
        [],
        (
            publication.archive.relative_path,
            publication.receipt_identity.relative_path,
        ),
    )
    try:
        archive_destination = publication.archive.destination(publication.root)
        archive_destination.parent.mkdir(parents=True, exist_ok=True)
        replace_bytes_with_retry(archive_destination, publication.corrupt_bytes)
        receipt_digest = publish_json_artifact(
            publication.root, publication.receipt_identity, publication.receipt
        )
        affected = [*publication.proved, *publication.causally_reopened]
        for row in affected:
            previous_digest = row["result_artifact_digest"]
            reopen_prepared_claim(row, previous_digest)
            row["reopen_lineage"] = (
                f"invalidation:{publication.receipt_identity.relative_path}"
                f"@{receipt_digest};"
                f"supersedes:{previous_digest}"
            )
        requirements = {row["requirement"] for row in affected}
        candidate = render_requirement_update(
            publication.original,
            publication.rows,
            publication.fields,
            requirements,
        )
        candidate_bytes = candidate.encode("utf-8")
        transaction.prepare_commit(candidate_bytes)
        replace_bytes_with_retry(publication.ledger, candidate_bytes)
        transaction.commit()
    except BaseException:
        transaction.rollback()
        raise
    return receipt_digest


def read_rows(content: str) -> tuple[list[str], list[dict[str, str]]]:
    reader = csv.DictReader(content.splitlines())
    fields = list(reader.fieldnames or ())
    return fields, list(reader)


def validate_causes(causes: tuple[str, ...]) -> None:
    if not causes:
        raise ValueError("phase invalidation requires at least one cause")
    if len(causes) != len(set(causes)):
        raise ValueError("phase invalidation causes must be unique")
    unknown = set(causes) - ALLOWED_CAUSES
    if unknown:
        raise ValueError(f"unsupported phase invalidation cause: {sorted(unknown)[0]}")


def exact_incident_row(
    phase_rows: list[dict[str, str]], requirement: str
) -> dict[str, str]:
    matches = [row for row in phase_rows if row["requirement"] == requirement]
    if len(matches) != 1:
        raise RuntimeError("incident requirement is not exactly one row in the phase")
    if matches[0]["result"] != "PROVED" or matches[0]["final_source"] != "true":
        raise RuntimeError("incident requirement is not currently proved")
    return matches[0]


def reject_incoherent_phase_posture(
    phase_rows: list[dict[str, str]], proved: list[dict[str, str]]
) -> None:
    governed = set(map(id, proved))
    for row in phase_rows:
        if id(row) in governed:
            continue
        if row["result"] != "OPEN" or row["final_source"] != "false":
            raise RuntimeError("phase contains a partially settled ledger posture")


def invalidation_receipt(
    request: InvalidationRequest,
    rows: list[dict[str, str]],
    proved: list[dict[str, str]],
    causally_reopened: list[dict[str, str]],
    incident: InvalidationIncident,
) -> dict[str, object]:
    return {
        "schema": "worth-ui-ledger-phase-invalidation-v2",
        "phase": request.phase,
        "source_revision": request.source_revision,
        "causes": sorted(request.causes),
        "incident": {
            "requirement": incident.row["requirement"],
            "canonical_artifact": incident.row["retained_result_artifact"],
            "ledger_artifact_sha256": incident.row["result_artifact_digest"],
            "observed_artifact_sha256": request.observed_digest,
            "observed_artifact_kind": observed_artifact_kind(
                incident.observed_payload
            ),
            "observed_schema": observed_schema(incident.observed_payload),
            "superseded_artifact": incident.archive_identity,
        },
        "invalidated_rows": [retained_row_lineage(row) for row in proved],
        "causally_reopened_rows": [
            retained_row_lineage(row) for row in causally_reopened
        ],
        "preserved_open_requirements": sorted(
            row["requirement"]
            for row in rows
            if int(row["phase"]) >= request.phase
            and row not in proved
            and row not in causally_reopened
            and row["result"] == "OPEN"
            and row["final_source"] == "false"
        ),
    }


def retained_row_lineage(row: dict[str, str]) -> dict[str, str]:
    return {
        "requirement": row["requirement"],
        "prior_result": row["result"],
        "prior_final_source": row["final_source"],
        "prior_result_artifact_digest": row["result_artifact_digest"],
        "prior_reopen_lineage": row["reopen_lineage"],
    }


def decode_object(content: bytes) -> dict[str, object]:
    try:
        payload = json.loads(content.decode("utf-8"))
    except (UnicodeDecodeError, json.JSONDecodeError) as error:
        raise RuntimeError("incident artifact is not a JSON object") from error
    if not isinstance(payload, dict):
        raise RuntimeError("incident artifact is not a JSON object")
    return payload


def observed_artifact_kind(payload: dict[str, object]) -> str:
    if "requirement" in payload and "schema_version" in payload:
        return "row-evidence"
    if "through_phase" in payload and str(payload.get("schema", "")).startswith(
        "worth-ui-phase-predecessor-handoff-"
    ):
        return "predecessor-handoff"
    return "unknown"


def observed_schema(payload: dict[str, object]) -> str:
    if "schema" in payload:
        return str(payload["schema"])
    if "schema_version" in payload:
        return f"row-schema-version-{payload['schema_version']}"
    return "unknown"

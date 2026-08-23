from __future__ import annotations

import csv
import hashlib
import json
from dataclasses import dataclass
from pathlib import Path

from worth_ui_ledger_artifact_identity import (
    ArtifactIdentity,
    artifact_drift_inventory,
    incident_row_evidence,
    require_row_evidence_identity,
)
from worth_ui_ledger_artifact_publication import (
    publish_json_artifact,
    replace_bytes_with_retry,
)
from worth_ui_ledger_artifact_transaction import ArtifactTransaction


@dataclass(frozen=True)
class DriftObservation:
    requirement: str
    phase: int
    ledger_digest: str
    observed_digest: str
    source_state_digest: str
    content: bytes
    archive: ArtifactIdentity


@dataclass(frozen=True)
class DriftCaptureRequest:
    incident_digest: str
    parent_invalidation: str
    parent_invalidation_digest: str
    expected_count: int


def capture_artifact_drift(
    root: Path,
    ledger: Path,
    request: DriftCaptureRequest,
) -> dict[str, object]:
    observations = observe_drift(root, ledger, request.incident_digest)
    if len(observations) != request.expected_count:
        raise RuntimeError(
            f"artifact drift count changed: expected {request.expected_count}, "
            f"observed {len(observations)}"
        )
    inventory = artifact_drift_inventory(request.incident_digest)
    payload = inventory_payload(
        request.incident_digest,
        request.parent_invalidation,
        request.parent_invalidation_digest,
        observations,
    )
    identities = tuple(
        [inventory.relative_path]
        + [observation.archive.relative_path for observation in observations]
    )
    transaction = ArtifactTransaction(root, ledger, [], identities)
    try:
        for observation in observations:
            destination = observation.archive.destination(root)
            destination.parent.mkdir(parents=True, exist_ok=True)
            replace_bytes_with_retry(destination, observation.content)
        inventory_digest = publish_json_artifact(root, inventory, payload)
        current_ledger = ledger.read_bytes()
        transaction.prepare_commit(current_ledger)
        transaction.commit()
    except BaseException:
        transaction.rollback()
        raise
    return {
        "inventory": inventory.relative_path,
        "inventory_sha256": inventory_digest,
        "observed_drift_count": len(observations),
        "requirements": [item.requirement for item in observations],
    }


def observe_drift(
    root: Path, ledger: Path, incident_digest: str
) -> list[DriftObservation]:
    with ledger.open(encoding="utf-8", newline="") as stream:
        rows = list(csv.DictReader(stream))
    observations = []
    for row in rows:
        ledger_digest = row["result_artifact_digest"]
        if row["result"] != "PROVED" or len(ledger_digest) != 64:
            continue
        canonical = require_row_evidence_identity(
            row["requirement"], row["retained_result_artifact"]
        )
        content = canonical.destination(root).read_bytes()
        observed_digest = hashlib.sha256(content).hexdigest()
        if observed_digest == ledger_digest:
            continue
        payload = decode_row_evidence(content)
        canonical.validate_json_payload(payload)
        observations.append(
            DriftObservation(
                row["requirement"],
                int(row["phase"]),
                ledger_digest,
                observed_digest,
                str(payload.get("source_state_digest", "missing")),
                content,
                incident_row_evidence(
                    incident_digest, row["requirement"], observed_digest
                ),
            )
        )
    return observations


def inventory_payload(
    incident_digest: str,
    parent_invalidation: str,
    parent_invalidation_digest: str,
    observations: list[DriftObservation],
) -> dict[str, object]:
    return {
        "schema": "worth-ui-ledger-artifact-drift-inventory-v1",
        "incident_artifact_sha256": incident_digest,
        "parent_invalidation": parent_invalidation,
        "parent_invalidation_sha256": parent_invalidation_digest,
        "observed_drift_count": len(observations),
        "rows": [
            {
                "phase": observation.phase,
                "requirement": observation.requirement,
                "ledger_artifact_sha256": observation.ledger_digest,
                "observed_artifact_sha256": observation.observed_digest,
                "observed_source_state_digest": observation.source_state_digest,
                "retained_observation": observation.archive.relative_path,
            }
            for observation in observations
        ],
    }


def decode_row_evidence(content: bytes) -> dict[str, object]:
    try:
        payload = json.loads(content.decode("utf-8"))
    except (UnicodeDecodeError, json.JSONDecodeError) as error:
        raise RuntimeError("drifted row evidence is not valid JSON") from error
    if not isinstance(payload, dict):
        raise RuntimeError("drifted row evidence is not a JSON object")
    return payload

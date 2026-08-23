from __future__ import annotations

import json
from dataclasses import dataclass
from pathlib import Path

from worth_ui_ledger_causal_revalidation import (
    SCHEMA as CAUSAL_REUSE_SCHEMA,
    available_receipts_match_payload_source,
    validate_causal_reuse,
)
from worth_ui_ledger_execution_observation_migration import migrate_payload
from worth_ui_ledger_row_cache import source_artifact_bindings


SHARED_MAIN_ROLES = ("main-discovery", "ignored-discovery", "main-test")


@dataclass(frozen=True)
class SharedExecutionLineageRequest:
    row: dict[str, str]
    payload: dict[str, object]
    root: Path
    revision: str
    state_digest: str
    current_claim: str


def inherit_shared_receipt_lineage(
    request: SharedExecutionLineageRequest,
) -> None:
    row = request.row
    payload = request.payload
    root = request.root
    shared_identity = payload.get("shared_main_artifact")
    if not isinstance(shared_identity, str):
        return
    shared = json.loads((root / shared_identity).read_text(encoding="utf-8"))
    inherited = validate_causal_reuse(
        root, shared, request.revision, request.state_digest
    )
    migrate_payload(root, payload, request.state_digest)
    receipts = payload.get("execution_receipts")
    if inherited is None or not isinstance(receipts, list):
        return
    inherited_keys = shared_main_observation_ids(shared, payload, inherited)
    if inherited_keys is None:
        return
    reject_invalid_staged_receipts(root, payload, receipts, inherited)
    shared_reuse = shared["causal_reuse"]
    payload["causal_reuse"] = {
        "schema": CAUSAL_REUSE_SCHEMA,
        "predecessor_artifact_sha256": payload["shared_main_artifact_digest"],
        "predecessor_source_revision": shared["source_revision"],
        "predecessor_source_state_digest": shared_reuse[
            "predecessor_source_state_digest"
        ],
        "predecessor_source_digest": shared["source_digest"],
        "predecessor_run_nonce": shared["run_nonce"],
        "claim_digest": request.current_claim,
        "exact_command": row["exact_command"],
        "source_artifact_bindings": source_artifact_bindings(
            root, row["exact_command"], row["requirement"]
        ),
        "execution_observation_ids": sorted(inherited_keys),
    }


def shared_main_observation_ids(
    shared: dict[str, object], payload: dict[str, object], inherited: set[str]
) -> set[str] | None:
    shared_receipts = receipt_ids_by_role(shared.get("execution_receipts"))
    payload_receipts = receipt_ids_by_role(payload.get("execution_receipts"))
    inherited_keys = {
        shared_receipts[role]
        for role in SHARED_MAIN_ROLES
        if role in shared_receipts
    }
    if (
        set(shared_receipts).intersection(SHARED_MAIN_ROLES) != set(SHARED_MAIN_ROLES)
        or any(payload_receipts.get(role) != shared_receipts[role] for role in SHARED_MAIN_ROLES)
        or not inherited.issuperset(inherited_keys)
    ):
        return None
    return inherited_keys


def receipt_ids_by_role(receipts: object) -> dict[object, object]:
    if not isinstance(receipts, list):
        return {}
    return {
        receipt.get("role"): receipt.get("observation_sha256")
        for receipt in receipts
        if isinstance(receipt, dict)
    }


def reject_invalid_staged_receipts(
    root: Path,
    payload: dict[str, object],
    receipts: list[object],
    inherited: set[str],
) -> None:
    observation_ids = {
        receipt.get("observation_sha256")
        for receipt in receipts
        if isinstance(receipt, dict)
    }
    if not all(isinstance(identity, str) for identity in observation_ids):
        raise RuntimeError("shared row has malformed execution lineage")
    extra_receipts = [
        receipt
        for receipt in receipts
        if isinstance(receipt, dict)
        and receipt.get("observation_sha256") not in inherited
    ]
    if extra_receipts and not available_receipts_match_payload_source(
        root, {**payload, "execution_receipts": extra_receipts}
    ):
        raise RuntimeError("shared row has invalid staged execution lineage")

from __future__ import annotations

import json
from dataclasses import dataclass
from pathlib import Path

from worth_ui_ledger_artifact_identity import (
    ArtifactIdentity,
    predecessor_handoff,
    predecessor_schema,
)
from worth_ui_ledger_candidate_basis import (
    CandidateBasis,
    from_path,
    verification_context_digest,
)


@dataclass(frozen=True)
class PredecessorVerification:
    root: Path
    ledger: Path
    phase: int
    revision: str
    state_digest: str


def expected_identity(
    verification: PredecessorVerification,
) -> tuple[ArtifactIdentity, CandidateBasis]:
    basis = from_path(verification.ledger, verification.phase - 1)
    context = verification_context_digest(
        verification.phase,
        verification.revision,
        verification.state_digest,
        basis,
    )
    return predecessor_handoff(verification.phase, context), basis


def is_current(
    identity: ArtifactIdentity,
    verification: PredecessorVerification,
) -> bool:
    expected, basis = expected_identity(verification)
    temporary = identity.relative_path.startswith(
        "workspaces/worth-ui/target/worth-ui-3141-verify-predecessor-"
    )
    if (temporary and identity.relative_path != expected.relative_path) or (
        not temporary
        and identity.relative_path
        != predecessor_handoff(verification.phase).relative_path
    ):
        return False
    try:
        payload = json.loads(
            identity.destination(verification.root).read_text(encoding="utf-8")
        )
    except (OSError, json.JSONDecodeError):
        return False
    observed_basis = payload.get("verification_basis")
    rows = payload.get("rows")
    if not isinstance(observed_basis, dict) or not isinstance(rows, list):
        return False
    observed_claims = [
        {"requirement": row.get("requirement"), "claim_digest": row.get("claim_digest")}
        for row in rows
        if isinstance(row, dict)
    ]
    return (
        payload.get("schema") == predecessor_schema(verification.phase)
        and payload.get("through_phase") == verification.phase - 1
        and payload.get("source_revision") == verification.revision
        and payload.get("source_state_digest") == verification.state_digest
        and observed_basis == basis.payload()
        and observed_claims == list(basis.claim_inventory)
    )

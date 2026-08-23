from __future__ import annotations

import csv
import hashlib
import io
import json
from dataclasses import dataclass
from pathlib import Path

from worth_ui_ledger_command import CLAIM_FIELDS, claim_digest_for_row


SCHEMA = "worth-ui-ledger-candidate-basis-v1"
CONTEXT_SCHEMA = "worth-ui-predecessor-verification-context-v1"
EXECUTION_INPUT_SCHEMA = "worth-ui-ledger-execution-input-v1"


@dataclass(frozen=True)
class CandidateBasis:
    through_phase: int
    candidate_prefix_digest: str
    claim_inventory: tuple[dict[str, str], ...]
    claim_inventory_digest: str

    def payload(self) -> dict[str, object]:
        return {
            "schema": SCHEMA,
            "through_phase": self.through_phase,
            "candidate_prefix_digest": self.candidate_prefix_digest,
            "claim_inventory": list(self.claim_inventory),
            "claim_inventory_digest": self.claim_inventory_digest,
        }


def from_path(identity: Path, through_phase: int) -> CandidateBasis:
    return from_text(identity.read_text(encoding="utf-8"), through_phase)


def from_text(content: str, through_phase: int) -> CandidateBasis:
    reader = csv.DictReader(io.StringIO(content, newline=""))
    fields = tuple(reader.fieldnames or ())
    rows = [row for row in reader if int(row["phase"]) <= through_phase]
    if not fields or not rows:
        raise ValueError("candidate ledger prefix is empty or lacks a header")
    normalized = [{field: row[field] for field in fields} for row in rows]
    inventory = tuple(
        {
            "requirement": row["requirement"],
            "claim_digest": claim_digest_for_row(row),
        }
        for row in rows
    )
    prefix = digest_json(
        {
            "schema": SCHEMA,
            "through_phase": through_phase,
            "fields": list(fields),
            "rows": normalized,
        }
    )
    inventory_digest = digest_json(
        {
            "schema": f"{SCHEMA}-claims",
            "through_phase": through_phase,
            "claims": inventory,
        }
    )
    return CandidateBasis(through_phase, prefix, inventory, inventory_digest)


def verification_context_digest(
    phase: int, revision: str, state_digest: str, basis: CandidateBasis
) -> str:
    if basis.through_phase != phase - 1:
        raise ValueError("candidate basis does not match predecessor phase")
    return digest_json(
        {
            "schema": CONTEXT_SCHEMA,
            "phase": phase,
            "source_revision": revision,
            "source_state_digest": state_digest,
            "candidate_prefix_digest": basis.candidate_prefix_digest,
            "claim_inventory_digest": basis.claim_inventory_digest,
        }
    )


def execution_input_digest(
    identity: Path, through_phase: int, requirement: str
) -> str:
    reader = csv.DictReader(io.StringIO(identity.read_text(encoding="utf-8"), newline=""))
    fields = tuple(reader.fieldnames or ())
    rows = [row for row in reader if int(row["phase"]) <= through_phase]
    owned = [row for row in rows if row["requirement"] == requirement]
    if not fields or len(owned) != 1:
        raise ValueError("ledger execution input lacks one exact governed row")
    normalized = []
    for row in rows:
        if row["requirement"] == requirement:
            normalized.append({"claim": {field: row[field] for field in CLAIM_FIELDS}})
        else:
            normalized.append({"evidence": {field: row[field] for field in fields}})
    return digest_json(
        {
            "schema": EXECUTION_INPUT_SCHEMA,
            "through_phase": through_phase,
            "fields": list(fields),
            "rows": normalized,
        }
    )


def digest_json(value: object) -> str:
    encoded = json.dumps(value, sort_keys=True, separators=(",", ":")).encode()
    return hashlib.sha256(encoded).hexdigest()

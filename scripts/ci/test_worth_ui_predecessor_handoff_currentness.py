from __future__ import annotations

import csv
import io
import json
import tempfile
import unittest
from pathlib import Path

from worth_ui_ledger_artifact_identity import ArtifactIdentity, predecessor_handoff
from worth_ui_ledger_candidate_basis import from_path, verification_context_digest
from worth_ui_ledger_command import CLAIM_FIELDS
from worth_ui_predecessor_handoff_currentness import (
    PredecessorVerification,
    is_current,
)


FIELDS = (*CLAIM_FIELDS, "command_result", "source_revision", "source_state_digest",
          "run_nonce", "result", "reopen_lineage", "result_artifact_digest")


def write_ledger(identity: Path, changes: dict[str, str] | None = None) -> None:
    stream = io.StringIO(newline="")
    writer = csv.DictWriter(stream, fieldnames=FIELDS, lineterminator="\n")
    writer.writeheader()
    row = {field: "value" for field in FIELDS}
    row.update({"phase": "1", "requirement": "P1-FIXTURE-01"})
    row.update(changes or {})
    writer.writerow(row)
    identity.write_text(stream.getvalue(), encoding="utf-8")


def write_handoff(
    root: Path, ledger: Path, revision: str, state: str
) -> ArtifactIdentity:
    basis = from_path(ledger, 1)
    context = verification_context_digest(2, revision, state, basis)
    identity = predecessor_handoff(2, context)
    destination = identity.destination(root)
    destination.parent.mkdir(parents=True)
    destination.write_text(json.dumps({
        "schema": "worth-ui-phase-predecessor-handoff-v4",
        "through_phase": 1,
        "source_revision": revision,
        "source_state_digest": state,
        "verification_basis": basis.payload(),
        "rows": list(basis.claim_inventory),
    }), encoding="utf-8")
    return identity


def verification(root: Path, ledger: Path) -> PredecessorVerification:
    return PredecessorVerification(root, ledger, 2, "a" * 40, "b" * 64)


class PredecessorHandoffCurrentnessTests(unittest.TestCase):
    def test_candidate_certification_mutations_reject_retained_handoff(self) -> None:
        for field in ("result", "run_nonce", "reopen_lineage"):
            with self.subTest(field=field), tempfile.TemporaryDirectory() as directory:
                root = Path(directory)
                ledger = root / "candidate.csv"
                write_ledger(ledger)
                identity = write_handoff(root, ledger, "a" * 40, "b" * 64)
                write_ledger(ledger, {field: "changed"})
                self.assertFalse(is_current(identity, verification(root, ledger)))

    def test_claim_mutation_and_legacy_schema_are_rejected(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            ledger = root / "candidate.csv"
            write_ledger(ledger)
            identity = write_handoff(root, ledger, "a" * 40, "b" * 64)
            write_ledger(ledger, {"owner": "changed"})
            self.assertFalse(is_current(identity, verification(root, ledger)))
            write_ledger(ledger)
            payload = json.loads(identity.destination(root).read_text(encoding="utf-8"))
            payload["schema"] = "worth-ui-phase-predecessor-handoff-v2"
            identity.destination(root).write_text(json.dumps(payload), encoding="utf-8")
            self.assertFalse(is_current(identity, verification(root, ledger)))

    def test_temporary_identity_changes_with_candidate_context(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            ledger = root / "candidate.csv"
            write_ledger(ledger)
            identity = write_handoff(root, ledger, "a" * 40, "b" * 64)
            self.assertTrue(is_current(identity, verification(root, ledger)))
            write_ledger(ledger, {"result": "changed"})
            changed = write_handoff(root, ledger, "a" * 40, "b" * 64)
            self.assertNotEqual(identity.relative_path, changed.relative_path)


if __name__ == "__main__":
    unittest.main()

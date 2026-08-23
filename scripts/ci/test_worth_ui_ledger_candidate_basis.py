from __future__ import annotations

import csv
import io
import unittest

from pathlib import Path
from tempfile import TemporaryDirectory

from worth_ui_ledger_candidate_basis import (
    execution_input_digest,
    from_text,
    verification_context_digest,
)
from worth_ui_ledger_command import CLAIM_FIELDS


FIELDS = (*CLAIM_FIELDS, "exact_command", "matched_test_count", "command_result",
          "retained_result_artifact", "source_revision", "source_digest",
          "source_state_digest", "run_nonce", "result", "reopen_lineage",
          "final_source", "result_artifact_digest")


def ledger(rows: list[dict[str, str]], newline: str = "\n") -> str:
    stream = io.StringIO(newline="")
    writer = csv.DictWriter(stream, fieldnames=FIELDS, lineterminator=newline)
    writer.writeheader()
    for row in rows:
        complete = {field: "value" for field in FIELDS}
        complete.update(row)
        writer.writerow(complete)
    return stream.getvalue()


class CandidateBasisTests(unittest.TestCase):
    def test_prefix_is_line_ending_independent_and_successor_independent(self) -> None:
        rows = [
            {"phase": "1", "requirement": "P1-A-01"},
            {"phase": "2", "requirement": "P2-A-01"},
        ]
        first = from_text(ledger(rows), 1)
        self.assertEqual(first, from_text(ledger(rows, "\r\n"), 1))
        changed_successor = [rows[0], {**rows[1], "result": "OPEN"}]
        self.assertEqual(first, from_text(ledger(changed_successor), 1))

    def test_certification_and_claim_mutations_have_exact_scope(self) -> None:
        row = {"phase": "1", "requirement": "P1-A-01"}
        baseline = from_text(ledger([row]), 1)
        for field in ("result", "result_artifact_digest", "run_nonce", "reopen_lineage"):
            changed = from_text(ledger([{**row, field: "changed"}]), 1)
            self.assertNotEqual(baseline.candidate_prefix_digest,
                                changed.candidate_prefix_digest)
            self.assertEqual(baseline.claim_inventory_digest,
                             changed.claim_inventory_digest)
        claim = from_text(ledger([{**row, "owner": "changed"}]), 1)
        self.assertNotEqual(baseline.claim_inventory_digest,
                            claim.claim_inventory_digest)
        self.assertNotEqual(
            verification_context_digest(2, "a" * 40, "b" * 64, baseline),
            verification_context_digest(2, "a" * 40, "b" * 64, claim),
        )

    def test_execution_input_excludes_only_the_evidence_produced_by_its_row(self) -> None:
        rows = [
            {"phase": "1", "requirement": "P1-A-01", "result": "OPEN"},
            {"phase": "1", "requirement": "P1-B-01", "result": "PROVED"},
        ]
        with TemporaryDirectory() as directory:
            identity = Path(directory) / "ledger.csv"

            def digest(candidates: list[dict[str, str]]) -> str:
                identity.write_text(ledger(candidates), encoding="utf-8")
                return execution_input_digest(identity, 1, "P1-A-01")

            baseline = digest(rows)
            self.assertEqual(baseline, digest([{**rows[0], "result": "PROVED"}, rows[1]]))
            self.assertNotEqual(baseline, digest([{**rows[0], "owner": "changed"}, rows[1]]))
            self.assertNotEqual(baseline, digest([rows[0], {**rows[1], "result": "OPEN"}]))


if __name__ == "__main__":
    unittest.main()

from __future__ import annotations

import csv
import io
import sys
import tempfile
import unittest
from pathlib import Path
from unittest import mock


CI = Path(__file__).resolve().parent
if str(CI) not in sys.path:
    sys.path.insert(0, str(CI))

import append_worth_ui_3141_phase6_rows as phase6_append


LEDGER = Path("_docs/worth-ui/milestone-3.14.1-proof-ledger.csv")
REQUIREMENT = "P6-INPUT-AFFINITY-01"


class PhaseSixLedgerAppendTests(unittest.TestCase):
    def test_open_contract_refresh_preserves_prefix_and_reopen_lineage(self) -> None:
        original = LEDGER.read_bytes()
        prefix = original[: original.index(b"6,P6-PREDECESSOR-01,")]
        fields, rows = parse_rows(original)
        row = next(item for item in rows if item["requirement"] == REQUIREMENT)
        row["owner"] = "stale-owner"
        row["reopen_lineage"] = "retained-invalidation"
        stale = serialize(fields, rows)

        updated = run_refresh(stale)

        self.assertTrue(updated.startswith(prefix))
        refreshed = requirement_row(updated, REQUIREMENT)
        self.assertEqual(refreshed["owner"], "worth-ui-host-native")
        self.assertEqual(refreshed["reopen_lineage"], "retained-invalidation")
        self.assertIn("observation/admission.rs", refreshed["source_identity"])
        self.assertIn("causal_mutation.rs", refreshed["source_identity"])

    def test_refresh_is_idempotent(self) -> None:
        refreshed = run_refresh(LEDGER.read_bytes())
        self.assertEqual(run_refresh(refreshed), refreshed)

    def test_proved_contract_is_not_rewritten(self) -> None:
        fields, rows = parse_rows(LEDGER.read_bytes())
        row = next(item for item in rows if item["requirement"] == REQUIREMENT)
        row["owner"] = "sealed-owner"
        row["result"] = "PROVED"
        row["final_source"] = "true"

        updated = run_refresh(serialize(fields, rows))

        sealed = requirement_row(updated, REQUIREMENT)
        self.assertEqual(sealed["owner"], "sealed-owner")
        self.assertEqual(sealed["result"], "PROVED")
        self.assertEqual(sealed["final_source"], "true")


def run_refresh(content: bytes) -> bytes:
    with tempfile.TemporaryDirectory() as directory:
        ledger = Path(directory) / "ledger.csv"
        ledger.write_bytes(content)
        with mock.patch.object(phase6_append, "LEDGER", ledger):
            if phase6_append.main() != 0:
                raise AssertionError("Phase 6 ledger refresh failed")
        return ledger.read_bytes()


def parse_rows(content: bytes) -> tuple[list[str], list[dict[str, str]]]:
    reader = csv.DictReader(io.StringIO(content.decode(), newline=""))
    fields = list(reader.fieldnames or ())
    return fields, list(reader)


def serialize(fields: list[str], rows: list[dict[str, str]]) -> bytes:
    stream = io.StringIO(newline="")
    writer = csv.DictWriter(stream, fieldnames=fields, lineterminator="\n")
    writer.writeheader()
    writer.writerows(rows)
    return stream.getvalue().encode()


def requirement_row(content: bytes, requirement: str) -> dict[str, str]:
    return next(
        row
        for row in csv.DictReader(io.StringIO(content.decode(), newline=""))
        if row["requirement"] == requirement
    )


if __name__ == "__main__":
    unittest.main()

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

import append_worth_ui_3141_phase5_rows as phase5_append


LEDGER = Path("_docs/worth-ui/milestone-3.14.1-proof-ledger.csv")
REQUIREMENT = b",P5-TEXT-ASYNC-PRESENTATION-01,"
CLOSE = b",P5-CLOSE-01,"


def ledger_without_async_row() -> bytes:
    return b"".join(
        line
        for line in LEDGER.read_bytes().splitlines(keepends=True)
        if REQUIREMENT not in line
    )


class PhaseFiveLedgerAppendTests(unittest.TestCase):
    def test_open_row_is_inserted_before_close_without_rewriting_prefix(self) -> None:
        original = ledger_without_async_row()
        close_line = next(
            line for line in original.splitlines(keepends=True) if CLOSE in line
        )
        first_phase_five = next(
            line
            for line in original.splitlines(keepends=True)
            if b"5,P5-PREDECESSOR-01," in line
        )
        historical_prefix = original[: original.index(first_phase_five)]
        with tempfile.TemporaryDirectory() as directory:
            ledger = Path(directory) / "ledger.csv"
            ledger.write_bytes(original)
            with mock.patch.object(phase5_append, "LEDGER", ledger):
                self.assertEqual(phase5_append.main(), 0)
            updated = ledger.read_bytes()

        self.assertTrue(updated.startswith(historical_prefix))
        self.assertTrue(updated.endswith(close_line))
        rows = list(csv.DictReader(io.StringIO(updated.decode(), newline="")))
        self.assertEqual(rows[-2]["requirement"], "P5-TEXT-ASYNC-PRESENTATION-01")
        self.assertEqual(rows[-2]["result"], "OPEN")
        self.assertEqual(rows[-1]["requirement"], "P5-CLOSE-01")

    def test_failed_candidate_validation_never_rewrites_ledger(self) -> None:
        original = ledger_without_async_row()
        with tempfile.TemporaryDirectory() as directory:
            ledger = Path(directory) / "ledger.csv"
            ledger.write_bytes(original)
            with (
                mock.patch.object(phase5_append, "LEDGER", ledger),
                mock.patch.object(
                    phase5_append,
                    "validate_candidate",
                    side_effect=RuntimeError("hostile candidate"),
                ),
            ):
                with self.assertRaisesRegex(RuntimeError, "hostile candidate"):
                    phase5_append.main()
            self.assertEqual(ledger.read_bytes(), original)

    def test_open_contract_refresh_preserves_the_historical_prefix(self) -> None:
        original = LEDGER.read_bytes()
        first_phase_five = next(
            line
            for line in original.splitlines(keepends=True)
            if b"5,P5-PREDECESSOR-01," in line
        )
        prefix = original[: original.index(first_phase_five)]
        with LEDGER.open(encoding="utf-8", newline="") as source:
            fields = list(csv.DictReader(source).fieldnames or ())
        lines = original.splitlines(keepends=True)
        cost_index = next(
            index for index, line in enumerate(lines) if b",P5-TEXT-COST-01," in line
        )
        cost = dict(zip(fields, next(csv.reader([lines[cost_index].decode()])), strict=True))
        cost["owner"] = "stale-owner"
        cost["structural_counters"] = "retained-scans=0"
        stream = io.StringIO(newline="")
        csv.DictWriter(stream, fieldnames=fields, lineterminator="\n").writerow(cost)
        lines[cost_index] = stream.getvalue().encode()
        stale = b"".join(lines)

        with tempfile.TemporaryDirectory() as directory:
            ledger = Path(directory) / "ledger.csv"
            ledger.write_bytes(stale)
            with mock.patch.object(phase5_append, "LEDGER", ledger):
                self.assertEqual(phase5_append.main(), 0)
            updated = ledger.read_bytes()

        self.assertTrue(updated.startswith(prefix))
        refreshed = next(
            row
            for row in csv.DictReader(io.StringIO(updated.decode(), newline=""))
            if row["requirement"] == "P5-TEXT-COST-01"
        )
        self.assertEqual(refreshed["owner"], "worth-ui-certification")
        self.assertEqual(refreshed["structural_counters"], "ui-locality-worlds=32")


if __name__ == "__main__":
    unittest.main()

from __future__ import annotations

import sys
import unittest
from pathlib import Path


CI = Path(__file__).resolve().parent
if str(CI) not in sys.path:
    sys.path.insert(0, str(CI))

import close_worth_ui_3141_ledger as ledger_closer


class LedgerPhaseSelectionTests(unittest.TestCase):
    def test_current_phase_source_drift_does_not_rewrite_predecessors(self) -> None:
        rows = [
            {
                "phase": "3", "requirement": "P3-ONE", "result": "PROVED",
                "final_source": "true", "source_state_digest": "old",
            },
            {
                "phase": "4", "requirement": "P4-ONE", "result": "PROVED",
                "final_source": "true", "source_state_digest": "old",
            },
        ]
        selected = ledger_closer.phase_rows_to_prepare(
            rows, 4, None, {"P4-ONE": object()}, "current"
        )
        self.assertEqual([row["requirement"] for row in selected], ["P4-ONE"])
        self.assertEqual(rows[0]["result"], "PROVED")

    def test_multiple_stale_phase_rows_refresh_atomically(self) -> None:
        rows = [
            {
                "phase": "5", "requirement": "P5-ONE", "result": "PROVED",
                "final_source": "true", "source_state_digest": "old",
            },
            {
                "phase": "5", "requirement": "P5-TWO", "result": "OPEN",
                "final_source": "false", "source_state_digest": "not-bound",
            },
            {
                "phase": "5", "requirement": "P5-THREE", "result": "PROVED",
                "final_source": "true", "source_state_digest": "current",
            },
        ]
        configured = {row["requirement"]: object() for row in rows}
        selected = ledger_closer.phase_rows_to_prepare(
            rows, 5, ["P5-ONE", "P5-TWO"], configured, "current"
        )
        self.assertEqual(
            [row["requirement"] for row in selected], ["P5-ONE", "P5-TWO"]
        )
        with self.assertRaisesRegex(RuntimeError, "P5-THREE is not one open"):
            ledger_closer.phase_rows_to_prepare(
                rows, 5, ["P5-ONE", "P5-THREE"], configured, "current"
            )


if __name__ == "__main__":
    unittest.main()

from __future__ import annotations

import sys
import hashlib
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch


CI = Path(__file__).resolve().parent
if str(CI) not in sys.path:
    sys.path.insert(0, str(CI))

import close_worth_ui_3141_ledger as ledger_closer
import worth_ui_ledger_closure_selection as closure_selection


class LedgerPhaseSelectionTests(unittest.TestCase):
    def test_read_ledger_uses_the_selection_owner_identity(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            identity = Path(directory) / "ledger.csv"
            identity.write_text("phase,requirement\n5,P5-ONE\n", encoding="utf-8")
            with patch.object(closure_selection, "LEDGER", identity):
                fields, rows = closure_selection.read_ledger()
        self.assertEqual(fields, ["phase", "requirement"])
        self.assertEqual(rows, [{"phase": "5", "requirement": "P5-ONE"}])

    def select(self, *arguments, prepare=None):
        operation = prepare or (lambda row, proof: None)
        with patch.object(ledger_closer, "prepare_claim", side_effect=operation):
            return ledger_closer.phase_rows_to_prepare(*arguments)

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
        selected = self.select(
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
        selected = self.select(
            rows, 5, ["P5-ONE", "P5-TWO"], configured, "current"
        )
        self.assertEqual(
            [row["requirement"] for row in selected], ["P5-ONE", "P5-TWO"]
        )
        with self.assertRaisesRegex(RuntimeError, "P5-THREE is not one open"):
            self.select(
                rows, 5, ["P5-ONE", "P5-THREE"], configured, "current"
            )

    def test_unrelated_global_state_drift_keeps_causally_current_row(self) -> None:
        identity = "scripts/ci/test_worth_ui_ledger_phase_selection.py"
        rows = [
            {
                "phase": "5",
                "requirement": "P5-ONE",
                "result": "PROVED",
                "final_source": "true",
                "source_state_digest": "old-global-state",
                "source_identity": identity,
                "source_digest": ledger_closer.source_digest((identity,)),
            }
        ]
        with patch.object(
            closure_selection, "row_has_current_causal_binding", return_value=True
        ):
            selected = self.select(
                rows, 5, None, {"P5-ONE": object()}, "new-global-state"
            )
        self.assertEqual(selected, [])

    def test_dependency_authentication_drift_selects_the_bound_row(self) -> None:
        identity = "scripts/ci/test_worth_ui_ledger_phase_selection.py"
        artifact = Path("workspaces/worth-ui/target/phase-selection.json")
        artifact.parent.mkdir(parents=True, exist_ok=True)
        content = b"{}"
        artifact.write_bytes(content)
        row = {
            **{field: "value" for field in closure_selection.CLAIM_FIELDS},
            "phase": "5",
            "requirement": "P5-ONE",
            "result": "PROVED",
            "final_source": "true",
            "source_identity": identity,
            "source_digest": ledger_closer.source_digest((identity,)),
            "retained_result_artifact": "workspaces/worth-ui/target/phase-selection.json",
            "result_artifact_digest": hashlib.sha256(content).hexdigest(),
        }
        try:
            selected = self.select(
                [row], 5, None, {"P5-ONE": object()}, "unrelated-global-state"
            )
            self.assertEqual(selected, [row])
        finally:
            artifact.unlink(missing_ok=True)

    def test_declared_causal_source_drift_reopens_the_row(self) -> None:
        identity = "scripts/ci/test_worth_ui_ledger_phase_selection.py"
        rows = [
            {
                "phase": "5",
                "requirement": "P5-ONE",
                "result": "PROVED",
                "final_source": "true",
                "source_state_digest": "current-global-state",
                "source_identity": identity,
                "source_digest": "stale-causal-digest",
            }
        ]
        selected = self.select(
            rows, 5, None, {"P5-ONE": object()}, "current-global-state"
        )
        self.assertEqual(selected, rows)

    def test_claim_command_or_oracle_drift_reopens_row_and_phase_close(self) -> None:
        identity = "scripts/ci/test_worth_ui_ledger_phase_selection.py"
        base = {
            "phase": "5",
            "result": "PROVED",
            "final_source": "true",
            "source_state_digest": "old-global-state",
            "source_identity": identity,
            "source_digest": ledger_closer.source_digest((identity,)),
            "production_entry": "owner::old",
            "independent_oracle": "oracle::old",
            "exact_command": "old-command",
            "retained_result_artifact": "row.json",
        }
        row = {**base, "requirement": "P5-ONE"}
        close = {**base, "requirement": "P5-CLOSE-01"}

        def current_claim(candidate, proof):
            if candidate["requirement"] == "P5-ONE":
                candidate["production_entry"] = "owner::current"
                candidate["independent_oracle"] = "oracle::current"
                candidate["exact_command"] = "current-command"

        selected = self.select(
            [row, close],
            5,
            None,
            {"P5-ONE": object(), "P5-CLOSE-01": object()},
            "new-global-state",
            prepare=current_claim,
        )
        self.assertEqual(
            [candidate["requirement"] for candidate in selected],
            ["P5-ONE", "P5-CLOSE-01"],
        )

    def test_phase_five_predecessor_transaction_owns_nested_handoffs(self) -> None:
        rows = [
            {
                "phase": "1",
                "requirement": "P1-ROW-01",
                "retained_result_artifact": "evidence/p1-row.json",
            },
            {
                "phase": "5",
                "requirement": "P5-PREDECESSOR-01",
                "retained_result_artifact": "evidence/p5-predecessor.json",
            },
        ]
        identities = ledger_closer.transaction_extra_identities(
            rows, [rows[1]], 5
        )
        self.assertEqual(
            set(identities),
            {
                "_docs/worth-ui/milestone-3.14.1-evidence/compile-contracts.json",
                "_docs/worth-ui/milestone-3.14.1-evidence/p3-predecessor-handoff.json",
                "_docs/worth-ui/milestone-3.14.1-evidence/p4-predecessor-handoff.json",
                "_docs/worth-ui/milestone-3.14.1-evidence/p5-predecessor-handoff.json",
                "_docs/worth-ui/milestone-3.14.1-evidence/p5-closure-portfolio.json",
                "evidence/p5-predecessor.json",
                "evidence/p1-row.json",
            },
        )

    def test_pinning_refresh_reissues_its_atlas_evidence_first(self) -> None:
        rows = [
            {
                "phase": "5", "requirement": "P5-ATLAS-01",
                "result": "PROVED", "final_source": "true",
            },
            {
                "phase": "5", "requirement": "P5-ATLAS-PINNING-01",
                "result": "PROVED", "final_source": "true",
            },
            {
                "phase": "5", "requirement": "P5-CLOSE-01",
                "result": "PROVED", "final_source": "true",
            },
        ]
        configured = {row["requirement"]: object() for row in rows}

        def current(row, *_arguments):
            return row["requirement"] != "P5-ATLAS-PINNING-01"

        with patch.object(
            closure_selection, "row_has_current_causal_binding", side_effect=current
        ):
            selected = self.select(rows, 5, None, configured, "current")
        self.assertEqual(
            [row["requirement"] for row in selected],
            ["P5-ATLAS-01", "P5-ATLAS-PINNING-01", "P5-CLOSE-01"],
        )

    def test_predecessor_refresh_reissues_async_compile_artifact_evidence(self) -> None:
        rows = [
            {
                "phase": "5", "requirement": "P5-PREDECESSOR-01",
                "result": "PROVED", "final_source": "true",
            },
            {
                "phase": "5", "requirement": "P5-TEXT-ASYNC-PRESENTATION-01",
                "result": "PROVED", "final_source": "true",
            },
            {
                "phase": "5", "requirement": "P5-CLOSE-01",
                "result": "PROVED", "final_source": "true",
            },
        ]
        configured = {row["requirement"]: object() for row in rows}

        def current(row, *_arguments):
            return row["requirement"] != "P5-PREDECESSOR-01"

        with patch.object(
            closure_selection, "row_has_current_causal_binding", side_effect=current
        ):
            selected = self.select(rows, 5, None, configured, "current")
        self.assertEqual(
            [row["requirement"] for row in selected],
            [
                "P5-PREDECESSOR-01",
                "P5-TEXT-ASYNC-PRESENTATION-01",
                "P5-CLOSE-01",
            ],
        )

if __name__ == "__main__":
    unittest.main()

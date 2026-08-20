from __future__ import annotations

import csv
import argparse
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

import verify_worth_ui_3141_ledger as verifier
import worth_ui_ledger_command as ledger_command
import worth_ui_ledger_operational_successors as portfolio
import worth_ui_ledger_portfolio_snapshot as snapshot
from worth_ui_ledger_runner_authentication import RunnerProvenanceUnavailable


LEDGER = Path("_docs/worth-ui/milestone-3.14.1-proof-ledger.csv")


class OperationalVerificationTests(unittest.TestCase):
    def test_default_verifier_validates_retained_portfolio_without_reexecution(self) -> None:
        with (
            patch.object(
                verifier,
                "parse_args",
                return_value=argparse.Namespace(through_phase=4, artifact=None),
            ),
            patch.object(verifier, "source_revision", return_value="a" * 40),
            patch.object(verifier, "source_state_digest", return_value="b" * 64),
            patch.object(
                verifier,
                "retained_source_binding",
                return_value=("a" * 40, "b" * 64),
            ),
            patch.object(verifier, "validate_retained_portfolio") as retained,
            patch.object(verifier, "closure_tests", return_value=2) as closure,
            patch.object(
                verifier,
                "execute_portfolio",
                side_effect=AssertionError("retained validation reexecuted the portfolio"),
            ),
        ):
            self.assertEqual(verifier.main(), 0)
        retained.assert_called_once()
        closure.assert_called_once()

    def test_historical_retained_binding_forces_current_source_revalidation(self) -> None:
        arguments = argparse.Namespace(through_phase=4, artifact=None)
        with (
            patch.object(verifier, "parse_args", return_value=arguments),
            patch.object(verifier, "source_revision", return_value="a" * 40),
            patch.object(verifier, "source_state_digest", return_value="b" * 64),
            patch.object(
                verifier,
                "retained_source_binding",
                return_value=("c" * 40, "d" * 64),
            ),
            patch.object(verifier, "persist_referenced_receipts") as persist,
            patch.object(verifier, "validate_retained_portfolio") as retained,
            patch.object(verifier, "closure_tests") as closure,
            patch.object(verifier, "execute_current_portfolio") as execute,
        ):
            self.assertEqual(verifier.main(), 0)
        execute.assert_called_once_with(arguments, "a" * 40, "b" * 64)
        persist.assert_not_called()
        retained.assert_not_called()
        closure.assert_not_called()

    def test_foreign_runner_provenance_triggers_operational_revalidation(self) -> None:
        arguments = argparse.Namespace(through_phase=4, artifact=None)
        with (
            patch.object(verifier, "parse_args", return_value=arguments),
            patch.object(verifier, "source_revision", return_value="a" * 40),
            patch.object(verifier, "source_state_digest", return_value="b" * 64),
            patch.object(verifier, "retained_source_binding", return_value=("a" * 40, "b" * 64)),
            patch.object(verifier, "persist_referenced_receipts"),
            patch.object(
                verifier, "validate_retained_portfolio",
                side_effect=RunnerProvenanceUnavailable("foreign key"),
            ),
            patch.object(verifier, "execute_portfolio", return_value=([], 2)) as execute,
        ):
            self.assertEqual(verifier.main(), 0)
        execute.assert_called_once_with(arguments)

    def test_verifier_executes_rows_inside_one_revision_bound_source_snapshot(self) -> None:
        revision = "a" * 40
        digest = "b" * 64
        observed = []

        def execute(_arguments: argparse.Namespace):
            observed.append(snapshot.source_state_for_row(revision))
            return [], 2

        with (
            patch.object(
                verifier,
                "parse_args",
                return_value=argparse.Namespace(
                    through_phase=3, artifact="target/predecessor.json"
                ),
            ),
            patch.object(verifier, "source_revision", side_effect=[revision, revision]),
            patch.object(verifier, "source_state_digest", return_value=digest),
            patch.object(verifier, "execute_portfolio", side_effect=execute),
            patch.object(verifier, "write_artifact"),
            patch.object(snapshot, "source_state_digest", return_value="c" * 64),
            patch.dict(
                "os.environ",
                {snapshot.REVISION_ENV: "", snapshot.DIGEST_ENV: ""},
                clear=False,
            ),
        ):
            self.assertEqual(verifier.main(), 0)
        self.assertEqual(observed, [digest])

    def test_portfolio_snapshot_rejects_a_different_revision(self) -> None:
        with snapshot.operational_source_snapshot("a" * 40, "b" * 64):
            with self.assertRaisesRegex(RuntimeError, "revision drifted"):
                snapshot.source_state_for_row("c" * 40)

    def test_claim_digest_reads_the_exact_candidate_claim(self) -> None:
        with LEDGER.open(encoding="utf-8", newline="") as stream:
            reader = csv.DictReader(stream)
            fields = list(reader.fieldnames or ())
            rows = list(reader)
        canonical = ledger_command.claim_digest("P1-CLOSE-01")
        next(row for row in rows if row["requirement"] == "P1-CLOSE-01")[
            "scenario_delta"
        ] = "candidate-only-claim"
        with tempfile.TemporaryDirectory() as directory:
            candidate = Path(directory) / "candidate.csv"
            with candidate.open("w", encoding="utf-8", newline="") as stream:
                writer = csv.DictWriter(stream, fieldnames=fields, lineterminator="\n")
                writer.writeheader()
                writer.writerows(rows)
            with patch.dict(
                "os.environ", {"WORTH_UI_MILESTONE_3141_LEDGER": str(candidate)}
            ):
                self.assertNotEqual(
                    ledger_command.claim_digest("P1-CLOSE-01"), canonical
                )

    def test_candidate_reopens_then_promotes_exact_rows(self) -> None:
        with LEDGER.open(encoding="utf-8", newline="") as stream:
            governed = [row for row in csv.DictReader(stream) if row["phase"] == "1"]
        replacements = portfolio.prepared_open_rows(governed)
        self.assertTrue(all(row["result"] == "OPEN" for row in replacements.values()))
        row = governed[0]
        payload = {
            "matched_test_count": 1,
            "source_revision": "revision",
            "source_digest": "source",
            "source_state_digest": "state",
            "run_nonce": "nonce",
            "artifact_sha256": "artifact",
            "executed_exact_command": row["exact_command"].replace(
                row["retained_result_artifact"], "fresh.json"
            ),
            "source_identity": row["source_identity"].split(";"),
        }
        with tempfile.TemporaryDirectory() as directory:
            candidate = Path(directory) / "candidate.csv"
            portfolio.record_proved_execution(
                row, "fresh.json", payload, replacements, LEDGER, candidate
            )
            with candidate.open(encoding="utf-8", newline="") as stream:
                observed = {item["requirement"]: item for item in csv.DictReader(stream)}
        self.assertEqual(observed[row["requirement"]]["result"], "PROVED")
        self.assertEqual(
            observed[row["requirement"]]["retained_failure_artifact"], "fresh.json"
        )
        self.assertEqual(
            observed[row["requirement"]]["retained_result_artifact"], "fresh.json"
        )
        still_open = next(key for key in replacements if key != row["requirement"])
        self.assertEqual(observed[still_open]["result"], "OPEN")

    def test_predecessor_candidate_reopens_out_of_scope_successors(self) -> None:
        reopened = verifier.successor_reopenings(2)
        self.assertTrue(reopened)
        self.assertTrue(all(int(row["phase"]) > 2 for row in reopened.values()))
        self.assertTrue(all(row["result"] == "OPEN" for row in reopened.values()))
        self.assertTrue(
            all(row["final_source"] == "false" for row in reopened.values())
        )

    def test_operational_candidate_reopens_every_governed_phase_before_reexecution(
        self,
    ) -> None:
        governed = [
            {"phase": "1", "requirement": "P1-ROW", "result": "PROVED"},
            {"phase": "2", "requirement": "P2-ROW", "result": "PROVED"},
            {"phase": "3", "requirement": "P3-ROW", "result": "PROVED"},
            {"phase": "4", "requirement": "P4-ROW", "result": "PROVED"},
        ]
        with patch.object(verifier, "successor_reopenings", return_value={}):
            reopened = verifier.operational_reopenings(4, governed)
        self.assertEqual(set(reopened), {row["requirement"] for row in governed})
        self.assertTrue(all(row["result"] == "OPEN" for row in reopened.values()))
        self.assertTrue(
            all(row["final_source"] == "false" for row in reopened.values())
        )

    def test_candidate_claim_is_rebound_before_its_execution(self) -> None:
        with LEDGER.open(encoding="utf-8", newline="") as stream:
            row = next(
                item for item in csv.DictReader(stream)
                if item["requirement"] == "P1-WORLDS-01"
            )
        replacements = portfolio.prepared_open_rows([row])
        with tempfile.TemporaryDirectory() as directory:
            candidate = Path(directory) / "candidate.csv"
            portfolio.write_candidate_ledger(LEDGER, candidate, replacements)
            command = row["exact_command"].split()
            command[command.index("--artifact") + 1] = "target/fresh-world.json"
            first_source = command.index("--source") + 1
            command[first_source] = "target/fresh-compile.json"
            portfolio.stage_execution_claim(
                candidate, row, "target/fresh-world.json", command
            )
            with candidate.open(encoding="utf-8", newline="") as stream:
                staged = next(
                    item for item in csv.DictReader(stream)
                    if item["requirement"] == row["requirement"]
                )
        self.assertEqual(staged["retained_failure_artifact"], "target/fresh-world.json")
        self.assertEqual(staged["retained_result_artifact"], "target/fresh-world.json")
        self.assertIn("--artifact target/fresh-world.json", staged["exact_command"])
        self.assertIn("target/fresh-compile.json", staged["source_identity"])

    def test_final_closure_checks_are_bound_to_the_fresh_candidate(self) -> None:
        candidate = Path("candidate.csv")
        with patch.object(verifier, "run") as run:
            self.assertEqual(verifier.closure_tests(3, candidate), 2)
        self.assertEqual([call.args[1] for call in run.call_args_list], [candidate, candidate])

    def test_phase_three_executes_one_flat_predecessor_portfolio(self) -> None:
        governed = [
            {"phase": "1", "requirement": "P1-HISTORICAL"},
            {"phase": "2", "requirement": "P2-HISTORICAL"},
            {"phase": "3", "requirement": "P3-PREDECESSOR-01"},
        ]
        arguments = argparse.Namespace(through_phase=3, artifact=None)
        class FlatPhaseTwo:
            fresh_compile = "fresh-compile.json"

            def __init__(self, *_args):
                pass

            def execute(self):
                return [
                    {"requirement": "P1-HISTORICAL"},
                    {"requirement": "P2-HISTORICAL"},
                ]

        with (
            patch.object(verifier, "rows", return_value=governed),
            patch.object(verifier, "successor_reopenings", return_value={}),
            patch.object(verifier, "write_candidate_ledger"),
            patch.object(verifier, "PhaseTwoPortfolioExecution", FlatPhaseTwo),
            patch.object(verifier, "predecessor_artifact", return_value={}),
            patch.object(verifier, "write_artifact") as write_artifact,
            patch.object(verifier, "PhaseThreePortfolioExecution") as phase_three,
            patch.object(verifier, "closure_tests", return_value=2),
            patch.object(verifier, "run") as run,
        ):
            observations, closure_count = verifier.execute_portfolio(arguments)
        run.assert_not_called()
        self.assertEqual(closure_count, 2)
        self.assertEqual(
            observations,
            [
                {"requirement": "P1-HISTORICAL"},
                {"requirement": "P2-HISTORICAL"},
            ],
        )
        write_artifact.assert_called_once()
        staged = phase_three.call_args.args[4]
        self.assertEqual(staged, [governed[2]])
        self.assertEqual(phase_three.call_args.args[7], "fresh-compile.json")
        phase_three.return_value.execute.assert_called_once()

from __future__ import annotations

import argparse
import csv
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
                return_value=argparse.Namespace(
                    through_phase=4, refresh_predecessor_for_phase=None
                ),
            ),
            patch.object(verifier, "source_revision", return_value="a" * 40),
            patch.object(verifier, "source_state_digest", return_value="b" * 64),
            patch.object(
                verifier,
                "retained_source_binding",
                return_value=("a" * 40, "b" * 64),
            ),
            patch.object(verifier, "validate_retained_portfolio") as retained,
            patch.object(verifier, "validate_current_causal_sources") as causal,
        ):
            self.assertEqual(verifier.main(), 0)
        retained.assert_called_once()
        causal.assert_called_once_with(4)

    def test_stale_global_binding_fails_without_reexecution(self) -> None:
        arguments = argparse.Namespace(
            through_phase=4, refresh_predecessor_for_phase=None
        )
        with (
            patch.object(verifier, "parse_args", return_value=arguments),
            patch.object(verifier, "source_revision", return_value="a" * 40),
            patch.object(verifier, "source_state_digest", return_value="b" * 64),
            patch.object(
                verifier,
                "retained_source_binding",
                return_value=("c" * 40, "d" * 64),
            ),
        ):
            with self.assertRaisesRegex(RuntimeError, "stale for the live source state"):
                verifier.main()

    def test_foreign_runner_provenance_fails_without_fallback_execution(self) -> None:
        arguments = argparse.Namespace(
            through_phase=4, refresh_predecessor_for_phase=None
        )
        with (
            patch.object(verifier, "parse_args", return_value=arguments),
            patch.object(verifier, "source_revision", return_value="a" * 40),
            patch.object(verifier, "source_state_digest", return_value="b" * 64),
            patch.object(
                verifier,
                "retained_source_binding",
                return_value=("a" * 40, "b" * 64),
            ),
            patch.object(
                verifier,
                "validate_retained_portfolio",
                side_effect=RunnerProvenanceUnavailable("foreign key"),
            ),
            patch.object(verifier, "validate_current_causal_sources") as causal,
        ):
            with self.assertRaises(RunnerProvenanceUnavailable):
                verifier.main()
        causal.assert_not_called()

    def test_artifact_refresh_uses_selective_predecessor_builder(self) -> None:
        with (
            patch.object(
                verifier,
                "parse_args",
                return_value=argparse.Namespace(
                    through_phase=3, refresh_predecessor_for_phase=4
                ),
            ),
            patch.object(verifier, "ledger_identity", return_value=Path("ledger.csv")),
            patch.object(verifier, "refresh_handoff") as refresh,
        ):
            self.assertEqual(verifier.main(), 0)
        refresh.assert_called_once_with(verifier.ROOT, Path("ledger.csv"), 4)

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
        changed = next(
            row for row in rows if row["requirement"] == "P1-CLOSE-01"
        )
        changed["scenario_delta"] = "candidate-only-claim"
        with tempfile.TemporaryDirectory() as directory:
            candidate = Path(directory) / "candidate.csv"
            with candidate.open("w", encoding="utf-8", newline="") as stream:
                writer = csv.DictWriter(stream, fieldnames=fields, lineterminator="\n")
                writer.writeheader()
                writer.writerows(rows)
            with patch.dict(
                "os.environ", {"WORTH_UI_MILESTONE_3141_LEDGER": str(candidate)}
            ):
                candidate_digest = ledger_command.claim_digest("P1-CLOSE-01")
                self.assertEqual(
                    candidate_digest, ledger_command.claim_digest_for_row(changed)
                )
                self.assertNotEqual(candidate_digest, canonical)

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
            observed[row["requirement"]]["retained_result_artifact"], "fresh.json"
        )
        still_open = next(key for key in replacements if key != row["requirement"])
        self.assertEqual(observed[still_open]["result"], "OPEN")

    def test_candidate_claim_is_rebound_before_its_execution(self) -> None:
        with LEDGER.open(encoding="utf-8", newline="") as stream:
            row = next(
                item
                for item in csv.DictReader(stream)
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
                    item
                    for item in csv.DictReader(stream)
                    if item["requirement"] == row["requirement"]
                )
        self.assertEqual(staged["retained_failure_artifact"], "target/fresh-world.json")
        self.assertEqual(staged["retained_result_artifact"], "target/fresh-world.json")
        self.assertIn("--artifact target/fresh-world.json", staged["exact_command"])
        self.assertIn("target/fresh-compile.json", staged["source_identity"])

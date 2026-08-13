import subprocess
import tempfile
import unittest
import sys
from pathlib import Path
from unittest.mock import Mock, patch

sys.path.insert(0, str(Path(__file__).resolve().parent))

import run_worth_ui_compile_contracts as compile_runner
import run_worth_ui_ledger_test as ledger_runner
import close_worth_ui_3141_ledger as ledger_closer
import verify_worth_ui_3141_ledger as ledger_verifier
import worth_ui_ledger_operational_successors as successor_verifier
from worth_ui_ledger_phase_three_portfolio import PhaseThreePortfolioExecution
import worth_ui_ledger_dependency as ledger_dependency
import worth_ui_3141_supporting_world as supporting_world
from worth_ui_3141_proof_plan import prepare_claim, proofs
from verify_worth_ui_3141_ledger import bind_fresh_compile_artifact
from worth_ui_ledger_source_state import source_state_digest
from worth_ui_ledger_dependency_tests import LedgerDependencyTests
from worth_ui_ledger_operational_verification_tests import OperationalVerificationTests
from worth_ui_predecessor_handoff_tests import PredecessorHandoffCostTests
from worth_ui_ledger_runner_race_tests import LedgerRunnerSettlementTests


class GovernedRaceTests(unittest.TestCase):
    def test_operational_verifier_reads_the_candidate_ledger_when_bound(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            candidate = Path(directory) / "candidate.csv"
            candidate.write_text("phase,requirement\n3,P3-CLOSE-01\n", encoding="utf-8")
            with patch.dict(
                "os.environ",
                {"WORTH_UI_MILESTONE_3141_LEDGER": str(candidate)},
            ):
                self.assertEqual(ledger_verifier.ledger_identity(), candidate.resolve())

    def test_phase_three_close_reads_the_authoritative_candidate_not_dependency_rebindings(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            candidate = root / "authoritative.csv"
            candidate.write_text("candidate", encoding="utf-8")
            captured = []

            def rerun(_row, _artifact, _compile, **options):
                captured.append(options["candidate_ledger"])
                return {"requirement": "P3-CLOSE-01"}

            with patch(
                "worth_ui_ledger_phase_three_portfolio.record_proved_execution"
            ):
                PhaseThreePortfolioExecution(
                    root, candidate, root, root / "dependency.csv",
                    [{"phase": "3", "requirement": "P3-CLOSE-01"}], {},
                    rerun, "compile.json", [], frozenset(),
                ).execute()
            self.assertEqual(captured, [root / "dependency.csv"])

    def test_phase_three_dependents_reuse_their_exact_producer_world(self) -> None:
        for requirement in ("P3-HEADLESS-COST-01", "P3-PRODUCER-SLOPE-01"):
            self.assertEqual(
                successor_verifier.shared_artifact(requirement, "mixed", "native"),
                "mixed",
            )
        self.assertEqual(
            successor_verifier.shared_artifact(
                "P3-DAMAGE-REPLAY-01", "mixed", "native"
            ),
            "native",
        )
        command = ["runner", "--source", ledger_verifier.P3_DELTA_ARTIFACT]
        self.assertEqual(
            ledger_verifier.bind_fresh_shared_world(command, "fresh-mixed.json"),
            ["runner", "--source", "fresh-mixed.json"],
        )

    def test_phase_three_proofs_bind_main_oracles_and_named_entry_sources(self) -> None:
        for requirement, proof in proofs().items():
            if not requirement.startswith("P3-"):
                continue
            oracle_source, oracle_symbol = proof.oracle_entry.rsplit("::", 1)
            production_source = proof.production_entry.rsplit("::", 1)[0]
            self.assertEqual(proof.test_name.rsplit("::", 1)[-1], oracle_symbol)
            self.assertIn(oracle_source, proof.sources)
            self.assertIn(production_source, proof.sources)

    def test_mixed_world_budget_matches_the_immutable_rust_contract(self) -> None:
        for requirement in (
            "P3-DELTA-SOURCE-01",
            "P3-HEADLESS-COST-01",
            "P3-PRODUCER-SLOPE-01",
        ):
            self.assertEqual(ledger_runner.execution_budget_ms(requirement), 90_000)
        self.assertEqual(ledger_runner.execution_budget_ms("P3-CLOSE-01"), 60_000)
        self.assertEqual(ledger_runner.execution_budget_ms("P4-BIDI-01"), 180_000)

    def test_test_budget_excludes_cargo_wrapper_overhead(self) -> None:
        from worth_ui_ledger_command import exact_test_duration_ms

        output = (
            "test result: ok. 1 passed; finished in 1.00s\n"
            "test result: ok. 1 passed; finished in 59.30s\n"
        )
        self.assertEqual(exact_test_duration_ms(output, 60_018), 59_300)
        self.assertEqual(
            exact_test_duration_ms("test result: ok; finished in 0.00s\n", 600), 1
        )
        self.assertEqual(exact_test_duration_ms("no libtest summary", 60_018), 60_018)

    def test_phase_four_predecessor_owns_the_unique_current_portfolio_cost(self) -> None:
        from worth_ui_3141_ledger_contracts import construction_cost, execution_cost

        self.assertEqual(
            construction_cost("P4-PREDECESSOR-01"),
            "main-tests=26;hostile-controls=27;product-processes=3;compile-sessions=2;"
            "courtroom-worlds=6",
        )
        self.assertEqual(
            execution_cost("P4-PREDECESSOR-01"),
            "executed-tests=55;presentations=28",
        )

    def test_phase_three_control_requires_the_exact_immutable_mutation_case(self) -> None:
        prefix = "WORTH_UI_LEDGER_MUTATION_CONTROLS="
        lawful = prefix + '{"P3-BASELINE-REPLAY-01":"opaque-baseline-clear"}'
        self.assertEqual(
            ledger_runner.mutation_control_observation(
                lawful, "P3-BASELINE-REPLAY-01"
            ),
            {
                "requirement": "P3-BASELINE-REPLAY-01",
                "case": "opaque-baseline-clear",
            },
        )
        for hostile in [
            "",
            prefix + '{"P3-BASELINE-REPLAY-01":"wrong"}',
            prefix + '{"P3-DAMAGE-REPLAY-01":"opaque-baseline-clear"}',
            lawful + "\n" + lawful,
        ]:
            self.assertIsNone(
                ledger_runner.mutation_control_observation(
                    hostile, "P3-BASELINE-REPLAY-01"
                )
            )

    def test_every_phase_three_proof_has_unique_source_identities(self) -> None:
        for requirement, proof in proofs().items():
            if requirement.startswith("P3-"):
                self.assertEqual(
                    len(proof.sources),
                    len(set(proof.sources)),
                    requirement,
                )

    def test_ignored_candidate_updates_do_not_change_governed_source_state(self) -> None:
        revision = ledger_runner.source_revision()
        before = source_state_digest(revision)
        candidate_root = Path("workspaces/worth-ui/target/milestone-3141-candidates")
        candidate_root.mkdir(parents=True, exist_ok=True)
        candidate = candidate_root / "race-test.csv"
        try:
            candidate.write_text("OPEN", encoding="utf-8")
            candidate.write_text("PROVED", encoding="utf-8")
            self.assertEqual(source_state_digest(revision), before)
        finally:
            candidate.unlink(missing_ok=True)

    def test_fresh_dependent_execution_reads_the_advanced_candidate_ledger(self) -> None:
        import csv
        import json

        with Path("_docs/worth-ui/milestone-3.14.1-proof-ledger.csv").open(
            encoding="utf-8", newline=""
        ) as stream:
            row = next(
                row for row in csv.DictReader(stream)
                if row["requirement"] == "P1-HEADLESS-COST-01"
            )
        payload = {
            "exit_posture": "passed",
            "requirement": row["requirement"],
            "source_identity": row["source_identity"].split(";"),
        }
        completed = Mock(returncode=0, stdout=json.dumps(payload) + "\n", stderr="")
        target = Path("workspaces/worth-ui/target")
        target.mkdir(parents=True, exist_ok=True)
        with tempfile.TemporaryDirectory(dir=target) as directory:
            root = Path(directory)
            candidate = root / "candidate.csv"
            candidate.write_bytes(
                Path("_docs/worth-ui/milestone-3.14.1-proof-ledger.csv").read_bytes()
            )
            with patch.object(subprocess, "run", return_value=completed) as run:
                observation = ledger_verifier.rerun_row(
                    row, root / "result.json", "compile.json", candidate_ledger=candidate
                )
            self.assertEqual(run.call_args.args[0][0], sys.executable)
            self.assertEqual(observation["executed_exact_command"].split()[0], "python")
            environment = run.call_args.kwargs["env"]
            self.assertEqual(
                environment["WORTH_UI_MILESTONE_3141_LEDGER"],
                str(candidate.resolve()),
            )

    def test_phase_three_world_consumes_the_fresh_delta_world(self) -> None:
        import csv
        import json

        with Path("_docs/worth-ui/milestone-3.14.1-proof-ledger.csv").open(
            encoding="utf-8", newline=""
        ) as stream:
            row = next(
                row for row in csv.DictReader(stream)
                if row["requirement"] == "P3-HP02-WORLD-01"
            )
        fresh = (
            "workspaces/worth-ui/target/"
            "worth-ui-3141-verify-test/fresh-delta-world.json"
        )
        fresh_path = Path(fresh)
        fresh_path.parent.mkdir(parents=True, exist_ok=True)
        fresh_path.write_text("fresh delta evidence", encoding="utf-8")
        executed_sources = list(proofs()[row["requirement"]].sources)
        executed_sources[executed_sources.index(supporting_world.MIXED_ARTIFACT)] = fresh
        payload = {
            "exit_posture": "passed",
            "requirement": row["requirement"],
            "source_identity": executed_sources,
        }
        completed = Mock(returncode=0, stdout=json.dumps(payload) + "\n", stderr="")
        target = Path("workspaces/worth-ui/target")
        target.mkdir(parents=True, exist_ok=True)
        with tempfile.TemporaryDirectory(dir=target) as directory:
            try:
                with patch.object(subprocess, "run", return_value=completed) as run:
                    ledger_verifier.rerun_row(
                        row,
                        Path(directory) / "result.json",
                        "compile.json",
                        supporting_world_artifact=fresh,
                    )
            finally:
                fresh_path.unlink(missing_ok=True)
            command = run.call_args.args[0]
            environment = run.call_args.kwargs["env"]
            self.assertIn(fresh, command)
            self.assertNotIn(supporting_world.MIXED_ARTIFACT, command)
            self.assertEqual(environment["WORTH_UI_SUPPORTING_WORLD_ARTIFACT"], fresh)

    def test_phase_preparation_preserves_the_proved_prefix_byte_for_byte(self) -> None:
        fields = ["phase", "requirement", "result", "final_source"]
        original = (
            "phase,requirement,result,final_source\n"
            "1,P1-ONE,PROVED,true\n"
            "2,P2-ONE,PROVED,true\n"
            "3,P3-ONE,OPEN,false\n"
            "4,P4-ONE,OPEN,false\n"
        )
        rows = list(csv_rows(original))
        selected = ledger_closer.phase_rows_to_prepare(
            rows, 3, "P3-ONE", {"P3-ONE": object()}
        )
        self.assertEqual([row["requirement"] for row in selected], ["P3-ONE"])
        selected[0]["result"] = "PROVED"
        selected[0]["final_source"] = "true"
        rendered = ledger_closer.render_requirement_update(
            original, rows, fields, {"P3-ONE"}
        )
        self.assertTrue(rendered.startswith(
            "phase,requirement,result,final_source\n"
            "1,P1-ONE,PROVED,true\n"
            "2,P2-ONE,PROVED,true\n"
        ))
        self.assertIn("3,P3-ONE,PROVED,true\n", rendered)
        self.assertTrue(rendered.endswith("4,P4-ONE,OPEN,false\n"))

    def test_phase_finalization_refuses_an_incomplete_successor_mapping(self) -> None:
        rows = [
            {"phase": "3", "requirement": "P3-ONE"},
            {"phase": "3", "requirement": "P3-TWO"},
        ]
        with self.assertRaisesRegex(RuntimeError, "proof mappings are incomplete"):
            ledger_closer.require_complete_phase_mapping(rows, 3, {"P3-ONE"})

    def test_current_phase_source_drift_reopens_without_rewriting_predecessors(self) -> None:
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

    def test_historical_phase_two_rows_never_reopen_on_later_source_drift(self) -> None:
        rows = [
            {
                "phase": "2", "requirement": "P2-ONE", "result": "PROVED",
                "final_source": "true", "source_state_digest": "historical",
            },
        ]
        selected = ledger_closer.phase_rows_to_prepare(
            rows, 2, None, {"P2-ONE": object()}, "current"
        )
        self.assertEqual(selected, [])
        self.assertEqual(rows[0]["result"], "PROVED")
        self.assertEqual(rows[0]["final_source"], "true")

    def test_phase_four_preparation_binds_the_exact_text_profile_digest(self) -> None:
        import hashlib

        row = {"requirement": "P4-BIDI-01"}
        proof = proofs()[row["requirement"]]
        from worth_ui_3141_proof_plan import prepare_claim

        prepare_claim(row, proof)
        manifest = Path(
            "workspaces/worth-ui/profiles/worth-ui-global-text-v2/manifest.toml"
        )
        self.assertEqual(row["font_profile_identity"], "worth-ui-global-text-v2")
        self.assertEqual(
            row["font_profile_digest"], hashlib.sha256(manifest.read_bytes()).hexdigest()
        )

    def test_expensive_hostile_controls_own_the_same_twenty_second_budget(self) -> None:
        from worth_ui_ledger_command import control_budget_ms

        self.assertEqual(control_budget_ms("P3-PREDECESSOR-01"), 20_000)
        self.assertEqual(control_budget_ms("P4-PREDECESSOR-01"), 20_000)
        self.assertEqual(control_budget_ms("P4-FONT-COLLECTION-01"), 20_000)
        self.assertEqual(control_budget_ms("P4-TEXT-RECONSTRUCTION-01"), 20_000)

    def test_operational_verifier_rebinds_compile_backed_rows(self) -> None:
        canonical = "_docs/worth-ui/milestone-3.14.1-evidence/compile-contracts.json"
        fresh = "workspaces/worth-ui/target/fresh-compile-contracts.json"
        command = ["runner", "--source", canonical, "--source", "production.rs"]
        self.assertEqual(
            bind_fresh_compile_artifact(command, fresh),
            ["runner", "--source", fresh, "--source", "production.rs"],
        )

    def test_compile_snapshot_rejects_each_post_execution_drift(self) -> None:
        before = {
            "source_revision": "a",
            "source_state_digest": "b",
            "cases": [{"source_sha256": "c", "stderr_sha256": "d"}],
        }
        self.assertFalse(compile_runner.governed_snapshot_changed(before, dict(before)))
        for field, value in [
            ("source_revision", "changed"),
            ("source_state_digest", "changed"),
            ("cases", [{"source_sha256": "changed", "stderr_sha256": "d"}]),
        ]:
            after = dict(before)
            after[field] = value
            self.assertTrue(compile_runner.governed_snapshot_changed(before, after), field)

    def test_ledger_snapshot_rejects_source_and_claim_drift(self) -> None:
        before = ("revision", "sources", "state", "claim")
        self.assertFalse(ledger_runner.governed_snapshot_changed(before, before))
        for index in range(len(before)):
            after = list(before)
            after[index] = "changed"
            self.assertTrue(ledger_runner.governed_snapshot_changed(before, tuple(after)), index)

    def test_compile_publication_refuses_drift_without_writing(self) -> None:
        before = {"source_revision": "a", "source_state_digest": "b", "cases": []}
        after = {**before, "source_state_digest": "changed"}
        writer = Mock()
        with patch.object(compile_runner, "write_artifact", writer):
            admitted = compile_runner.publish_artifact_if_unchanged(
                "evidence.json", {"exit_posture": "passed"}, before, after
            )
        self.assertFalse(admitted)
        writer.assert_not_called()

def csv_rows(content: str):
    import csv
    import io

    return csv.DictReader(io.StringIO(content))


if __name__ == "__main__":
    unittest.main()

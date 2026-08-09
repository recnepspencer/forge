import subprocess
import unittest
import sys
from pathlib import Path
from unittest.mock import Mock, patch

sys.path.insert(0, str(Path(__file__).resolve().parent))

import run_worth_ui_compile_contracts as compile_runner
import run_worth_ui_ledger_test as ledger_runner
from verify_worth_ui_3141_ledger import bind_fresh_compile_artifact


class GovernedRaceTests(unittest.TestCase):
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

    def test_ledger_execution_rejects_claim_drift_at_the_real_settlement(self) -> None:
        test = ledger_runner.GovernedTest(
            "P1-AFFINITY-01", "worth-ui-runtime", "lib", "lib", (),
            "owner::exact_test", ("source.rs",), "artifact.json", None,
        )
        listed = completed("owner::exact_test: test\n")
        ignored = completed("")
        executed = completed("test result: ok. 1 passed; 0 failed; 0 ignored;\n")
        expected_costs = (
            ledger_runner.construction_cost(test.requirement),
            ledger_runner.execution_cost(test.requirement),
        )
        with (
            patch.object(ledger_runner, "timed_run", side_effect=[(listed, 1), (ignored, 1), (executed, 1)]),
            patch.object(ledger_runner, "source_revision", return_value="revision"),
            patch.object(ledger_runner, "source_digest", return_value="sources"),
            patch.object(ledger_runner, "source_state_digest", return_value="state"),
            patch.object(ledger_runner, "claim_digest", side_effect=["before", "after"]),
            patch.object(ledger_runner, "p1_counter_observation", return_value="work=3"),
            patch.object(ledger_runner, "observed_costs", return_value=expected_costs),
        ):
            payload, exit_code = ledger_runner.result_payload(test)
        self.assertEqual(payload["exit_posture"], "source-changed")
        self.assertEqual(exit_code, 1)


def completed(stdout: str) -> subprocess.CompletedProcess[str]:
    return subprocess.CompletedProcess([], 0, stdout, "")


if __name__ == "__main__":
    unittest.main()

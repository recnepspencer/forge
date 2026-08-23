from __future__ import annotations

import subprocess
import unittest
from unittest.mock import patch

from worth_ui_3141_ledger_contracts import construction_cost, execution_cost
from worth_ui_ledger_command import GovernedTest
import worth_ui_ledger_governed_snapshot as governed_snapshot
import worth_ui_ledger_row_evidence as row_evidence


class LedgerRunnerSettlementTests(unittest.TestCase):
    def test_ledger_execution_rejects_claim_drift_at_the_real_settlement(self) -> None:
        test = GovernedTest(
            "P1-AFFINITY-01", "worth-ui-runtime", "lib", "lib", (),
            "owner::exact_test", ("source.rs",), "artifact.json", None,
        )
        listed = completed("owner::exact_test: test\n")
        ignored = completed("")
        executed = completed("test result: ok. 1 passed; 0 failed; 0 ignored;\n")
        expected_costs = (
            construction_cost(test.requirement),
            execution_cost(test.requirement),
        )
        with (
            patch.object(
                row_evidence,
                "timed_execution",
                side_effect=[
                    (listed, 1, execution_receipt("list")),
                    (ignored, 1, execution_receipt("ignored")),
                    (executed, 1, execution_receipt("main")),
                ],
            ),
            patch.object(governed_snapshot, "source_revision", return_value="revision"),
            patch.object(governed_snapshot, "source_digest", return_value="sources"),
            patch.object(governed_snapshot, "source_state_for_row", return_value="state"),
            patch.object(governed_snapshot, "claim_digest", side_effect=["before", "after"]),
            patch.object(row_evidence, "p1_counter_observation", return_value="work=3"),
            patch.object(row_evidence, "observed_costs", return_value=expected_costs),
        ):
            payload, exit_code = row_evidence.result_payload(test)
        self.assertEqual(payload["exit_posture"], "source-changed")
        self.assertEqual(exit_code, 1)

    def test_predecessor_execution_refreshes_the_handoff_before_discovery(self) -> None:
        test = GovernedTest(
            "P3-PREDECESSOR-01", "worth-ui-certification", "test",
            "topology_contracts", (), "owner::exact_test",
            ("_docs/worth-ui/milestone-3.14.1-evidence/p3-predecessor-handoff.json",),
            "artifact.json", None,
        )
        listed = completed("owner::exact_test: test\n")
        executed = completed("test result: ok. 1 passed; 0 failed; 0 ignored;\n")
        expected_costs = (
            construction_cost(test.requirement),
            execution_cost(test.requirement),
        )
        with (
            patch.object(governed_snapshot, "refresh_predecessor_handoff") as refresh,
            patch.object(
                row_evidence,
                "timed_execution",
                side_effect=[
                    (listed, 1, execution_receipt("list")),
                    (listed, 1, execution_receipt("ignored")),
                    (executed, 1, execution_receipt("main")),
                ],
            ),
            patch.object(governed_snapshot, "source_revision", return_value="revision"),
            patch.object(governed_snapshot, "source_digest", return_value="sources"),
            patch.object(governed_snapshot, "source_state_for_row", return_value="state"),
            patch.object(governed_snapshot, "claim_digest", return_value="claim"),
            patch.object(
                row_evidence, "p1_counter_observation", return_value="requirements=30"
            ),
            patch.object(row_evidence, "observed_costs", return_value=expected_costs),
        ):
            payload, exit_code = row_evidence.result_payload(test)
        refresh.assert_called_once_with(test)
        self.assertEqual(payload["exit_posture"], "passed")
        self.assertEqual(exit_code, 0)


def completed(stdout: str) -> subprocess.CompletedProcess[str]:
    return subprocess.CompletedProcess([], 0, stdout, "")


def execution_receipt(identity: str) -> dict[str, object]:
    return {
        "key": identity.ljust(64, "0"),
        "command_sha256": "a" * 64,
        "duration_ms": 1,
        "reused": False,
    }

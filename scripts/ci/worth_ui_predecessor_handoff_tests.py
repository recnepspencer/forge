import json
import subprocess
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

import run_worth_ui_ledger_test as ledger_runner
import verify_worth_ui_3141_ledger as ledger_verifier
import worth_ui_ledger_operational_successors as successors
import worth_ui_ledger_governed_snapshot as governed_snapshot
from worth_ui_ledger_verifier_rebinding import bind_fresh_predecessor_handoff
from worth_ui_ledger_runner_authentication import authenticates
from worth_ui_3141_proof_plan import prepare_claim, proofs


class PredecessorHandoffCostTests(unittest.TestCase):
    def test_supplied_predecessor_handoff_prevents_recursive_portfolio_replay(self) -> None:
        with tempfile.TemporaryDirectory(dir=ledger_verifier.TARGET) as directory:
            identity = Path(directory) / "p3-predecessor-handoff.json"
            identity.write_text("{}", encoding="utf-8")
            source = identity.relative_to(ledger_verifier.ROOT).as_posix()
            test = ledger_runner.GovernedTest(
                "P3-PREDECESSOR-01", "package", "test", "target", (), "owner::test",
                (source,), "result.json", None,
            )
            with (
                patch.dict("os.environ", {"WORTH_UI_PREDECESSOR_ARTIFACT": source}),
                patch.object(governed_snapshot, "refresh_predecessor_handoff") as refresh,
            ):
                governed_snapshot.refresh_handoff_when_required(test)
            refresh.assert_not_called()

    def test_predecessor_handoff_input_is_distinct_from_row_result(self) -> None:
        row = {"requirement": "P3-PREDECESSOR-01"}
        prepare_claim(row, proofs()["P3-PREDECESSOR-01"])
        handoffs = [
            source for source in row["source_identity"].split(";")
            if source.endswith("p3-predecessor-handoff.json")
        ]
        self.assertEqual(len(handoffs), 1)
        self.assertNotEqual(handoffs[0], row["retained_result_artifact"])
        command = ["runner", "--source", handoffs[0]]
        self.assertEqual(
            bind_fresh_predecessor_handoff(command, "fresh.json"),
            ["runner", "--source", "fresh.json"],
        )

    def test_nested_replay_does_not_consume_the_outer_candidate(self) -> None:
        test = ledger_runner.GovernedTest(
            "P3-PREDECESSOR-01", "worth-ui-certification", "test",
            "topology_contracts", (), "owner::test",
            ("workspaces/worth-ui/target/p3-predecessor-handoff.json",),
            "result.json", None,
        )
        completed = subprocess.CompletedProcess([], 0, "", "")
        with (
            patch.dict(
                "os.environ",
                {
                    "WORTH_UI_MILESTONE_3141_LEDGER": "outer.csv",
                    "WORTH_UI_SHARED_WORLD_ARTIFACT": "shared.json",
                },
            ),
            patch.object(subprocess, "run", return_value=completed) as run,
        ):
            ledger_runner.refresh_predecessor_handoff(test)
        environment = run.call_args.kwargs["env"]
        self.assertNotIn("WORTH_UI_MILESTONE_3141_LEDGER", environment)
        self.assertNotIn("WORTH_UI_SHARED_WORLD_ARTIFACT", environment)

    def test_predecessor_handoff_derives_unique_process_and_world_costs(self) -> None:
        retained_defaults = {
            "package": "package",
            "target_kind": "test",
            "target_name": "target",
            "features": [],
            "test_name": "test",
            "matched_test_count": 1,
            "declared_ignored_test_count": 0,
            "expected_declared_ignored": False,
            "passed_test_count": 1,
            "ignored_test_count": 0,
            "exit_posture": "passed",
            "source_revision": "revision",
            "source_identity": [],
            "source_rebindings": [],
            "source_digest": "digest",
            "source_state_digest": "state",
            "run_nonce": "nonce",
            "artifact_sha256": "artifact",
            "runner_authentication": "machine-authenticated",
            "structural_counter": "counter=1",
            "hostile_control": {},
        }
        observations = [
            observation(
                retained_defaults,
                "P2-WORLD-01",
                "product-processes=1;courtroom-worlds=1",
                "presentations=1",
                "p2",
            ),
            observation(
                retained_defaults,
                "P3-DELTA-SOURCE-01",
                "product-processes=0;courtroom-worlds=1",
                "presentations=5",
                "mixed",
            ),
            observation(
                retained_defaults,
                "P3-HP02-WORLD-01",
                "product-processes=1;courtroom-worlds=1",
                "presentations=7",
                "native",
            ),
            {
                **observation(
                    retained_defaults,
                    "P3-HEADLESS-COST-01",
                    "product-processes=0;courtroom-worlds=1",
                    "presentations=5",
                    "mixed-support",
                ),
                "shared_main_artifact": "mixed.json",
            },
        ]
        artifact = ledger_verifier.predecessor_artifact(
            3, "revision", "state", observations, 1
        )
        self.assertEqual(artifact["product_processes"], 2)
        self.assertEqual(artifact["courtroom_worlds"], 3)
        self.assertEqual(artifact["presentations"], 13)
        self.assertEqual(
            artifact["rows"][0]["construction_cost"],
            "product-processes=1;courtroom-worlds=1",
        )
        self.assertNotEqual(
            artifact["rows"][0]["runner_authentication"],
            "machine-authenticated",
        )
        projected = dict(artifact["rows"][0])
        tag = projected.pop("runner_authentication")
        projected.pop("artifact_sha256", None)
        self.assertTrue(authenticates(projected, tag, ledger_verifier.ROOT))
        projected["test_name"] = "substituted"
        self.assertFalse(authenticates(projected, tag, ledger_verifier.ROOT))
        self.assertEqual(artifact["rows"][3]["shared_main_artifact"], "mixed.json")

    def test_outer_portfolio_imports_the_exact_fresh_predecessor_rows(self) -> None:
        with tempfile.TemporaryDirectory(
            prefix="worth-ui-predecessor-import-", dir=ledger_verifier.TARGET
        ) as directory:
            temporary = Path(directory)
            identity = temporary / "p3-predecessor-handoff.json"
            rows = [
                {"requirement": "P1-ONE", "exit_posture": "passed"},
                {"requirement": "P2-TWO", "exit_posture": "passed"},
            ]
            identity.write_text(
                json.dumps({
                    "schema": "worth-ui-phase-predecessor-handoff-v1",
                    "through_phase": 2,
                    "source_revision": "revision",
                    "source_state_digest": "state",
                    "rows": rows,
                }),
                encoding="utf-8",
            )
            observation = {
                "source_identity": [identity.relative_to(ledger_verifier.ROOT).as_posix()],
                "source_revision": "revision",
                "source_state_digest": "state",
            }
            self.assertEqual(
                successors.predecessor_observations(
                    observation, temporary, ledger_verifier.ROOT,
                    {"P1-ONE", "P2-TWO"},
                ),
                rows,
            )
            rows[1]["exit_posture"] = "test-failed"
            identity.write_text(
                json.dumps({
                    "schema": "worth-ui-phase-predecessor-handoff-v1",
                    "through_phase": 2,
                    "source_revision": "revision",
                    "source_state_digest": "state",
                    "rows": rows,
                }),
                encoding="utf-8",
            )
            with self.assertRaisesRegex(RuntimeError, "incomplete or non-passing"):
                successors.predecessor_observations(
                    observation, temporary, ledger_verifier.ROOT,
                    {"P1-ONE", "P2-TWO"},
                )


def observation(defaults, requirement, construction, execution, entry):
    return {
        **defaults,
        "requirement": requirement,
        "executed_test_count": 1,
        "construction_cost": construction,
        "execution_cost": execution,
        "mapping_source_identity": [],
        "production_entry": entry,
        "independent_oracle": f"{entry}-oracle",
    }

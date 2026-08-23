import json
import secrets
import shutil
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

import worth_ui_ledger_operational_successors as successors
import worth_ui_ledger_governed_snapshot as governed_snapshot
from worth_ui_ledger_command import GovernedTest, ROOT
from worth_ui_ledger_candidate_basis import CandidateBasis
from worth_ui_predecessor_handoff import predecessor_artifact
from worth_ui_ledger_verifier_rebinding import bind_fresh_predecessor_handoff
from worth_ui_ledger_runner_authentication import authenticates
from worth_ui_3141_proof_plan import prepare_claim, proofs


TARGET = ROOT / "workspaces/worth-ui/target"


class PredecessorHandoffCostTests(unittest.TestCase):
    def test_temporary_predecessor_handoff_namespace_is_exact(self) -> None:
        digest = "a" * 64
        valid = (
            "workspaces/worth-ui/target/"
            f"worth-ui-3141-verify-predecessor-{digest}/p3-predecessor-handoff.json"
        )
        self.assertTrue(governed_snapshot.is_temporary_predecessor_handoff(valid, 3))
        self.assertFalse(
            governed_snapshot.is_temporary_predecessor_handoff(
                valid.replace("a" * 64, "A" * 64), 3
            )
        )
        self.assertFalse(
            governed_snapshot.is_temporary_predecessor_handoff(
                valid.replace("p3-predecessor", "p4-predecessor"), 3
            )
        )
        self.assertFalse(
            governed_snapshot.is_temporary_predecessor_handoff(
                valid.replace("workspaces/worth-ui/target", "target"), 3
            )
        )

    def test_supplied_predecessor_handoff_is_selectively_refreshed(self) -> None:
        digest = secrets.token_hex(32)
        directory = TARGET / f"worth-ui-3141-verify-predecessor-{digest}"
        directory.mkdir(parents=True)
        try:
            identity = directory / "p3-predecessor-handoff.json"
            identity.write_text("{}", encoding="utf-8")
            source = identity.relative_to(ROOT).as_posix()
            test = GovernedTest(
                "P3-PREDECESSOR-01", "package", "test", "target", (), "owner::test",
                (source,), "result.json", None,
            )
            with (
                patch.dict("os.environ", {"WORTH_UI_PREDECESSOR_ARTIFACT": source}),
                patch.object(governed_snapshot, "refresh_predecessor_handoff") as refresh,
            ):
                governed_snapshot.refresh_handoff_when_required(test)
                refresh.assert_called_once_with(test)
        finally:
            shutil.rmtree(directory)

    def test_each_predecessor_handoff_input_is_distinct_and_rebindable(self) -> None:
        for phase in (3, 4, 5):
            requirement = f"P{phase}-PREDECESSOR-01"
            row = {"requirement": requirement}
            prepare_claim(row, proofs()[requirement])
            handoffs = [
                source for source in row["source_identity"].split(";")
                if source.endswith(f"p{phase}-predecessor-handoff.json")
            ]
            self.assertEqual(len(handoffs), 1, requirement)
            self.assertNotEqual(handoffs[0], row["retained_result_artifact"])
            command = ["runner", "--source", handoffs[0]]
            self.assertEqual(
                bind_fresh_predecessor_handoff(command, "fresh.json", phase),
                ["runner", "--source", "fresh.json"],
            )

    def test_selective_refresh_uses_the_exact_bound_candidate(self) -> None:
        test = GovernedTest(
            "P3-PREDECESSOR-01", "worth-ui-certification", "test",
            "topology_contracts", (), "owner::test",
            (
                "workspaces/worth-ui/target/"
                f"worth-ui-3141-verify-predecessor-{'a' * 64}/"
                "p3-predecessor-handoff.json",
            ),
            "result.json", None,
        )
        with (
            patch.dict(
                "os.environ",
                {
                    "WORTH_UI_MILESTONE_3141_LEDGER": "outer.csv",
                    "WORTH_UI_SHARED_WORLD_ARTIFACT": "shared.json",
                },
            ),
            patch.object(governed_snapshot, "refresh_handoff") as refresh,
        ):
            governed_snapshot.refresh_predecessor_handoff(test)
        self.assertEqual(refresh.call_args.args[1], Path("outer.csv").resolve())
        self.assertEqual(refresh.call_args.args[2], 3)
        self.assertEqual(refresh.call_args.args[3].relative_path, test.sources[0])

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
            "executed_exact_command": "cargo test --exact owner::test",
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
        with patch("worth_ui_predecessor_handoff.aggregate_executions", return_value=[]):
            artifact = predecessor_artifact(
                3,
                "revision",
                "state",
                observations,
                1,
                CandidateBasis(3, "a" * 64, (), "b" * 64),
            )
        self.assertEqual(artifact["product_processes"], 2)
        self.assertEqual(artifact["courtroom_worlds"], 3)
        self.assertEqual(artifact["presentations"], 13)
        self.assertEqual(
            artifact["rows"][0]["construction_cost"],
            "product-processes=1;courtroom-worlds=1",
        )
        self.assertEqual(
            artifact["rows"][0]["executed_exact_command"],
            retained_defaults["executed_exact_command"],
        )
        self.assertNotEqual(
            artifact["rows"][0]["runner_authentication"],
            "machine-authenticated",
        )
        projected = dict(artifact["rows"][0])
        tag = projected.pop("runner_authentication")
        self.assertTrue(authenticates(projected, tag, ROOT))
        projected["test_name"] = "substituted"
        self.assertFalse(authenticates(projected, tag, ROOT))
        projected = dict(artifact["rows"][0])
        tag = projected.pop("runner_authentication")
        projected["artifact_sha256"] = "0" * 64
        self.assertFalse(authenticates(projected, tag, ROOT))
        self.assertEqual(artifact["rows"][3]["shared_main_artifact"], "mixed.json")

    def test_outer_portfolio_imports_the_exact_fresh_predecessor_rows(self) -> None:
        with tempfile.TemporaryDirectory(
            prefix="worth-ui-predecessor-import-", dir=TARGET
        ) as directory:
            temporary = Path(directory)
            identity = temporary / "p3-predecessor-handoff.json"
            rows = [
                {"requirement": "P1-ONE", "exit_posture": "passed"},
                {"requirement": "P2-TWO", "exit_posture": "passed"},
            ]
            identity.write_text(
                json.dumps({
                    "schema": "worth-ui-phase-predecessor-handoff-v4",
                    "through_phase": 2,
                    "source_revision": "revision",
                    "source_state_digest": "state",
                    "rows": rows,
                }),
                encoding="utf-8",
            )
            observation = {
                "source_identity": [identity.relative_to(ROOT).as_posix()],
                "source_revision": "revision",
                "source_state_digest": "state",
            }
            self.assertEqual(
                successors.predecessor_observations(
                    observation, temporary, ROOT,
                    {"P1-ONE", "P2-TWO"},
                ),
                rows,
            )
            rows[1]["exit_posture"] = "test-failed"
            identity.write_text(
                json.dumps({
                    "schema": "worth-ui-phase-predecessor-handoff-v4",
                    "through_phase": 2,
                    "source_revision": "revision",
                    "source_state_digest": "state",
                    "rows": rows,
                }),
                encoding="utf-8",
            )
            with self.assertRaisesRegex(RuntimeError, "incomplete or non-passing"):
                successors.predecessor_observations(
                    observation, temporary, ROOT,
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

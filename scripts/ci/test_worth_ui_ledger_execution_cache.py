from __future__ import annotations

import csv
import json
import os
import sys
import tempfile
import unittest
from pathlib import Path
from unittest import mock

sys.path.insert(0, str(Path(__file__).resolve().parent))

from worth_ui_ledger_execution_cache import (
    CACHE_ENV,
    CANDIDATE_LEDGER_ENV,
    COMPILE_ARTIFACT,
    COMPILE_ARTIFACT_ENV,
    PREDECESSOR_ARTIFACT_ENV,
    causal_artifact_dependencies,
    digest_json,
    execution_binding,
    invalidate_receipts,
    timed_execution,
)
from worth_ui_ledger_command import CLAIM_FIELDS


LEDGER_FIELDS = (*CLAIM_FIELDS, "exact_command", "retained_result_artifact", "result", "final_source")


def write_ledger(identity: Path, rows: list[dict[str, str]]) -> None:
    with identity.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=LEDGER_FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)


def ledger_row(phase: int, requirement: str) -> dict[str, str]:
    row = {field: f"{field}:{requirement}" for field in LEDGER_FIELDS}
    row.update({"phase": str(phase), "requirement": requirement, "result": "OPEN", "final_source": "false"})
    return row


class ExecutionCacheTests(unittest.TestCase):
    def test_exact_command_executes_once_per_source_state(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            with mock.patch.dict(os.environ, {CACHE_ENV: str(root / "cache")}):
                with mock.patch("subprocess.run") as run:
                    run.return_value.returncode = 0
                    run.return_value.stdout = "passed"
                    run.return_value.stderr = ""
                    first = timed_execution(["cargo", "test", "exact"], root, "a" * 40, "b" * 64, "main")
                    second = timed_execution(["cargo", "test", "exact"], root, "a" * 40, "b" * 64, "control")
                self.assertEqual(run.call_count, 1)
                self.assertFalse(first[2]["reused"])
                self.assertTrue(second[2]["reused"])

    def test_source_or_command_drift_executes_fresh(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            with mock.patch.dict(os.environ, {CACHE_ENV: str(root / "cache")}):
                with mock.patch("subprocess.run") as run:
                    run.return_value.returncode = 0
                    run.return_value.stdout = "passed"
                    run.return_value.stderr = ""
                    timed_execution(["cargo", "test", "a"], root, "a" * 40, "b" * 64, "main")
                    timed_execution(["cargo", "test", "b"], root, "a" * 40, "b" * 64, "main")
                    timed_execution(["cargo", "test", "a"], root, "a" * 40, "c" * 64, "main")
                self.assertEqual(run.call_count, 3)

    def test_corrupted_receipt_is_never_reused(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            cache = root / "cache"
            with mock.patch.dict(os.environ, {CACHE_ENV: str(cache)}):
                with mock.patch("subprocess.run") as run:
                    run.return_value.returncode = 0
                    run.return_value.stdout = "passed"
                    run.return_value.stderr = ""
                    timed_execution(["cargo", "test", "exact"], root, "a" * 40, "b" * 64, "main")
                    receipt = next(cache.rglob("*.json"))
                    payload = json.loads(receipt.read_text(encoding="utf-8"))
                    payload["record"]["stdout"] = "forged"
                    payload["receipt_sha256"] = digest_json(payload["record"])
                    receipt.write_text(json.dumps(payload), encoding="utf-8")
                    timed_execution(["cargo", "test", "exact"], root, "a" * 40, "b" * 64, "main")
                self.assertEqual(run.call_count, 2)

    def test_candidate_ledger_binds_only_ledger_consuming_commands(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            candidate = root / "candidate.csv"
            rows = [
                ledger_row(4, "P4-SHAPING-01"),
                ledger_row(5, "P5-COLOR-EMOJI-01"),
                ledger_row(5, "P5-CLOSE-01"),
            ]
            write_ledger(candidate, rows)
            with mock.patch.dict(
                os.environ,
                {"WORTH_UI_MILESTONE_3141_LEDGER": str(candidate)},
                clear=False,
            ):
                ordinary_before = execution_binding(
                    ["cargo", "test", "ordinary::test"], root, "a" * 40, "b" * 64
                )
                ledger_before = execution_binding(
                    ["cargo", "test", "milestone_3141_phase1_ledger::closure"],
                    root,
                    "a" * 40,
                    "b" * 64,
                    (CANDIDATE_LEDGER_ENV,),
                    "P5-PREDECESSOR-01",
                )
                rows[0]["result"] = "PROVED"
                rows[0]["final_source"] = "true"
                rows[1]["owner"] = "phase-five-only-change"
                write_ledger(candidate, rows)
                ordinary_after = execution_binding(
                    ["cargo", "test", "ordinary::test"], root, "a" * 40, "b" * 64
                )
                settlement_after = execution_binding(
                    ["cargo", "test", "milestone_3141_phase1_ledger::closure"],
                    root,
                    "a" * 40,
                    "b" * 64,
                    (CANDIDATE_LEDGER_ENV,),
                    "P5-PREDECESSOR-01",
                )
                rows[0]["owner"] = "phase-four-claim-change"
                write_ledger(candidate, rows)
                claim_after = execution_binding(
                    ["cargo", "test", "milestone_3141_phase1_ledger::closure"],
                    root, "a" * 40, "b" * 64, (CANDIDATE_LEDGER_ENV,),
                    "P5-PREDECESSOR-01",
                )
        self.assertEqual(ordinary_before, ordinary_after)
        self.assertEqual(ledger_before, settlement_after)
        self.assertNotEqual(ledger_before, claim_after)

    def test_dependency_declaration_is_command_and_role_specific(self) -> None:
        self.assertEqual(
            causal_artifact_dependencies(
                ["cargo", "test", "compile_contract_artifact::exact"], "main-test"
            ),
            (COMPILE_ARTIFACT_ENV,),
        )
        self.assertEqual(
            causal_artifact_dependencies(
                ["cargo", "test", "milestone_3141_phase1_ledger::exact"],
                "main-test",
            ),
            (CANDIDATE_LEDGER_ENV,),
        )
        self.assertEqual(
            causal_artifact_dependencies(
                ["cargo", "test", "predecessor_handoff::exact"], "main-test"
            ),
            (PREDECESSOR_ARTIFACT_ENV,),
        )
        self.assertEqual(
            causal_artifact_dependencies(
                ["cargo", "test", "compile_contract_artifact::exact"],
                "main-discovery",
            ),
            (),
        )
        self.assertEqual(
            causal_artifact_dependencies(
                ["cargo", "test", "producer_slope::exact"], "control-test"
            ),
            (),
        )

    def test_close_ledger_binding_excludes_only_its_own_staged_claim(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            candidate = root / "candidate.csv"
            rows = [
                ledger_row(5, "P5-COLOR-EMOJI-01"),
                ledger_row(5, "P5-CLOSE-01"),
            ]
            write_ledger(candidate, rows)
            dependencies = (CANDIDATE_LEDGER_ENV,)
            with mock.patch.dict(
                os.environ, {CANDIDATE_LEDGER_ENV: str(candidate)}, clear=False
            ):
                before = execution_binding(
                    ["cargo", "test", "milestone_3141_phase1_ledger::closure"],
                    root, "a" * 40, "b" * 64, dependencies, "P5-CLOSE-01",
                )
                rows[1]["exact_command"] = "temporary staged command"
                write_ledger(candidate, rows)
                self_rebound = execution_binding(
                    ["cargo", "test", "milestone_3141_phase1_ledger::closure"],
                    root, "a" * 40, "b" * 64, dependencies, "P5-CLOSE-01",
                )
                rows[0]["exact_command"] = "changed sibling command"
                write_ledger(candidate, rows)
                sibling_changed = execution_binding(
                    ["cargo", "test", "milestone_3141_phase1_ledger::closure"],
                    root, "a" * 40, "b" * 64, dependencies, "P5-CLOSE-01",
                )
        self.assertEqual(before, self_rebound)
        self.assertNotEqual(before, sibling_changed)

    def test_default_compile_artifact_content_is_causally_bound(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            artifact = root / COMPILE_ARTIFACT
            artifact.parent.mkdir(parents=True)
            artifact.write_text("first", encoding="utf-8")
            command = ["cargo", "test", "compile_contract_artifact::exact"]
            with mock.patch.dict(os.environ, {}, clear=True):
                before = execution_binding(
                    command,
                    root,
                    "a" * 40,
                    "b" * 64,
                    causal_artifact_dependencies(command, "main-test"),
                )
                artifact.write_text("second", encoding="utf-8")
                after = execution_binding(
                    command,
                    root,
                    "a" * 40,
                    "b" * 64,
                    causal_artifact_dependencies(command, "main-test"),
                )
        self.assertNotEqual(before, after)

    def test_budget_rejection_evicts_only_the_owning_test_receipt(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            cache = root / "cache"
            with mock.patch.dict(os.environ, {CACHE_ENV: str(cache)}):
                with mock.patch("subprocess.run") as run:
                    run.return_value.returncode = 0
                    run.return_value.stdout = "passed"
                    run.return_value.stderr = ""
                    discovery = timed_execution(
                        ["cargo", "test", "--list"], root, "a" * 40, "b" * 64, "main-discovery"
                    )[2]
                    test = timed_execution(
                        ["cargo", "test", "exact"], root, "a" * 40, "b" * 64, "main-test"
                    )[2]
                receipts = [
                    {"role": "main-discovery", **discovery},
                    {"role": "main-test", **test},
                ]
                invalidate_receipts(receipts, {"main-test"})
            remaining = list(cache.rglob("*.json"))
            self.assertEqual(len(remaining), 1)
            self.assertIn(discovery["key"], remaining[0].name)

    def test_artifact_transport_path_does_not_defeat_content_reuse(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            first = root / "first.json"
            second = root / "second.json"
            first.write_text("same bytes", encoding="utf-8")
            second.write_text("same bytes", encoding="utf-8")
            command = ["cargo", "test", "ordinary::test"]
            with mock.patch.dict(
                os.environ, {"WORTH_UI_COMPILE_ARTIFACT": str(first)}, clear=False
            ):
                before = execution_binding(
                    command, root, "a" * 40, "b" * 64, (COMPILE_ARTIFACT_ENV,)
                )
            with mock.patch.dict(
                os.environ, {"WORTH_UI_COMPILE_ARTIFACT": str(second)}, clear=False
            ):
                after = execution_binding(
                    command, root, "a" * 40, "b" * 64, (COMPILE_ARTIFACT_ENV,)
                )
            second.write_text("changed bytes", encoding="utf-8")
            with mock.patch.dict(
                os.environ, {"WORTH_UI_COMPILE_ARTIFACT": str(second)}, clear=False
            ):
                changed = execution_binding(
                    command, root, "a" * 40, "b" * 64, (COMPILE_ARTIFACT_ENV,)
                )
        self.assertEqual(before, after)
        self.assertNotEqual(before, changed)

    def test_irrelevant_shared_world_does_not_split_rust_control_execution(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            first = root / "world-one.json"
            second = root / "world-two.json"
            first.write_text("one", encoding="utf-8")
            second.write_text("two", encoding="utf-8")
            command = ["cargo", "test", "producer_slope::exact"]
            environment = {CACHE_ENV: str(root / "cache")}
            with mock.patch.dict(os.environ, environment, clear=False):
                with mock.patch("subprocess.run") as run:
                    run.return_value.returncode = 0
                    run.return_value.stdout = "passed"
                    run.return_value.stderr = ""
                    os.environ["WORTH_UI_SHARED_WORLD_ARTIFACT"] = str(first)
                    first_run = timed_execution(
                        command, root, "a" * 40, "b" * 64, "control-test"
                    )
                    os.environ["WORTH_UI_SHARED_WORLD_ARTIFACT"] = str(second)
                    second_run = timed_execution(
                        command, root, "a" * 40, "b" * 64, "control-test"
                    )
            self.assertEqual(run.call_count, 1)
            self.assertEqual(first_run[2]["key"], second_run[2]["key"])
            self.assertTrue(second_run[2]["reused"])


if __name__ == "__main__":
    unittest.main()

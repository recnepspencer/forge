from __future__ import annotations

import csv
import os
import sys
import tempfile
import unittest
from pathlib import Path
from unittest import mock

sys.path.insert(0, str(Path(__file__).resolve().parent))

from worth_ui_ledger_command import CLAIM_FIELDS
from worth_ui_ledger_execution_binding import (
    CANDIDATE_LEDGER_ENV,
    COMPILE_ARTIFACT,
    COMPILE_ARTIFACT_ENV,
    PREDECESSOR_ARTIFACT_ENV,
    GovernedExecutionSnapshot,
    causal_artifact_dependencies,
    execution_binding,
)


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


class ExecutionBindingTests(unittest.TestCase):
    def test_candidate_ledger_binds_only_ledger_consuming_commands(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            candidate = root / "candidate.csv"
            snapshot = GovernedExecutionSnapshot("a" * 40, "b" * 64)
            rows = [
                ledger_row(4, "P4-SHAPING-01"),
                ledger_row(5, "P5-COLOR-EMOJI-01"),
                ledger_row(5, "P5-CLOSE-01"),
            ]
            write_ledger(candidate, rows)
            with mock.patch.dict(os.environ, {CANDIDATE_LEDGER_ENV: str(candidate)}, clear=False):
                ordinary_before = execution_binding(
                    ["cargo", "test", "ordinary::test"], root, snapshot
                )
                ledger_before = execution_binding(
                    ["cargo", "test", "milestone_3141_phase1_ledger::closure"],
                    root,
                    snapshot,
                    (CANDIDATE_LEDGER_ENV,),
                    "P5-PREDECESSOR-01",
                )
                rows[0]["result"] = "PROVED"
                rows[0]["final_source"] = "true"
                rows[1]["owner"] = "phase-five-only-change"
                write_ledger(candidate, rows)
                ordinary_after = execution_binding(
                    ["cargo", "test", "ordinary::test"], root, snapshot
                )
                settlement_after = execution_binding(
                    ["cargo", "test", "milestone_3141_phase1_ledger::closure"],
                    root,
                    snapshot,
                    (CANDIDATE_LEDGER_ENV,),
                    "P5-PREDECESSOR-01",
                )
                rows[0]["owner"] = "phase-four-claim-change"
                write_ledger(candidate, rows)
                claim_after = execution_binding(
                    ["cargo", "test", "milestone_3141_phase1_ledger::closure"],
                    root,
                    snapshot,
                    (CANDIDATE_LEDGER_ENV,),
                    "P5-PREDECESSOR-01",
                )
        self.assertEqual(ordinary_before, ordinary_after)
        self.assertNotEqual(ledger_before, settlement_after)
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
                ["cargo", "test", "milestone_3141_phase1_ledger::exact"], "main-test"
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
                ["cargo", "test", "compile_contract_artifact::exact"], "main-discovery"
            ),
            (),
        )
        self.assertEqual(
            causal_artifact_dependencies(
                ["cargo", "test", "producer_slope::exact"], "control-test"
            ),
            (),
        )

    def test_close_ledger_binding_covers_its_complete_staged_prefix(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            candidate = root / "candidate.csv"
            snapshot = GovernedExecutionSnapshot("a" * 40, "b" * 64)
            rows = [
                ledger_row(5, "P5-COLOR-EMOJI-01"),
                ledger_row(5, "P5-CLOSE-01"),
            ]
            write_ledger(candidate, rows)
            dependencies = (CANDIDATE_LEDGER_ENV,)
            with mock.patch.dict(os.environ, {CANDIDATE_LEDGER_ENV: str(candidate)}, clear=False):
                before = execution_binding(
                    ["cargo", "test", "milestone_3141_phase1_ledger::closure"],
                    root, snapshot, dependencies, "P5-CLOSE-01",
                )
                original_owner = rows[1]["owner"]
                rows[1]["exact_command"] = "temporary staged command"
                write_ledger(candidate, rows)
                self_rebound = execution_binding(
                    ["cargo", "test", "milestone_3141_phase1_ledger::closure"],
                    root, snapshot, dependencies, "P5-CLOSE-01",
                )
                rows[1]["owner"] = "changed self claim owner"
                write_ledger(candidate, rows)
                self_claim_changed = execution_binding(
                    ["cargo", "test", "milestone_3141_phase1_ledger::closure"],
                    root, snapshot, dependencies, "P5-CLOSE-01",
                )
                rows[1]["owner"] = original_owner
                rows[0]["exact_command"] = "changed sibling command"
                write_ledger(candidate, rows)
                sibling_changed = execution_binding(
                    ["cargo", "test", "milestone_3141_phase1_ledger::closure"],
                    root, snapshot, dependencies, "P5-CLOSE-01",
                )
        self.assertEqual(before, self_rebound)
        self.assertNotEqual(before, self_claim_changed)
        self.assertNotEqual(before, sibling_changed)

    def test_default_compile_artifact_content_is_causally_bound(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            artifact = root / COMPILE_ARTIFACT
            artifact.parent.mkdir(parents=True)
            artifact.write_text("first", encoding="utf-8")
            command = ["cargo", "test", "compile_contract_artifact::exact"]
            snapshot = GovernedExecutionSnapshot("a" * 40, "b" * 64)
            with mock.patch.dict(os.environ, {}, clear=True):
                before = execution_binding(
                    command,
                    root,
                    snapshot,
                    causal_artifact_dependencies(command, "main-test"),
                )
                artifact.write_text("second", encoding="utf-8")
                after = execution_binding(
                    command,
                    root,
                    snapshot,
                    causal_artifact_dependencies(command, "main-test"),
                )
        self.assertNotEqual(before, after)

    def test_artifact_transport_path_does_not_defeat_content_reuse(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            first = root / "first.json"
            second = root / "second.json"
            first.write_text("same bytes", encoding="utf-8")
            second.write_text("same bytes", encoding="utf-8")
            command = ["cargo", "test", "ordinary::test"]
            snapshot = GovernedExecutionSnapshot("a" * 40, "b" * 64)
            with mock.patch.dict(os.environ, {COMPILE_ARTIFACT_ENV: str(first)}, clear=False):
                before = execution_binding(command, root, snapshot, (COMPILE_ARTIFACT_ENV,))
            with mock.patch.dict(os.environ, {COMPILE_ARTIFACT_ENV: str(second)}, clear=False):
                after = execution_binding(command, root, snapshot, (COMPILE_ARTIFACT_ENV,))
            second.write_text("changed bytes", encoding="utf-8")
            with mock.patch.dict(os.environ, {COMPILE_ARTIFACT_ENV: str(second)}, clear=False):
                changed = execution_binding(command, root, snapshot, (COMPILE_ARTIFACT_ENV,))
        self.assertEqual(before, after)
        self.assertNotEqual(before, changed)


if __name__ == "__main__":
    unittest.main()

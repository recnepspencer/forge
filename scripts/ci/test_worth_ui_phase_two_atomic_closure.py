from __future__ import annotations

import hashlib
import json
import sys
import tempfile
import unittest
from pathlib import Path


CI = Path(__file__).resolve().parent
if str(CI) not in sys.path:
    sys.path.insert(0, str(CI))

from worth_ui_ledger_atomic_closure import ClosurePreparation
from worth_ui_ledger_phase_two_closure import (
    admit_invalidated_phase_one_prefix,
    phase_two_closure_plan,
)


class PhaseTwoAtomicClosureTests(unittest.TestCase):
    def test_plan_preserves_the_exact_dependency_order(self) -> None:
        rows = [
            row(1, "P1-INDEPENDENT-01", "PROVED", "true"),
            row(1, "P1-WORLDS-01", "PROVED", "true"),
            row(1, "P1-HEADLESS-COST-01", "PROVED", "true"),
            row(1, "P1-CLOSE-01", "PROVED", "true"),
            row(2, "P2-WORLD-01", "OPEN", "false"),
            row(2, "P2-APPLICATION-01", "OPEN", "false"),
            row(2, "P2-CLOSE-01", "OPEN", "false"),
        ]
        plan = phase_two_closure_plan(rows, Path.cwd())
        self.assertEqual(plan.verify_phase, 2)
        self.assertIs(
            plan.preparation, ClosurePreparation.CURRENT_COMPILE_CONTRACTS
        )
        self.assertEqual(
            [candidate["requirement"] for candidate in plan.selected],
            [
                "P1-INDEPENDENT-01",
                "P1-WORLDS-01",
                "P1-HEADLESS-COST-01",
                "P1-CLOSE-01",
                "P2-WORLD-01",
                "P2-APPLICATION-01",
                "P2-CLOSE-01",
            ],
        )

    def test_governed_v2_invalidation_admits_the_complete_open_prefix(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            rows = [
                row(1, "P1-ONE-01", "OPEN", "false"),
                row(1, "P1-CLOSE-01", "OPEN", "false"),
                row(2, "P2-WORLD-01", "OPEN", "false"),
            ]
            bind_receipt(root, rows)
            admitted = admit_invalidated_phase_one_prefix(rows, root)
            self.assertEqual(
                [candidate["requirement"] for candidate in admitted],
                ["P1-ONE-01", "P1-CLOSE-01"],
            )

    def test_unlineaged_open_prefix_is_rejected(self) -> None:
        rows = [
            row(1, "P1-ONE-01", "OPEN", "false"),
            row(1, "P1-CLOSE-01", "OPEN", "false"),
            row(2, "P2-WORLD-01", "OPEN", "false"),
        ]
        with self.assertRaisesRegex(RuntimeError, "lacks governed invalidation"):
            admit_invalidated_phase_one_prefix(rows, Path.cwd())

    def test_forged_receipt_digest_is_rejected(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            rows = [
                row(1, "P1-ONE-01", "OPEN", "false"),
                row(1, "P1-CLOSE-01", "OPEN", "false"),
                row(2, "P2-WORLD-01", "OPEN", "false"),
            ]
            receipt = bind_receipt(root, rows)
            receipt.write_text("{}\n", encoding="utf-8")
            with self.assertRaisesRegex(RuntimeError, "receipt digest"):
                admit_invalidated_phase_one_prefix(rows, root)


def bind_receipt(root: Path, rows: list[dict[str, str]]) -> Path:
    archive_bytes = b'{"schema_version": 6, "exit_posture": "test-failed"}\n'
    observed = hashlib.sha256(archive_bytes).hexdigest()
    archive_identity = f"evidence/superseded/p1-close-01/{observed}.json"
    archive = root / archive_identity
    archive.parent.mkdir(parents=True)
    archive.write_bytes(archive_bytes)
    phase_one = [candidate for candidate in rows if candidate["phase"] == "1"]
    receipt = {
        "schema": "worth-ui-ledger-phase-invalidation-v2",
        "phase": 1,
        "incident": {
            "observed_artifact_sha256": observed,
            "superseded_artifact": archive_identity,
        },
        "invalidated_rows": [
            {
                "requirement": candidate["requirement"],
                "prior_result_artifact_digest": candidate["prior_digest"],
            }
            for candidate in phase_one
        ],
        "causally_reopened_rows": [],
        "preserved_open_requirements": [
            candidate["requirement"]
            for candidate in rows
            if int(candidate["phase"]) > 1
        ],
    }
    content = (json.dumps(receipt, sort_keys=True) + "\n").encode()
    digest = hashlib.sha256(content).hexdigest()
    identity = f"evidence/invalidations/p1/{observed}.json"
    destination = root / identity
    destination.parent.mkdir(parents=True)
    destination.write_bytes(content)
    for candidate in phase_one:
        candidate["reopen_lineage"] = (
            f"invalidation:{identity}@{digest};"
            f"supersedes:{candidate['prior_digest']}"
        )
    return destination


def row(
    phase: int, requirement: str, result: str, final_source: str
) -> dict[str, str]:
    return {
        "phase": str(phase),
        "requirement": requirement,
        "result": result,
        "final_source": final_source,
        "reopen_lineage": "none",
        "prior_digest": hashlib.sha256(requirement.encode()).hexdigest(),
        "exact_command": f"runner --requirement {requirement} --artifact evidence/{requirement}.json",
        "retained_result_artifact": f"evidence/{requirement}.json",
    }


if __name__ == "__main__":
    unittest.main()

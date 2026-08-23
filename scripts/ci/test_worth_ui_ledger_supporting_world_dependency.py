from __future__ import annotations

import csv
import json
import os
import sys
import tempfile
import unittest
from pathlib import Path
from types import SimpleNamespace
from unittest.mock import patch

sys.path.insert(0, str(Path(__file__).resolve().parent))

import worth_ui_3141_supporting_world as supporting_world


class SupportingWorldDependencyTests(unittest.TestCase):
    def test_hp02_entrypoint_rejects_an_open_mixed_world_producer(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            artifact_path = root / supporting_world.MIXED_ARTIFACT
            artifact_path.parent.mkdir(parents=True)
            artifact = {
                "schema_version": 5,
                "requirement": supporting_world.MIXED_REQUIREMENT,
                "package": "worth-ui-certification",
                "target_kind": "test",
                "target_name": "application_contracts",
                "test_name": supporting_world.MIXED_TEST,
                "matched_test_count": 1,
                "declared_ignored_test_count": 1,
                "expected_declared_ignored": True,
                "executed_test_count": 1,
                "passed_test_count": 1,
                "ignored_test_count": 0,
                "exit_posture": "passed",
                "test_exit_code": 0,
                "source_revision": "revision",
                "source_state_digest": "state",
                "structural_counter": "source-rows=1",
                "construction_cost": (
                    "main-tests=1;hostile-controls=1;product-processes=0;"
                    "compile-sessions=0;courtroom-worlds=1"
                ),
                "execution_cost": "executed-tests=2;presentations=5",
                "test_stdout": "WORTH_UI_LEDGER_WORLD=1\n",
            }
            artifact_path.write_text(json.dumps(artifact), encoding="utf-8")
            ledger = root / "candidate.csv"
            with ledger.open("w", encoding="utf-8", newline="") as stream:
                writer = csv.DictWriter(
                    stream,
                    fieldnames=["requirement", "result", "final_source"],
                )
                writer.writeheader()
                writer.writerow({
                    "requirement": supporting_world.MIXED_REQUIREMENT,
                    "result": "OPEN",
                    "final_source": "false",
                })
            test = SimpleNamespace(
                requirement="P3-HP02-WORLD-01",
                sources=(supporting_world.MIXED_ARTIFACT,),
            )
            with patch.dict(
                os.environ, {"WORTH_UI_MILESTONE_3141_LEDGER": str(ledger)}
            ):
                with self.assertRaisesRegex(ValueError, "not final-source proved"):
                    supporting_world.validate_phase3_hp02_support(
                        test, "revision", "state", root
                    )


if __name__ == "__main__":
    unittest.main()

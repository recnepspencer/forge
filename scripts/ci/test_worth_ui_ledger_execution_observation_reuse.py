from __future__ import annotations

import json
import os
import sys
import tempfile
import unittest
from pathlib import Path
from unittest import mock

sys.path.insert(0, str(Path(__file__).resolve().parent))

from worth_ui_ledger_execution_observation_store import (
    CACHE_ENV,
    invalidate_references,
)
from worth_ui_ledger_execution_runner import timed_execution


class ExecutionObservationReuseTests(unittest.TestCase):
    def test_exact_command_executes_once_per_source_state(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            with mock.patch.dict(os.environ, {CACHE_ENV: str(root / "cache")}):
                with mock.patch("subprocess.run") as run:
                    run.return_value.returncode = 0
                    run.return_value.stdout = "passed"
                    run.return_value.stderr = ""
                    first = timed_execution(
                        ["cargo", "test", "exact"], root, "a" * 40, "b" * 64, "main"
                    )
                    second = timed_execution(
                        ["cargo", "test", "exact"], root, "a" * 40, "b" * 64, "control"
                    )
                self.assertEqual(run.call_count, 1)
                self.assertEqual(first[2]["acquisition"], "executed")
                self.assertEqual(second[2]["acquisition"], "reused")

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
                    receipt = next((cache / "execution-observations").rglob("*.json"))
                    payload = json.loads(receipt.read_text(encoding="utf-8"))
                    payload["record"]["stdout"] = "forged"
                    receipt.write_text(json.dumps(payload), encoding="utf-8")
                    timed_execution(["cargo", "test", "exact"], root, "a" * 40, "b" * 64, "main")
                self.assertEqual(run.call_count, 2)

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
                invalidate_references(receipts, {"main-test"})
            remaining = list((cache / "execution-bindings").rglob("*.json"))
            self.assertEqual(len(remaining), 1)
            self.assertIn(discovery["execution_binding_key"], remaining[0].name)

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
            self.assertEqual(
                first_run[2]["execution_binding_key"],
                second_run[2]["execution_binding_key"],
            )
            self.assertEqual(second_run[2]["acquisition"], "reused")


if __name__ == "__main__":
    unittest.main()

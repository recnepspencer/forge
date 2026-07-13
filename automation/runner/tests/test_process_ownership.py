import json
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

from runner.authority.run_identity.process_ownership import (
    active_provider_pids,
    active_runner_pid,
    stop_owned_processes,
)


class FakePaths:
    def __init__(self, root: Path) -> None:
        self.executions = root / "executions"
        self.active_lock = root / "locks" / "run.active.lock"


class ProcessOwnershipTests(unittest.TestCase):
    def setUp(self) -> None:
        self.temporary = tempfile.TemporaryDirectory()
        self.paths = FakePaths(Path(self.temporary.name))

    def tearDown(self) -> None:
        self.temporary.cleanup()

    def test_active_processes_come_only_from_owned_runtime_records(self) -> None:
        self.paths.executions.mkdir(parents=True)
        self.paths.active_lock.parent.mkdir(parents=True)
        self.paths.active_lock.write_text(json.dumps({"pid": 101}), encoding="utf-8")
        (self.paths.executions / "active.json").write_text(
            json.dumps({"state": "launched", "runner_pid": 101, "provider_pid": 202}), encoding="utf-8"
        )
        (self.paths.executions / "done.json").write_text(
            json.dumps({"state": "finished", "runner_pid": 101, "provider_pid": 303}), encoding="utf-8"
        )

        self.assertEqual(active_runner_pid(self.paths), 101)
        self.assertEqual(active_provider_pids(self.paths, 101), [202])
        self.assertEqual(active_provider_pids(self.paths, 999), [])

    def test_stop_terminates_provider_before_runner_and_verifies_exit(self) -> None:
        with (
            patch(
                "runner.authority.run_identity.process_ownership.active_provider_pids",
                return_value=[202],
            ),
            patch(
                "runner.authority.run_identity.process_ownership.active_runner_pid",
                return_value=101,
            ),
            patch("runner.authority.run_identity.process_ownership.terminate_process_tree") as terminate,
            patch("runner.authority.run_identity.process_ownership.wait_for_exit"),
            patch(
                "runner.authority.run_identity.process_ownership.pid_is_running",
                side_effect=[True, False, False],
            ),
        ):
            stop_owned_processes(self.paths)

        self.assertEqual([call.args[0] for call in terminate.call_args_list], [202, 101])


if __name__ == "__main__":
    unittest.main()

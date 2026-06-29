from __future__ import annotations

import sys
import unittest
from unittest.mock import patch
from pathlib import Path

RUNNER_DIR = Path(__file__).resolve().parents[1]
if str(RUNNER_DIR) not in sys.path:
    sys.path.insert(0, str(RUNNER_DIR))

from codex_cli import CodexResult
from runner_faults import FailureKind, RecoveryKind, RunnerFault
from runner_recovery import run_recovery

from support import load_json, make_temp_state_copy


class RunnerFailureModeTests(unittest.TestCase):
    def test_codex_exit_failure_requests_codex_recovery(self) -> None:
        path = make_temp_state_copy()

        with patch("runner_recovery.run_codex", return_value=CodexResult(exit_code=0)):
            outcome = run_recovery(
                path,
                None,
                RunnerFault(
                    kind=FailureKind.CODEX_EXIT,
                    reason="RunnerFailure: codex exited with 1",
                    details="traceback",
                ),
                lambda *_: "recovery prompt",
            )

        state = load_json(path)
        events = [entry["event"] for entry in state["history"][-3:]]
        self.assertTrue(outcome.should_continue)
        self.assertEqual(outcome.decision.kind, RecoveryKind.CODEX_RECOVERY)
        self.assertIn("runner_recovery_requested", events)
        self.assertIn("runner_recovery_succeeded", events)

    def test_failed_codex_recovery_stops_terminally(self) -> None:
        path = make_temp_state_copy()

        with patch("runner_recovery.run_codex", return_value=CodexResult(exit_code=1)):
            outcome = run_recovery(
                path,
                None,
                RunnerFault(
                    kind=FailureKind.CODEX_EXIT,
                    reason="RunnerFailure: codex exited with 1",
                    details="traceback",
                ),
                lambda *_: "recovery prompt",
            )

        state = load_json(path)
        self.assertFalse(outcome.should_continue)
        self.assertEqual(outcome.decision.kind, RecoveryKind.CODEX_RECOVERY)
        self.assertEqual(state["history"][-1]["event"], "runner_terminal_stop")


if __name__ == "__main__":
    unittest.main()

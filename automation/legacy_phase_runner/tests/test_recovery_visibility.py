from __future__ import annotations

import json
import os
import sys
import tempfile
import unittest
from pathlib import Path


RUNNER_DIR = Path(__file__).resolve().parents[1]
if str(RUNNER_DIR) not in sys.path:
    sys.path.insert(0, str(RUNNER_DIR))

from operator_commands import status_view
from orchestrator import pending_recovery_reason
from runtime_paths import RuntimePaths, active_run_liveness
import runtime_paths


class RecoveryVisibilityTests(unittest.TestCase):
    def test_unclosed_selected_turn_routes_to_recovery(self) -> None:
        events = [
            {
                "event_type": "prompt_selected",
                "phase_id": 6,
                "turn": "review",
                "payload": {"turn_instance_id": "review-1"},
            }
        ]
        self.assertEqual(
            pending_recovery_reason(events, {"phase": 6, "turn": "review"}, "review-1"),
            "prior agent turn was selected but no terminal event was recorded",
        )

    def test_status_marks_dead_driver_as_recovery_required(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            original_root = runtime_paths.RUNTIME_ROOT
            runtime_paths.RUNTIME_ROOT = Path(temp_dir) / "runtime"
            try:
                paths = RuntimePaths("dead-driver")
                paths.locks_dir.mkdir(parents=True)
                paths.active_lock.write_text(json.dumps({"pid": os.getpid() + 9_999_999}), encoding="utf-8")
                liveness = active_run_liveness(paths)
            finally:
                runtime_paths.RUNTIME_ROOT = original_root
        self.assertEqual(liveness, "not_running")
        status = status_view(
            {"run_id": "dead-driver", "current": {"phase": 6, "turn": "review"}, "completed_at": None,
             "stopped": False, "stop_reason": None, "session": {"thread_id": None},
             "latest_summary": None, "last_event": None, "phases": []},
            liveness,
        )
        self.assertTrue(status["recovery_required"])


if __name__ == "__main__":
    unittest.main()

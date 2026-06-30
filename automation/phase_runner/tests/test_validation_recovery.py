from __future__ import annotations

import json
import sys
import unittest
from pathlib import Path

RUNNER_DIR = Path(__file__).resolve().parents[1]
if str(RUNNER_DIR) not in sys.path:
    sys.path.insert(0, str(RUNNER_DIR))

from runner_runtime import validate_command
from validation import validate_state

from support import load_json, make_temp_state_copy


class ValidationTests(unittest.TestCase):
    def test_validate_command_rejects_non_boolean_fast_mode(self) -> None:
        path = make_temp_state_copy()
        state = load_json(path)
        state["session"]["fast_mode"] = "yes"
        path.write_text(json.dumps(state, indent=2) + "\n", encoding="utf-8")

        exit_code = validate_command(path)

        self.assertEqual(exit_code, 2)

    def test_validate_command_persists_normalized_note_buckets(self) -> None:
        path = make_temp_state_copy()
        state = load_json(path)
        state["phases"][0]["notes"]["remaining"] = "bad-string"
        state["phases"][0]["notes"]["findings"] = {"oops": True}
        path.write_text(json.dumps(state, indent=2) + "\n", encoding="utf-8")

        exit_code = validate_command(path)
        repaired = load_json(path)

        self.assertEqual(exit_code, 0)
        self.assertEqual(repaired["phases"][0]["notes"]["remaining"], ["bad-string"])
        self.assertEqual(repaired["phases"][0]["notes"]["findings"], [{"oops": True}])
        self.assertEqual(repaired["history"][-1]["event"], "runner_state_normalized")

    def test_validate_state_rejects_complete_passed_phase_on_current_cursor(self) -> None:
        path = make_temp_state_copy()
        state = load_json(path)
        state["current"] = {"phase": 3, "turn": "test_repair_implement"}
        state["phases"][2]["status"] = "complete"
        state["phases"][2]["qa_status"] = "passed"

        errors = validate_state(state, path)

        self.assertTrue(
            any(
                "complete/passed phase cannot keep an active current cursor" in error
                for error in errors
            )
        )


if __name__ == "__main__":
    unittest.main()

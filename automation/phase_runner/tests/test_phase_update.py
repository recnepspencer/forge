from __future__ import annotations

import sys
import unittest
from pathlib import Path

RUNNER_DIR = Path(__file__).resolve().parents[1]
if str(RUNNER_DIR) not in sys.path:
    sys.path.insert(0, str(RUNNER_DIR))

from phase_update import PhaseUpdateError, apply_phase_update, parse_phase_update

from support import load_json, make_temp_state_copy


class PhaseUpdateTests(unittest.TestCase):
    def test_apply_phase_update_advances_review_to_repair(self) -> None:
        path = make_temp_state_copy()
        state = load_json(path)
        state["current"] = {"phase": 1, "turn": "review"}

        update = parse_phase_update(
            {
                "phase": 1,
                "completed_turn": "review",
                "status": "regressed",
                "qa_status": "failed",
                "next_turn": "repair",
                "detail": "phase 1 review failed",
                "notes": {"findings": ["real gap"]},
            }
        )

        apply_phase_update(state, update)

        self.assertEqual(state["current"], {"phase": 1, "turn": "repair"})
        self.assertEqual(state["phases"][0]["status"], "regressed")
        self.assertEqual(state["phases"][0]["qa_status"], "failed")
        self.assertEqual(state["history"][-1]["event"], "codex_turn_completed")

    def test_apply_phase_update_rejects_wrong_cursor(self) -> None:
        path = make_temp_state_copy()
        state = load_json(path)
        state["current"] = {"phase": 1, "turn": "review"}
        update = parse_phase_update(
            {
                "phase": 1,
                "completed_turn": "repair",
                "status": "complete",
                "qa_status": "needed",
                "next_turn": "review",
                "detail": "wrong turn",
            }
        )

        with self.assertRaises(PhaseUpdateError):
            apply_phase_update(state, update)

    def test_apply_phase_update_advances_code_quality_review_to_next_phase(self) -> None:
        path = make_temp_state_copy()
        state = load_json(path)
        state["current"] = {"phase": 1, "turn": "code_quality_review"}
        update = parse_phase_update(
            {
                "phase": 1,
                "completed_turn": "code_quality_review",
                "status": "complete",
                "qa_status": "passed",
                "next_turn": None,
                "detail": "phase 1 closed",
                "notes": {"done": ["closed"]},
            }
        )

        apply_phase_update(state, update)

        self.assertEqual(state["current"], {"phase": 2, "turn": "plan"})
        self.assertEqual(state["phases"][0]["qa_status"], "passed")


if __name__ == "__main__":
    unittest.main()

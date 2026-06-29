from __future__ import annotations

import json
import sys
import unittest
from pathlib import Path

RUNNER_DIR = Path(__file__).resolve().parents[1]
if str(RUNNER_DIR) not in sys.path:
    sys.path.insert(0, str(RUNNER_DIR))

from runner_faults import FailureKind, RecoveryKind, RunnerFault
from runner_recovery import choose_recovery
from runner_runtime import validate_command

from support import load_json, make_temp_state_copy


class ValidationRecoveryTests(unittest.TestCase):
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

    def test_choose_recovery_uses_local_normalization_for_repairable_validation_failure(
        self,
    ) -> None:
        path = make_temp_state_copy()
        state = load_json(path)
        state["phases"][0]["notes"]["remaining"] = "bad-string"
        state["phases"][0]["notes"]["findings"] = {"oops": True}
        path.write_text(json.dumps(state, indent=2) + "\n", encoding="utf-8")

        decision = choose_recovery(
            path,
            RunnerFault(
                kind=FailureKind.STATE_VALIDATION,
                reason="validation failure",
                details="validation error: phases[0].notes.remaining must be a list",
            ),
        )

        self.assertEqual(decision.kind, RecoveryKind.LOCAL_NORMALIZE)

    def test_choose_recovery_uses_codex_recovery_for_prompt_failure(self) -> None:
        path = make_temp_state_copy()

        decision = choose_recovery(
            path,
            RunnerFault(
                kind=FailureKind.PROMPT_RENDER,
                reason="missing prompt token",
                details="KeyError: token missing",
            ),
        )

        self.assertEqual(decision.kind, RecoveryKind.CODEX_RECOVERY)


if __name__ == "__main__":
    unittest.main()

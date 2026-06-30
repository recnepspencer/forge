from __future__ import annotations

import sys
import unittest
from pathlib import Path

RUNNER_DIR = Path(__file__).resolve().parents[1]
if str(RUNNER_DIR) not in sys.path:
    sys.path.insert(0, str(RUNNER_DIR))

from state_normalization import normalize_state

from support import load_json, make_temp_state_copy


class StateNormalizationTests(unittest.TestCase):
    def test_normalize_state_repairs_note_buckets(self) -> None:
        path = make_temp_state_copy()
        state = load_json(path)
        state["phases"][0]["notes"]["remaining"] = "bad-string"
        state["phases"][0]["notes"]["findings"] = {"oops": True}

        changed = normalize_state(state)

        self.assertTrue(changed)
        self.assertEqual(state["phases"][0]["notes"]["remaining"], ["bad-string"])
        self.assertEqual(state["phases"][0]["notes"]["findings"], [{"oops": True}])

    def test_normalize_state_is_idempotent(self) -> None:
        path = make_temp_state_copy()
        state = load_json(path)

        first = normalize_state(state)
        second = normalize_state(state)

        self.assertFalse(first)
        self.assertFalse(second)


if __name__ == "__main__":
    unittest.main()

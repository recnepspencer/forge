from __future__ import annotations

import sys
import unittest
from pathlib import Path

RUNNER_DIR = Path(__file__).resolve().parents[1]
if str(RUNNER_DIR) not in sys.path:
    sys.path.insert(0, str(RUNNER_DIR))

from codex_cli import build_command, session_config

from support import load_json, make_temp_state_copy


class CodexCliTests(unittest.TestCase):
    def test_session_fast_mode_populates_official_codex_keys(self) -> None:
        path = make_temp_state_copy()
        state = load_json(path)
        session = state["session"]
        session["model"] = "gpt-5.4"
        session["reasoning_effort"] = "medium"
        session["service_tier"] = "fast"
        session["fast_mode"] = True

        config = session_config(session)

        self.assertEqual(config["service_tier"], "fast")
        self.assertTrue(config["features.fast_mode"])

    def test_build_command_emits_fast_mode_config_args(self) -> None:
        path = make_temp_state_copy()
        state = load_json(path)
        session = state["session"]
        session["model"] = "gpt-5.4"
        session["reasoning_effort"] = "medium"
        session["service_tier"] = "fast"
        session["fast_mode"] = True
        session.pop("thread_id", None)

        command = build_command(state)
        rendered = " ".join(command)

        self.assertIn('-c service_tier="fast"', rendered)
        self.assertIn("-c features.fast_mode=true", rendered)


if __name__ == "__main__":
    unittest.main()

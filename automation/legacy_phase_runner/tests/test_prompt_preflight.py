from __future__ import annotations

import copy
import sys
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch


RUNNER_DIR = Path(__file__).resolve().parents[1]
if str(RUNNER_DIR) not in sys.path:
    sys.path.insert(0, str(RUNNER_DIR))

from config_schema import load_config, validate_config
from event_log import load_events
from orchestrator import append_runtime_event, run_single_turn
from runtime_paths import RuntimePaths
import runtime_paths


class PromptPreflightTests(unittest.TestCase):
    def test_active_runner_config_accepts_legacy_state_file_token(self) -> None:
        config_path = RUNNER_DIR / "config" / "automation-runner-milestone-1.json"
        self.assertEqual(validate_config(load_config(config_path), config_path), [])

    def test_unknown_template_token_is_rejected_before_provider_execution(self) -> None:
        config_path = RUNNER_DIR / "config" / "automation-runner-milestone-1.json"
        config = copy.deepcopy(load_config(config_path))
        with tempfile.TemporaryDirectory() as temp_dir:
            bad_template = Path(temp_dir) / "bad.md"
            bad_template.write_text("Unknown: {not_a_runner_token}\n", encoding="utf-8")
            config["turn_templates"]["review"] = str(bad_template)
            errors = validate_config(config, config_path)
        self.assertIn(
            "turn_templates.review uses unsupported template variable {not_a_runner_token}",
            errors,
        )

    def test_render_failure_records_fault_and_enters_recovery(self) -> None:
        config_path = RUNNER_DIR / "config" / "automation-runner-milestone-1.json"
        with tempfile.TemporaryDirectory() as temp_dir:
            original_root = runtime_paths.RUNTIME_ROOT
            runtime_paths.RUNTIME_ROOT = Path(temp_dir) / "runtime"
            try:
                paths = RuntimePaths("render-fault")
                append_runtime_event(
                    paths,
                    "run_started",
                    payload={"config_path": str(config_path.resolve())},
                )
                with patch("orchestrator.render_prompt", side_effect=ValueError("broken template")), patch(
                    "orchestrator.run_recovery_turn", return_value=0
                ) as recovery:
                    self.assertEqual(run_single_turn(config_path, "render-fault", None), 0)
                recovery.assert_called_once()
                self.assertIn("prompt preparation failed: broken template", recovery.call_args.args[3])
                faults = [event for event in load_events(paths.events) if event["event_type"] == "runner_fault"]
                self.assertEqual(len(faults), 1)
                self.assertIn("prompt preparation failed: broken template", faults[0]["payload"]["reason"])
            finally:
                runtime_paths.RUNTIME_ROOT = original_root


if __name__ == "__main__":
    unittest.main()

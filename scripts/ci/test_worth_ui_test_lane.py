import json
import os
from tempfile import TemporaryDirectory
from unittest import TestCase, main
from unittest.mock import patch

import run_worth_ui_test_lane as lane_runner


class WorthUiTestLaneTests(TestCase):
    def test_fast_lane_covers_all_feature_library_and_facade_proof(self) -> None:
        commands = lane_runner.commands_for("fast")

        self.assertEqual(len(commands), 2)
        for command in commands:
            self.assertIn("--all-features", command)
        self.assertIn("--lib", commands[0])
        self.assertIn("capability_contracts", commands[1])
        self.assertIn("registry_contracts", commands[1])

    def test_documentation_and_platform_lanes_have_exact_scope(self) -> None:
        documentation = lane_runner.commands_for("documentation")[0]
        platform = lane_runner.commands_for("platform-check")[0]

        self.assertIn("--doc", documentation)
        self.assertIn("--all-features", documentation)
        self.assertIn("--all-targets", platform)
        self.assertIn("--all-features", platform)

    def test_full_lane_retains_workspace_and_dependency_contracts(self) -> None:
        commands = lane_runner.commands_for("full")

        self.assertEqual(len(commands), 2)
        self.assertIn("--workspace", commands[0])
        self.assertIn("--all-features", commands[0])
        self.assertIn("host_contract_only_adapter", " ".join(commands[1]))

    def test_report_is_machine_readable_and_preserves_failed_command(self) -> None:
        outcomes = [
            {
                "argv": ["cargo", "test"],
                "command": "cargo test",
                "duration_seconds": 1.25,
                "exit_code": 1,
                "error": None,
            }
        ]
        with TemporaryDirectory() as temporary:
            with (
                patch.dict(os.environ, {"WORTH_UI_LANE_REPORT_DIR": temporary}),
                patch.object(lane_runner, "compiler_cache_stats", return_value=None),
            ):
                lane_runner.write_report("fast", outcomes)

            payload = json.loads(
                (lane_runner.Path(temporary) / "fast.json").read_text(encoding="utf-8")
            )

        self.assertFalse(payload["success"])
        self.assertEqual(payload["commands"][0]["exit_code"], 1)
        self.assertEqual(payload["total_duration_seconds"], 1.25)


if __name__ == "__main__":
    main()

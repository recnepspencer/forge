from __future__ import annotations

import sys
import unittest
from pathlib import Path
from unittest.mock import patch

RUNNER_DIR = Path(__file__).resolve().parents[1]
if str(RUNNER_DIR) not in sys.path:
    sys.path.insert(0, str(RUNNER_DIR))

from completion_handoff import resume_completion_handoff_target
from config_schema import validate_config


class CompletionHandoffTests(unittest.TestCase):
    def test_config_accepts_completion_handoff(self) -> None:
        config = minimal_config_with_completion_handoff()
        self.assertEqual(validate_config(config, Path(config["_config_path"])), [])

    def test_completion_handoff_resumes_stopped_target_with_reason(self) -> None:
        with patch("completion_handoff.RuntimePaths") as runtime_paths, patch(
            "completion_handoff.config_path_for_run", return_value=Path("C:/tmp/target.json")
        ), patch(
            "completion_handoff.refresh_projection",
            return_value={"completed_at": None, "stopped": True},
        ), patch(
            "completion_handoff.resume_run_with_reason", return_value=0
        ) as resume_run_with_reason:
            runtime_paths.return_value.events.exists.return_value = True

            result = resume_completion_handoff_target(
                {
                    "next_run_id": "worth76",
                    "loop": True,
                    "sleep_seconds": 45,
                    "reason": "side-quest complete; resume milestone 7.6",
                },
                polling_run_id="worth76hotcold",
            )

        self.assertEqual(result, 0)
        resume_run_with_reason.assert_called_once_with(
            "worth76",
            True,
            45,
            None,
            "side-quest complete; resume milestone 7.6",
        )

    def test_completion_handoff_skips_active_target(self) -> None:
        with patch("completion_handoff.RuntimePaths") as runtime_paths, patch(
            "completion_handoff.config_path_for_run", return_value=Path("C:/tmp/target.json")
        ), patch(
            "completion_handoff.refresh_projection",
            return_value={"completed_at": None, "stopped": False},
        ), patch("completion_handoff.resume_run_with_reason") as resume_run_with_reason:
            runtime_paths.return_value.events.exists.return_value = True

            result = resume_completion_handoff_target({"next_run_id": "worth76"})

        self.assertEqual(result, 0)
        resume_run_with_reason.assert_not_called()

    def test_completion_handoff_uses_default_reason(self) -> None:
        with patch("completion_handoff.RuntimePaths") as runtime_paths, patch(
            "completion_handoff.config_path_for_run", return_value=Path("C:/tmp/target.json")
        ), patch(
            "completion_handoff.refresh_projection",
            return_value={"completed_at": None, "stopped": True},
        ), patch(
            "completion_handoff.resume_run_with_reason", return_value=0
        ) as resume_run_with_reason:
            runtime_paths.return_value.events.exists.return_value = True

            resume_completion_handoff_target(
                {"next_run_id": "worth76"},
                polling_run_id="worth76hotcold",
            )

        resume_run_with_reason.assert_called_once_with(
            "worth76",
            True,
            30,
            None,
            "completion handoff from worth76hotcold",
        )


def minimal_config_with_completion_handoff() -> dict:
    return {
        "_config_path": str((RUNNER_DIR / "config" / "completion-handoff-test.json").resolve()),
        "schema_version": 1,
        "project": {
            "name": "handoff test",
            "cwd": str(RUNNER_DIR.parents[1]),
            "spec_file": "automation/phase_runner/README.md",
            "context_files": ["automation/phase_runner/README.md"],
        },
        "turn_templates": {
            "plan": str((RUNNER_DIR / "templates" / "plan.md").resolve()),
            "implement": str((RUNNER_DIR / "templates" / "implement.md").resolve()),
            "review": str((RUNNER_DIR / "templates" / "review_test_hardening.md").resolve()),
            "repair_plan": str((RUNNER_DIR / "templates" / "plan.md").resolve()),
            "repair": str((RUNNER_DIR / "templates" / "repair.md").resolve()),
            "test_review": str((RUNNER_DIR / "templates" / "test_review.md").resolve()),
            "test_repair_plan": str((RUNNER_DIR / "templates" / "test_repair_plan.md").resolve()),
            "test_repair_implement": str((RUNNER_DIR / "templates" / "test_repair_implement.md").resolve()),
            "code_quality_review": str((RUNNER_DIR / "templates" / "code_quality_review.md").resolve()),
            "code_quality_repair": str((RUNNER_DIR / "templates" / "code_quality_repair.md").resolve()),
        },
        "contract_template": str((RUNNER_DIR / "templates" / "_contract_test_hardening.md").resolve()),
        "session_defaults": {
            "command": "codex",
            "model": "gpt-5.4",
            "reasoning_effort": "medium",
            "config": {"approval_policy": "never", "sandbox_mode": "danger-full-access"},
        },
        "runner_control": {
            "completion_handoff": {
                "next_run_id": "worth76",
                "loop": True,
                "sleep_seconds": 30,
                "reason": "side-quest complete; resume milestone 7.6",
            }
        },
        "phases": [
            {
                "id": 1,
                "title": "Phase 1",
                "owner": "owner",
                "scope": ["scope/path"],
                "acceptance": ["acceptance"],
                "instructions": "do the thing",
                "qa_focus": "no nonsense",
            }
        ],
    }


if __name__ == "__main__":
    unittest.main()

from __future__ import annotations

import sys
import unittest
from pathlib import Path

RUNNER_DIR = Path(__file__).resolve().parents[1]
if str(RUNNER_DIR) not in sys.path:
    sys.path.insert(0, str(RUNNER_DIR))

from config_schema import validate_config
from projector import project_run
from prompts import render_prompt


class PhaseExecutionModeTests(unittest.TestCase):
    def test_single_prompt_phase_uses_phase_contract_override(self) -> None:
        config = single_prompt_only_config()
        projection = project_run(config, [event("run_started", 1)], "singleprompt")

        rendered = render_prompt(
            config,
            projection,
            Path(config["_config_path"]),
            Path(config["_config_path"]).with_suffix(".projection.json"),
            Path(config["_config_path"]).with_suffix(".events.jsonl"),
            expected_turn_instance_id="turn-1",
        )

        self.assertIn("This phase runs one explicit prompt and closes on one declared success event.", rendered)
        self.assertIn('"turn_instance_id":"turn-1"', rendered)
        self.assertNotIn("Only `code_quality_review` advances to the next phase", rendered)

    def test_single_prompt_phase_starts_on_single_prompt_turn(self) -> None:
        config = single_prompt_only_config()
        projection = project_run(config, [event("run_started", 1)], "singleprompt")
        self.assertEqual(projection["current"], {"phase": 1, "turn": "single_prompt"})

    def test_single_prompt_phase_closes_after_declared_success_event(self) -> None:
        config = mixed_mode_config()
        events = [
            event("run_started", 1),
            event("plan_posted", 2, 1, "plan"),
            event("implementation_completed", 3, 1, "implement"),
            event("review_passed", 4, 1, "review"),
            event("test_review_passed", 5, 1, "test_review"),
            event("code_quality_review_passed", 6, 1, "code_quality_review"),
            event("single_prompt_completed", 7, 2, "single_prompt"),
        ]

        projection = project_run(config, events, "mixedmode")

        self.assertIsNone(projection["current"])
        self.assertEqual(projection["phases"][1]["status"], "complete")
        self.assertEqual(projection["phases"][1]["qa_status"], "passed")

    def test_single_prompt_phase_follows_standard_phase_with_single_prompt_cursor(self) -> None:
        config = mixed_mode_config()
        events = [
            event("run_started", 1),
            event("plan_posted", 2, 1, "plan"),
            event("implementation_completed", 3, 1, "implement"),
            event("review_passed", 4, 1, "review"),
            event("test_review_passed", 5, 1, "test_review"),
            event("code_quality_review_passed", 6, 1, "code_quality_review"),
        ]

        projection = project_run(config, events, "mixedmode")

        self.assertEqual(projection["current"], {"phase": 2, "turn": "single_prompt"})

    def test_validate_config_rejects_non_closeout_single_prompt_event(self) -> None:
        config = single_prompt_only_config(success_event_type="plan_posted")
        errors = validate_config(config, Path(config["_config_path"]))
        self.assertTrue(
            any("success_event_type must be one of ['single_prompt_completed']" in error for error in errors)
        )


def single_prompt_only_config(success_event_type: str = "single_prompt_completed") -> dict:
    return {
        "_config_path": str((RUNNER_DIR / "config" / "phase-execution-mode-test.json").resolve()),
        "schema_version": 1,
        "project": {
            "name": "single prompt test",
            "cwd": str(RUNNER_DIR.parents[1]),
            "spec_file": "automation/phase_runner/README.md",
            "context_files": ["automation/phase_runner/README.md"],
        },
        "turn_templates": standard_turn_templates(),
        "contract_template": str((RUNNER_DIR / "templates" / "_contract_test_hardening.md").resolve()),
        "session_defaults": {
            "command": "codex",
            "model": "gpt-5.4",
            "reasoning_effort": "medium",
            "config": {"approval_policy": "never", "sandbox_mode": "danger-full-access"},
        },
        "phases": [
            {
                "id": 1,
                "title": "Single prompt phase",
                "owner": "owner",
                "scope": ["automation/phase_runner/README.md"],
                "acceptance": ["done in one pass"],
                "instructions": "Update the doc once.",
                "qa_focus": "Do not reopen the standard loop.",
                "execution_mode": "single_prompt",
                "prompt_template": str((RUNNER_DIR / "templates" / "single_prompt.md").resolve()),
                "contract_template": str(
                    (RUNNER_DIR / "templates" / "_contract_single_prompt_worth_geometry.md").resolve()
                ),
                "success_event_type": success_event_type,
            }
        ],
    }


def mixed_mode_config() -> dict:
    config = single_prompt_only_config()
    config["phases"] = [
        phase_row(1, "Standard phase"),
        config["phases"][0],
    ]
    config["phases"][1]["id"] = 2
    config["phases"][1]["title"] = "Single prompt closeout"
    return config


def phase_row(phase_id: int, title: str) -> dict:
    return {
        "id": phase_id,
        "title": title,
        "owner": "owner",
        "scope": ["scope/path"],
        "acceptance": ["acceptance"],
        "instructions": "do the thing",
        "qa_focus": "no nonsense",
    }


def standard_turn_templates() -> dict[str, str]:
    template_dir = RUNNER_DIR / "templates"
    return {
        "plan": str((template_dir / "plan.md").resolve()),
        "implement": str((template_dir / "implement.md").resolve()),
        "review": str((template_dir / "review_test_hardening.md").resolve()),
        "repair_plan": str((template_dir / "plan.md").resolve()),
        "repair": str((template_dir / "repair.md").resolve()),
        "test_review": str((template_dir / "test_review.md").resolve()),
        "test_repair_plan": str((template_dir / "test_repair_plan.md").resolve()),
        "test_repair_implement": str((template_dir / "test_repair_implement.md").resolve()),
        "code_quality_review": str((template_dir / "code_quality_review.md").resolve()),
    }


def event(
    event_type: str,
    sequence: int,
    phase_id: int | None = None,
    turn: str | None = None,
    payload: dict | None = None,
) -> dict:
    return {
        "run_id": "run123",
        "sequence": sequence,
        "at": "2026-07-01T00:00:00+00:00",
        "event_type": event_type,
        "phase_id": phase_id,
        "turn": turn,
        "thread_id": None,
        "payload": payload or {},
    }


if __name__ == "__main__":
    unittest.main()

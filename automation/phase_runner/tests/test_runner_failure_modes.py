from __future__ import annotations

import json
import sys
import tempfile
import unittest
from pathlib import Path

RUNNER_DIR = Path(__file__).resolve().parents[1]
if str(RUNNER_DIR) not in sys.path:
    sys.path.insert(0, str(RUNNER_DIR))

import runtime_paths
from config_schema import load_config, validate_config
from event_log import validate_event_log
from orchestrator import extract_runner_event, import_legacy_run, refresh_projection
from projector import project_run


class DurableRunnerTests(unittest.TestCase):
    def test_real_config_validates(self) -> None:
        config_path = RUNNER_DIR / "config" / "worth-ui-milestone-3.3.json"
        config = load_config(config_path)
        self.assertEqual(validate_config(config, config_path), [])

    def test_projection_is_deterministic_and_cursor_is_derived(self) -> None:
        config = minimal_config()
        events = [
            event("run_started", 1),
            event("plan_posted", 2, 1, "plan"),
            event("implementation_completed", 3, 1, "implement"),
            event("review_passed", 4, 1, "review"),
            event("test_review_passed", 5, 1, "test_review"),
            event("code_quality_review_passed", 6, 1, "code_quality_review"),
        ]
        first = project_run(config, events, "run123")
        second = project_run(config, events, "run123")
        self.assertEqual(first, second)
        self.assertEqual(first["current"], {"phase": 2, "turn": "plan"})

    def test_validate_event_log_rejects_unknown_event(self) -> None:
        events = [event("made_up_event", 1)]
        errors = validate_event_log(events, "run123")
        self.assertTrue(any("unknown event_type" in error for error in errors))

    def test_projection_rejects_illegal_turn_sequence(self) -> None:
        config = minimal_config()
        events = [
            event("run_started", 1),
            event("plan_posted", 2, 1, "plan"),
            event("code_quality_review_passed", 3, 1, "code_quality_review"),
        ]
        with self.assertRaisesRegex(ValueError, "targets turn"):
            project_run(config, events, "run123")

    def test_extract_runner_event_reads_marker(self) -> None:
        parsed = extract_runner_event(
            [
                "work happened\nRUNNER_EVENT: {\"event_type\":\"review_failed\",\"payload\":{\"notes\":{\"findings\":[\"gap\"]}}}"
            ]
        )
        self.assertEqual(parsed["event_type"], "review_failed")
        self.assertEqual(parsed["payload"]["notes"]["findings"], ["gap"])

    def test_import_legacy_preserves_completed_state(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_root = Path(temp_dir)
            original_root = runtime_paths.RUNTIME_ROOT
            runtime_paths.RUNTIME_ROOT = temp_root / "runtime"
            try:
                config_path = temp_root / "config.json"
                config_path.write_text(json.dumps(minimal_config_public(1)), encoding="utf-8")
                legacy_path = temp_root / "legacy.json"
                legacy_path.write_text(json.dumps(legacy_completed_state()), encoding="utf-8")
                run_id = import_legacy_run(legacy_path, config_path, "legacy001")
                projection = refresh_projection(config_path, run_id)
            finally:
                runtime_paths.RUNTIME_ROOT = original_root
            self.assertIsNone(projection["current"])
            self.assertIsNotNone(projection["completed_at"])
            self.assertEqual(projection["phases"][0]["status"], "complete")
            self.assertEqual(projection["phases"][0]["qa_status"], "passed")

    def test_import_legacy_preserves_repair_cursor(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_root = Path(temp_dir)
            original_root = runtime_paths.RUNTIME_ROOT
            runtime_paths.RUNTIME_ROOT = temp_root / "runtime"
            try:
                config_path = temp_root / "config.json"
                config_path.write_text(json.dumps(minimal_config_public(2)), encoding="utf-8")
                legacy_path = temp_root / "legacy.json"
                legacy_path.write_text(json.dumps(legacy_repair_state()), encoding="utf-8")
                run_id = import_legacy_run(legacy_path, config_path, "repair001")
                projection = refresh_projection(config_path, run_id)
            finally:
                runtime_paths.RUNTIME_ROOT = original_root
            self.assertEqual(projection["current"], {"phase": 1, "turn": "repair"})
            self.assertEqual(projection["phases"][0]["status"], "regressed")
            self.assertEqual(projection["phases"][0]["qa_status"], "failed")

    def test_import_legacy_preserves_test_repair_implement_cursor(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_root = Path(temp_dir)
            original_root = runtime_paths.RUNTIME_ROOT
            runtime_paths.RUNTIME_ROOT = temp_root / "runtime"
            try:
                config_path = temp_root / "config.json"
                config_path.write_text(json.dumps(minimal_config_public(2)), encoding="utf-8")
                legacy_path = temp_root / "legacy.json"
                legacy_path.write_text(json.dumps(legacy_test_repair_implement_state()), encoding="utf-8")
                run_id = import_legacy_run(legacy_path, config_path, "testrepair001")
                projection = refresh_projection(config_path, run_id)
            finally:
                runtime_paths.RUNTIME_ROOT = original_root
            self.assertEqual(projection["current"], {"phase": 1, "turn": "test_repair_implement"})
            self.assertEqual(projection["phases"][0]["status"], "complete")
            self.assertEqual(projection["phases"][0]["qa_status"], "passed")


def minimal_config() -> dict:
    config = minimal_config_public()
    config["_config_path"] = "C:/tmp/config.json"
    return config


def minimal_config_public(phase_count: int = 2) -> dict:
    return {
        "schema_version": 1,
        "project": {
            "name": "test run",
            "cwd": str(RUNNER_DIR.parents[2]),
            "spec_file": "README.md",
            "context_files": ["README.md"],
        },
        "turn_templates": {
            "plan": "templates/plan.md",
            "implement": "templates/implement.md",
            "review": "templates/review_test_hardening.md",
            "repair": "templates/repair.md",
            "test_review": "templates/test_review.md",
            "test_repair_plan": "templates/test_repair_plan.md",
            "test_repair_implement": "templates/test_repair_implement.md",
            "code_quality_review": "templates/code_quality_review.md",
        },
        "contract_template": "templates/_contract_test_hardening.md",
        "session_defaults": {
            "command": "codex",
            "model": "gpt-5.4",
            "reasoning_effort": "medium",
            "config": {"approval_policy": "never", "sandbox_mode": "danger-full-access"},
        },
        "phases": [phase_row(index, f"Phase {index}") for index in range(1, phase_count + 1)],
    }


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


def event(event_type: str, sequence: int, phase_id: int | None = None, turn: str | None = None) -> dict:
    return {
        "run_id": "run123",
        "sequence": sequence,
        "at": "2026-07-01T00:00:00+00:00",
        "event_type": event_type,
        "phase_id": phase_id,
        "turn": turn,
        "thread_id": None,
        "payload": {},
    }


def legacy_completed_state() -> dict:
    return {
        "session": {"thread_id": "thread-1"},
        "current": None,
        "phases": [
            {
                "id": 1,
                "status": "complete",
                "qa_status": "passed",
                "notes": {
                    "plan": ["planned"],
                    "done": ["done"],
                    "remaining": [],
                    "findings": [],
                    "verification": ["checked"],
                },
            }
        ],
    }


def legacy_repair_state() -> dict:
    return {
        "session": {"thread_id": "thread-1"},
        "current": {"phase": 1, "turn": "repair"},
        "phases": [
            {
                "id": 1,
                "status": "regressed",
                "qa_status": "failed",
                "notes": {
                    "plan": ["planned"],
                    "done": ["implemented"],
                    "remaining": [],
                    "findings": ["review gap"],
                    "verification": ["checked"],
                },
            }
        ],
    }


def legacy_test_repair_implement_state() -> dict:
    return {
        "session": {"thread_id": "thread-1"},
        "current": {"phase": 1, "turn": "test_repair_implement"},
        "phases": [
            {
                "id": 1,
                "status": "complete",
                "qa_status": "passed",
                "notes": {
                    "plan": ["planned"],
                    "done": ["implemented"],
                    "remaining": [],
                    "findings": ["test finding"],
                    "verification": ["checked"],
                },
            }
        ],
    }


if __name__ == "__main__":
    unittest.main()

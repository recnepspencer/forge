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
from agent_cli import build_command, runner_timeouts, timeout_reason
from config_schema import load_config, validate_config
from event_log import load_events, validate_event_log
from orchestrator import (
    extract_runner_event,
    import_legacy_run,
    maybe_reset_stuck_session,
    pending_recovery_reason,
    qa_repair_cycles_since_last_reset,
    refresh_projection,
    turn_is_current,
)
from projector import project_run
from runtime_paths import RuntimePaths, acquire_active_run_lock
from runtime_paths import clear_stop_requested, mark_stop_requested, stop_requested


class DurableRunnerTests(unittest.TestCase):
    def test_real_config_validates(self) -> None:
        config_path = RUNNER_DIR / "config" / "worth-ui-milestone-3.8.json"
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

    def test_phase_id_start_allows_zero_based_milestones(self) -> None:
        config = minimal_config_public()
        config["project"]["cwd"] = str(RUNNER_DIR.parents[1])
        config["project"]["spec_file"] = "AGENTS.md"
        config["project"]["context_files"] = ["AGENTS.md"]
        config["runner_control"] = {"phase_id_start": 0}
        config["phases"] = [phase_row(0, "Phase 0"), phase_row(1, "Phase 1")]
        config["_config_path"] = "C:/tmp/config.json"

        self.assertEqual(validate_config(config, RUNNER_DIR / "config" / "zero-start.json"), [])

        started = project_run(config, [event("run_started", 1)], "run123")
        self.assertEqual(started["current"], {"phase": 0, "turn": "plan"})

        after_phase_zero = project_run(
            config,
            [
                event("run_started", 1),
                event("plan_posted", 2, 0, "plan"),
                event("implementation_completed", 3, 0, "implement"),
                event("review_passed", 4, 0, "review"),
                event("test_review_passed", 5, 0, "test_review"),
                event("code_quality_review_passed", 6, 0, "code_quality_review"),
            ],
            "run123",
        )
        self.assertEqual(after_phase_zero["current"], {"phase": 1, "turn": "plan"})

        after_phase_one = project_run(
            config,
            [
                event("run_started", 1),
                event("plan_posted", 2, 0, "plan"),
                event("implementation_completed", 3, 0, "implement"),
                event("review_passed", 4, 0, "review"),
                event("test_review_passed", 5, 0, "test_review"),
                event("code_quality_review_passed", 6, 0, "code_quality_review"),
                event("plan_posted", 7, 1, "plan"),
                event("implementation_completed", 8, 1, "implement"),
                event("review_passed", 9, 1, "review"),
                event("test_review_passed", 10, 1, "test_review"),
                event("code_quality_review_passed", 11, 1, "code_quality_review"),
            ],
            "run123",
        )
        self.assertIsNone(after_phase_one["current"])

    def test_phase_advancement_follows_config_order_not_id_arithmetic(self) -> None:
        config = minimal_config_public()
        config["project"]["cwd"] = str(RUNNER_DIR.parents[1])
        config["project"]["spec_file"] = "AGENTS.md"
        config["project"]["context_files"] = ["AGENTS.md"]
        config["runner_control"] = {"phase_id_start": 0}
        config["phases"] = [phase_row(0, "Phase 0"), phase_row(10, "Phase 10")]
        config["_config_path"] = "C:/tmp/config.json"

        self.assertEqual(validate_config(config, RUNNER_DIR / "config" / "sparse.json"), [])

        after_phase_zero = project_run(
            config,
            [
                event("run_started", 1),
                event("plan_posted", 2, 0, "plan"),
                event("implementation_completed", 3, 0, "implement"),
                event("review_passed", 4, 0, "review"),
                event("test_review_passed", 5, 0, "test_review"),
                event("code_quality_review_passed", 6, 0, "code_quality_review"),
            ],
            "run123",
        )
        self.assertEqual(after_phase_zero["current"], {"phase": 10, "turn": "plan"})

        after_phase_ten = project_run(
            config,
            [
                event("run_started", 1),
                event("plan_posted", 2, 0, "plan"),
                event("implementation_completed", 3, 0, "implement"),
                event("review_passed", 4, 0, "review"),
                event("test_review_passed", 5, 0, "test_review"),
                event("code_quality_review_passed", 6, 0, "code_quality_review"),
                event("plan_posted", 7, 10, "plan"),
                event("implementation_completed", 8, 10, "implement"),
                event("review_passed", 9, 10, "review"),
                event("test_review_passed", 10, 10, "test_review"),
                event("code_quality_review_passed", 11, 10, "code_quality_review"),
            ],
            "run123",
        )
        self.assertIsNone(after_phase_ten["current"])

    def test_phase_validation_rejects_duplicate_phase_ids(self) -> None:
        config = minimal_config_public()
        config["project"]["cwd"] = str(RUNNER_DIR.parents[1])
        config["project"]["spec_file"] = "AGENTS.md"
        config["project"]["context_files"] = ["AGENTS.md"]
        config["phases"] = [phase_row(1, "Phase 1"), phase_row(1, "Phase 1 again")]

        errors = validate_config(config, RUNNER_DIR / "config" / "duplicate.json")
        self.assertTrue(any("duplicates phase id 1" in error for error in errors))

    def test_projection_repairs_stale_cursor_after_phase_is_complete(self) -> None:
        config = minimal_config()
        events = [
            event("run_started", 1),
            event("plan_posted", 2, 1, "plan"),
            event("implementation_completed", 3, 1, "implement"),
            event("review_passed", 4, 1, "review"),
            event("test_review_passed", 5, 1, "test_review"),
            event("code_quality_review_passed", 6, 1, "code_quality_review"),
            event(
                "prompt_selected",
                7,
                1,
                "test_repair_implement",
                payload={"turn_instance_id": "stale-1"},
            ),
        ]
        projection = project_run(config, events, "run123")
        self.assertEqual(projection["current"], {"phase": 2, "turn": "plan"})
        self.assertIsNone(projection["current_turn_instance_id"])

    def test_run_resumed_can_clear_persisted_session_thread(self) -> None:
        config = minimal_config()
        events = [
            {
                **event("run_started", 1),
                "thread_id": "thread-old",
            },
            {
                **event(
                    "run_resumed",
                    2,
                    payload={"reason": "operator resume", "reset_session_thread": True},
                ),
                "thread_id": None,
            },
        ]
        projection = project_run(config, events, "run123")
        self.assertIsNone(projection["session"]["thread_id"])
    def test_code_quality_review_failure_reopens_repair_gate(self) -> None:
        config = minimal_config()
        events = [
            event("run_started", 1),
            event("plan_posted", 2, 1, "plan"),
            event("implementation_completed", 3, 1, "implement"),
            event("review_passed", 4, 1, "review"),
            event("test_review_passed", 5, 1, "test_review"),
            event(
                "code_quality_review_failed",
                6,
                1,
                "code_quality_review",
                payload={"notes": {"findings": ["flat proof-flow directory"]}},
            ),
        ]
        projection = project_run(config, events, "run123")
        self.assertEqual(projection["current"], {"phase": 1, "turn": "code_quality_repair"})
        self.assertEqual(projection["phases"][0]["status"], "regressed")
        self.assertEqual(projection["phases"][0]["qa_status"], "failed")
        self.assertEqual(
            projection["phases"][0]["notes"]["findings"],
            ["flat proof-flow directory"],
        )

    def test_code_quality_repair_returns_to_structural_review(self) -> None:
        config = minimal_config()
        events = [
            event("run_started", 1),
            event("plan_posted", 2, 1, "plan"),
            event("implementation_completed", 3, 1, "implement"),
            event("review_passed", 4, 1, "review"),
            event("test_review_passed", 5, 1, "test_review"),
            event("code_quality_review_failed", 6, 1, "code_quality_review"),
            event("code_quality_repair_completed", 7, 1, "code_quality_repair"),
        ]
        projection = project_run(config, events, "run123")
        self.assertEqual(projection["current"], {"phase": 1, "turn": "code_quality_review"})
        self.assertEqual(projection["phases"][0]["status"], "complete")
        self.assertEqual(projection["phases"][0]["qa_status"], "needed")

    def test_qa_repair_cycle_count_resets_after_fresh_session_reset(self) -> None:
        events = [
            event("repair_completed", 1, 1, "repair"),
            event("code_quality_repair_completed", 2, 1, "code_quality_repair"),
            event(
                "session_reset",
                3,
                1,
                "code_quality_review",
                {"reason": "fresh session", "cycle_count": 2, "threshold": 2},
            ),
            event("repair_completed", 4, 1, "repair"),
        ]

        self.assertEqual(qa_repair_cycles_since_last_reset(events, 1), 1)

    def test_stuck_qa_repair_cycles_clear_persisted_session_once(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_root = Path(temp_dir)
            original_root = runtime_paths.RUNTIME_ROOT
            runtime_paths.RUNTIME_ROOT = temp_root / "runtime"
            try:
                config = minimal_config_public()
                config["runner_control"] = {"fresh_session_after_qa_repair_cycles": 2}
                config_path = temp_root / "config.json"
                config_path.write_text(json.dumps(config), encoding="utf-8")
                paths = RuntimePaths("fresh-reset")
                paths.events.parent.mkdir(parents=True, exist_ok=True)
                events = [
                    event("run_started", 1, payload={"config_path": str(config_path)}),
                    event("plan_posted", 2, 1, "plan"),
                    event("implementation_completed", 3, 1, "implement"),
                    event("review_failed", 4, 1, "review"),
                    event("repair_completed", 5, 1, "repair"),
                    event("review_failed", 6, 1, "review"),
                    event("repair_completed", 7, 1, "repair"),
                ]
                events[-1]["thread_id"] = "old-thread"
                paths.events.write_text(
                    "\n".join(json.dumps({**item, "run_id": "fresh-reset"}) for item in events)
                    + "\n",
                    encoding="utf-8",
                )
                projection = refresh_projection(config_path, "fresh-reset")

                self.assertTrue(maybe_reset_stuck_session(config_path, "fresh-reset", projection))
                reset_projection = refresh_projection(config_path, "fresh-reset")
                self.assertIsNone(reset_projection["session"]["thread_id"])
                self.assertEqual(
                    reset_projection["session"]["fresh_recovery"]["cycle_count"],
                    2,
                )
                self.assertFalse(
                    maybe_reset_stuck_session(config_path, "fresh-reset", reset_projection)
                )
            finally:
                runtime_paths.RUNTIME_ROOT = original_root
    def test_projection_keeps_prompt_instance_after_cursor_repairs_to_next_phase(self) -> None:
        config = minimal_config()
        events = [
            event("run_started", 1),
            event("plan_posted", 2, 1, "plan"),
            event("implementation_completed", 3, 1, "implement"),
            event("review_passed", 4, 1, "review"),
            event("test_review_passed", 5, 1, "test_review"),
            event("code_quality_review_passed", 6, 1, "code_quality_review"),
            event(
                "prompt_selected",
                7,
                2,
                "plan",
                payload={"turn_instance_id": "phase-2-plan-1"},
            ),
        ]
        projection = project_run(config, events, "run123")
        self.assertEqual(projection["current"], {"phase": 2, "turn": "plan"})
        self.assertEqual(projection["current_turn_instance_id"], "phase-2-plan-1")

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

    def test_extract_runner_event_rejects_wrong_turn_instance(self) -> None:
        with self.assertRaisesRegex(ValueError, "turn_instance_id must be 'expected-1'"):
            extract_runner_event(
                [
                    "RUNNER_EVENT: "
                    "{\"event_type\":\"review_failed\","
                    "\"payload\":{\"turn_instance_id\":\"wrong-1\",\"notes\":{\"findings\":[\"gap\"]}}}"
                ],
                "expected-1",
            )

    def test_extract_runner_event_rejects_malformed_note_payloads(self) -> None:
        with self.assertRaisesRegex(ValueError, "payload.notes.done must be a list of strings"):
            extract_runner_event(
                [
                    "RUNNER_EVENT: "
                    "{\"event_type\":\"implementation_completed\","
                    "\"payload\":{\"notes\":{\"done\":\"not-a-list\"}}}"
                ]
            )

    def test_pending_recovery_reason_detects_completed_turn_without_outcome(self) -> None:
        current = {"phase": 1, "turn": "review"}
        events = [
            event("run_started", 1),
            event("prompt_selected", 2, 1, "review", {"turn_instance_id": "review-1"}),
            event("codex_turn_completed", 3, 1, "review", {"turn_instance_id": "review-1"}),
        ]
        self.assertEqual(
            pending_recovery_reason(events, current, "review-1"),
            "prior agent turn completed but outcome was not recorded",
        )

    def test_pending_recovery_reason_ignores_stale_same_turn_from_prior_attempt(self) -> None:
        current = {"phase": 1, "turn": "review"}
        events = [
            event("run_started", 1),
            event("prompt_selected", 2, 1, "review", {"turn_instance_id": "review-old"}),
            event("codex_turn_completed", 3, 1, "review", {"turn_instance_id": "review-old"}),
            event("review_failed", 4, 1, "review"),
            event("repair_completed", 5, 1, "repair"),
        ]
        self.assertIsNone(pending_recovery_reason(events, current, None))
        self.assertIsNone(pending_recovery_reason(events, current, "review-new"))

    def test_turn_is_current_false_after_phase_advances(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_root = Path(temp_dir)
            original_root = runtime_paths.RUNTIME_ROOT
            runtime_paths.RUNTIME_ROOT = temp_root / "runtime"
            try:
                config_path = temp_root / "config.json"
                config_path.write_text(json.dumps(minimal_config_public(2)), encoding="utf-8")
                events_path = RuntimePaths("turn-check").events
                events_path.parent.mkdir(parents=True, exist_ok=True)
                events_path.write_text(
                    "\n".join(
                        json.dumps({**item, "run_id": "turn-check"})
                        for item in [
                            event("run_started", 1),
                            event("plan_posted", 2, 1, "plan"),
                            event("implementation_completed", 3, 1, "implement"),
                            event("review_passed", 4, 1, "review"),
                            event("test_review_passed", 5, 1, "test_review"),
                            event("code_quality_review_passed", 6, 1, "code_quality_review"),
                        ]
                    )
                    + "\n",
                    encoding="utf-8",
                )
                self.assertFalse(
                    turn_is_current(
                        config_path,
                        "turn-check",
                        {"phase": 1, "turn": "review"},
                        "review-old",
                    )
                )
            finally:
                runtime_paths.RUNTIME_ROOT = original_root

    def test_load_events_ignores_incomplete_tail_line(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            path = Path(temp_dir) / "events.jsonl"
            path.write_text(
                json.dumps(event("run_started", 1)) + "\n" + '{"run_id":"run123"',
                encoding="utf-8",
            )
            events = load_events(path)
        self.assertEqual(len(events), 1)
        self.assertEqual(events[0]["event_type"], "run_started")

    def test_active_run_lock_rejects_second_holder(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            original_root = runtime_paths.RUNTIME_ROOT
            runtime_paths.RUNTIME_ROOT = Path(temp_dir) / "runtime"
            try:
                paths = RuntimePaths("lock-test")
                with acquire_active_run_lock(paths):
                    with self.assertRaisesRegex(RuntimeError, "already active"):
                        with acquire_active_run_lock(paths):
                            self.fail("second lock holder unexpectedly acquired the active run lock")
            finally:
                runtime_paths.RUNTIME_ROOT = original_root

    def test_runner_timeouts_and_timeout_reason_use_runner_control(self) -> None:
        timeouts = runner_timeouts(
            {"runner_control": {"turn_timeout_seconds": 12, "idle_timeout_seconds": 3}}
        )
        self.assertEqual(timeouts, {"turn_timeout_seconds": 12, "idle_timeout_seconds": 3})
        self.assertEqual(
            timeout_reason(2.0, 3.5, timeouts),
            "agent turn produced no output for 3 seconds",
        )
        self.assertEqual(
            timeout_reason(12.5, 1.0, timeouts),
            "agent turn timed out after 12 seconds",
        )

    def test_grok_provider_uses_prompt_file_and_streaming_json(self) -> None:
        state = {
            "project": {"cwd": "C:/work/project"},
            "session": {
                "provider": "grok",
                "command": "C:/Users/Esther/.grok/bin/grok.exe",
                "model": "grok-composer-2.5-fast",
                "config": {"sandbox_mode": "danger-full-access"},
            },
        }
        prompt_path = Path("C:/tmp/prompt.txt")
        command = build_command(state, prompt_path)
        self.assertEqual(command[0], "C:/Users/Esther/.grok/bin/grok.exe")
        self.assertIn("--prompt-file", command)
        self.assertIn(str(prompt_path), command)
        self.assertIn("--output-format", command)
        self.assertIn("streaming-json", command)
        self.assertIn("--model", command)
        self.assertIn("grok-composer-2.5-fast", command)
        self.assertIn("--always-approve", command)
        self.assertIn("--sandbox", command)
        self.assertIn("off", command)

    def test_stop_request_marker_round_trips(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            original_root = runtime_paths.RUNTIME_ROOT
            runtime_paths.RUNTIME_ROOT = Path(temp_dir) / "runtime"
            try:
                paths = RuntimePaths("stop-test")
                self.assertFalse(stop_requested(paths))
                mark_stop_requested(paths)
                self.assertTrue(stop_requested(paths))
                clear_stop_requested(paths)
                self.assertFalse(stop_requested(paths))
            finally:
                runtime_paths.RUNTIME_ROOT = original_root

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

    def test_import_legacy_repairs_stale_test_repair_implement_cursor(self) -> None:
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
            self.assertEqual(projection["current"], {"phase": 2, "turn": "plan"})
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
            "repair_plan": "templates/plan.md",
            "repair": "templates/repair.md",
            "test_review": "templates/test_review.md",
            "test_repair_plan": "templates/test_repair_plan.md",
            "test_repair_implement": "templates/test_repair_implement.md",
            "code_quality_review": "templates/code_quality_review.md",
            "code_quality_repair": "templates/code_quality_repair.md",
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

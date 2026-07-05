from __future__ import annotations

import copy
import sys
import unittest
from pathlib import Path

RUNNER_DIR = Path(__file__).resolve().parents[1]
if str(RUNNER_DIR) not in sys.path:
    sys.path.insert(0, str(RUNNER_DIR))

from config_schema import load_config, validate_config
from projector import project_run


class BoundaryReviewTurnTests(unittest.TestCase):
    def test_s7_config_accepts_boundary_review_turn(self) -> None:
        config_path = RUNNER_DIR / "config" / "forge-store-s7.json"
        config = load_config(config_path)
        self.assertEqual(validate_config(config, config_path), [])

    def test_configured_boundary_review_starts_and_advances_phases(self) -> None:
        config = minimal_boundary_config()
        first = project_run(config, [event("run_started", 1)], "run123")
        self.assertEqual(first["current"], {"phase": 1, "turn": "boundary_review"})

        after_boundary = project_run(
            config,
            [
                event("run_started", 1),
                event("boundary_review_completed", 2, 1, "boundary_review"),
            ],
            "run123",
        )
        self.assertEqual(after_boundary["current"], {"phase": 1, "turn": "plan"})

        after_close = project_run(
            config,
            [
                event("run_started", 1),
                event("boundary_review_completed", 2, 1, "boundary_review"),
                event("plan_posted", 3, 1, "plan"),
                event("implementation_completed", 4, 1, "implement"),
                event("review_passed", 5, 1, "review"),
                event("test_review_passed", 6, 1, "test_review"),
                event("code_quality_review_passed", 7, 1, "code_quality_review"),
            ],
            "run123",
        )
        self.assertEqual(after_close["current"], {"phase": 2, "turn": "boundary_review"})

    def test_boundary_review_prompt_instance_survives_next_phase_cursor_repair(self) -> None:
        config = minimal_boundary_config()
        projection = project_run(
            config,
            [
                event("run_started", 1),
                event("boundary_review_completed", 2, 1, "boundary_review"),
                event("plan_posted", 3, 1, "plan"),
                event("implementation_completed", 4, 1, "implement"),
                event("review_passed", 5, 1, "review"),
                event("test_review_passed", 6, 1, "test_review"),
                event("code_quality_review_passed", 7, 1, "code_quality_review"),
                event(
                    "prompt_selected",
                    8,
                    2,
                    "boundary_review",
                    {"turn_instance_id": "phase-2-boundary-1"},
                ),
            ],
            "run123",
        )
        self.assertEqual(projection["current"], {"phase": 2, "turn": "boundary_review"})
        self.assertEqual(projection["current_turn_instance_id"], "phase-2-boundary-1")

    def test_boundary_review_config_replays_pre_boundary_plan_events(self) -> None:
        config = minimal_boundary_config()
        projection = project_run(
            config,
            [
                event("run_started", 1),
                event("plan_posted", 2, 1, "plan"),
            ],
            "run123",
        )
        self.assertEqual(projection["current"], {"phase": 1, "turn": "implement"})

    def test_boundary_review_config_repairs_pre_boundary_next_phase_plan_prompt(self) -> None:
        config = minimal_boundary_config()
        projection = project_run(
            config,
            [
                event("run_started", 1),
                event("boundary_review_completed", 2, 1, "boundary_review"),
                event("plan_posted", 3, 1, "plan"),
                event("implementation_completed", 4, 1, "implement"),
                event("review_passed", 5, 1, "review"),
                event(
                    "prompt_selected",
                    6,
                    2,
                    "plan",
                    {"turn_instance_id": "pre-boundary-plan"},
                ),
                event("plan_posted", 7, 2, "plan"),
            ],
            "run123",
        )
        self.assertEqual(projection["current"], {"phase": 2, "turn": "implement"})


def minimal_boundary_config() -> dict:
    config = copy.deepcopy(load_config(RUNNER_DIR / "config" / "forge-store-s7.json"))
    config["phases"] = config["phases"][:2]
    return config


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

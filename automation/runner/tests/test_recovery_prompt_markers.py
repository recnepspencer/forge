from __future__ import annotations

import unittest

from runner.graph_runtime.prompt_turns import expected_runner_event_markers


class RecoveryPromptMarkerTests(unittest.TestCase):
    def test_single_outcome_names_exact_event_type_and_turn_instance(self) -> None:
        marker = expected_runner_event_markers(
            frozenset({"repair_completed"}),
            "repair-turn-1",
        )

        self.assertEqual(
            marker,
            'RUNNER_EVENT: {"event_type":"repair_completed","payload":{"turn_instance_id":"repair-turn-1"}}',
        )

    def test_multiple_outcomes_are_explicit_and_deterministic(self) -> None:
        markers = expected_runner_event_markers(
            frozenset({"review_passed", "review_failed"}),
            "review-turn-1",
        ).splitlines()

        self.assertEqual(
            markers,
            [
                'RUNNER_EVENT: {"event_type":"review_failed","payload":{"turn_instance_id":"review-turn-1"}}',
                'RUNNER_EVENT: {"event_type":"review_passed","payload":{"turn_instance_id":"review-turn-1"}}',
            ],
        )

    def test_missing_outcome_contract_fails_before_prompt_delivery(self) -> None:
        with self.assertRaisesRegex(ValueError, "at least one supported runner outcome"):
            expected_runner_event_markers(frozenset(), "turn-1")


if __name__ == "__main__":
    unittest.main()

from __future__ import annotations

import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

from runner.authority.run_identity import RuntimePaths
from runner.operator_signals.dispatcher import dispatch_authority_event
from runner.operator_signals.detectors import signals_for_event
from runner.operator_signals.policies.validation import validate_notification_policy
from runner.operator_signals.replay import planned_fanout_for_event, replay_signal_fanout
from runner.graph_runtime.runtime_lane import dispatch_appended_event


def policy() -> dict:
    return {"project": {"name": "signals"}, "session_defaults": {"provider": "codex", "model": "test"}, "phases": [{"id": 7, "title": "Signals"}], "notification_policy": {"signals": {
        "crash": {"enabled": True, "delivery": "immediate", "sinks": ["file"]},
        "no_edit_stall": {"enabled": True, "delivery": "immediate", "sinks": ["file"]},
        "run_completed": {"enabled": True, "delivery": "final", "sinks": ["file"]},
    }}}


def crash_event(sequence: int = 4) -> dict:
    return {"run_id": "signals", "sequence": sequence, "event_type": "runner_fault", "phase_id": 7,
            "turn": "implement", "payload": {"reason": "crashed", "failure_family": "provider_crash"}}


class OperatorSignalTests(unittest.TestCase):
    def test_replay_and_live_use_the_same_canonical_signal(self) -> None:
        event = crash_event()
        self.assertEqual(replay_signal_fanout(policy(), [event]), planned_fanout_for_event(policy(), event))
        payload = planned_fanout_for_event(policy(), event)[0].payload
        self.assertEqual(payload["project_name"], "signals")
        self.assertIn("event_log_file", payload["details"])

    def test_healthy_event_does_not_produce_stall(self) -> None:
        event = {"run_id": "signals", "sequence": 5, "event_type": "prompt_selected", "phase_id": 7, "turn": "implement", "payload": {}}
        self.assertEqual(signals_for_event(event), ())

    def test_non_signal_event_never_loads_runtime_config(self) -> None:
        event = {"run_id": "signals", "sequence": 5, "event_type": "prompt_selected", "phase_id": 7, "turn": "implement", "payload": {}}
        with patch("runner.graph_runtime.runtime_lane.load_admitted_projection_inputs") as load:
            dispatch_appended_event(RuntimePaths("signals"), event)
        load.assert_not_called()

    def test_file_delivery_is_derived_and_does_not_change_event(self) -> None:
        with tempfile.TemporaryDirectory() as temp:
            paths = RuntimePaths("signals")
            with patch("runner.authority.run_identity.runtime_paths.CANONICAL_RUNTIME_ROOT", Path(temp)):
                event = crash_event()
                deliveries = dispatch_authority_event(paths, policy(), event)
                self.assertEqual(len(deliveries), 1)
                self.assertFalse(paths.events.exists())
                self.assertTrue(paths.notifications.exists())

    def test_unknown_signal_policy_is_denied(self) -> None:
        errors: list[str] = []
        validate_notification_policy({"signals": {"invented": {"enabled": True, "delivery": "immediate", "sinks": []}}}, errors)
        self.assertTrue(errors)

    def test_hook_failure_stays_derived(self) -> None:
        configured = policy()
        configured["notification_policy"]["signals"]["crash"]["sinks"] = ["command_hook"]
        configured["notification_policy"]["command_hook"] = ["missing-hook"]
        deliveries = dispatch_authority_event(RuntimePaths("signals"), configured, crash_event())
        self.assertEqual(deliveries, ({"sink": "command_hook", "delivered": False},))

    def test_unexpected_sink_failure_cannot_crash_the_runner(self) -> None:
        with patch("runner.operator_signals.dispatcher.deliver", side_effect=RuntimeError("bridge bug")):
            deliveries = dispatch_authority_event(RuntimePaths("signals"), policy(), crash_event())
        self.assertEqual(deliveries, ({"sink": "file", "delivered": False},))

    def test_recovery_success_is_suppressed_without_a_policy_signal(self) -> None:
        event = {"run_id": "signals", "sequence": 6, "event_type": "recovery_completed", "phase_id": 7, "turn": "implement", "payload": {}}
        self.assertEqual(replay_signal_fanout(policy(), [event]), ())


if __name__ == "__main__":
    unittest.main()

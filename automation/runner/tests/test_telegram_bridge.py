from __future__ import annotations

import unittest
from unittest.mock import patch

from runner.telegram_bridge.cli import format_signal
from runner.telegram_bridge.routing import route_reply


def reply_update(message_id: int = 81, update_id: int = 901) -> dict:
    return {
        "update_id": update_id,
        "message": {
            "chat": {"id": 1234},
            "text": "retry once, then pause",
            "reply_to_message": {"message_id": message_id},
        },
    }


class TelegramBridgeTests(unittest.TestCase):
    def test_reply_routes_by_recorded_message_not_human_metadata(self) -> None:
        alert = {"signal_id": "run:4:crash", "run_id": "run", "phase_id": 6, "turn": "review"}
        with (
            patch("runner.telegram_bridge.routing.find_alert", return_value=alert),
            patch("runner.telegram_bridge.routing.operator_override_source_exists", return_value=False),
            patch("runner.telegram_bridge.routing.inject_operator_override") as inject,
        ):
            disposition = route_reply(reply_update(), "1234")
        self.assertEqual(disposition.status, "injected")
        self.assertEqual(disposition.signal_id, "run:4:crash")
        inject.assert_called_once_with(
            "run", "retry once, then pause", phase_id=6, turn="review", source_id="telegram:901"
        )

    def test_unrelated_or_stale_reply_never_reaches_a_runner(self) -> None:
        with patch("runner.telegram_bridge.routing.inject_operator_override") as inject:
            self.assertEqual(route_reply(reply_update(), "different-chat").status, "rejected_wrong_chat")
        inject.assert_not_called()
        with (
            patch("runner.telegram_bridge.routing.find_alert", return_value={"signal_id": "old", "run_id": "run", "phase_id": 6, "turn": "review"}),
            patch("runner.telegram_bridge.routing.operator_override_source_exists", return_value=False),
            patch("runner.telegram_bridge.routing.inject_operator_override", side_effect=ValueError("stale cursor")) as inject,
        ):
            self.assertEqual(route_reply(reply_update(), "1234").status, "rejected_stale")
        inject.assert_called_once()

    def test_outbound_alert_instructs_a_plain_reply(self) -> None:
        text = format_signal({"signal_kind": "crash", "summary": "provider failed", "project_name": "runner", "phase_id": 6, "turn": "review", "run_id": "r", "signal_id": "r:4:crash"})
        self.assertIn("Reply directly to this message", text)
        self.assertIn("r:4:crash", text)


if __name__ == "__main__":
    unittest.main()

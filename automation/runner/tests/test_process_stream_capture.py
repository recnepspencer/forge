from __future__ import annotations

import json
import unittest

from runner.adapters.process_runtime import update_capture_from_stream_line


class ProcessStreamCaptureTests(unittest.TestCase):
    def test_json_scalar_is_captured_as_plain_agent_output(self) -> None:
        capture: dict = {}

        update_capture_from_stream_line(capture, '"plain json text"')
        update_capture_from_stream_line(capture, '["plain", "json", "list"]')

        self.assertEqual(
            capture["agent_messages"],
            ['"plain json text"', '["plain", "json", "list"]'],
        )

    def test_grok_text_chunks_form_one_agent_message(self) -> None:
        capture: dict = {}

        for chunk in (
            "RUNNER_EVENT: ",
            '{"event_type":"repair_completed",',
            '"payload":{"turn_instance_id":"turn-1"}}',
        ):
            update_capture_from_stream_line(
                capture,
                json.dumps({"type": "text", "data": chunk}),
            )
        update_capture_from_stream_line(
            capture,
            json.dumps({"type": "end", "sessionId": "grok-session-1"}),
        )

        self.assertEqual(
            capture["agent_messages"],
            [
                'RUNNER_EVENT: {"event_type":"repair_completed",'
                '"payload":{"turn_instance_id":"turn-1"}}'
            ],
        )
        self.assertEqual(capture["session_id"], "grok-session-1")
        self.assertNotIn("streaming_agent_text", capture)


if __name__ == "__main__":
    unittest.main()

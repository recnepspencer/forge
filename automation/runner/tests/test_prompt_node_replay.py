import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

from runner.graph_runtime.nodes.prompt_nodes import replayable_ordinary_prompt


class FakePaths:
    def __init__(self, root: Path) -> None:
        self.events = root / "events.jsonl"
        self.instantiations = root / "instantiations"


class PromptNodeReplayTests(unittest.TestCase):
    def setUp(self) -> None:
        self.temporary = tempfile.TemporaryDirectory()
        self.paths = FakePaths(Path(self.temporary.name))

    def tearDown(self) -> None:
        self.temporary.cleanup()

    def write_prompt(self, turn_instance_id: str) -> None:
        prompt_root = self.paths.instantiations / turn_instance_id
        prompt_root.mkdir(parents=True)
        (prompt_root / "prompt.md").write_text("Do the work.", encoding="utf-8")

    def test_reuses_latest_published_identity(self) -> None:
        self.write_prompt("turn-2")
        events = [
            {"event_type": "prompt_selected", "phase_id": 4, "turn": "plan", "payload": {"turn_instance_id": "turn-1"}},
            {"event_type": "run_stopped", "phase_id": None, "turn": None, "payload": {}},
            {"event_type": "prompt_selected", "phase_id": 4, "turn": "plan", "payload": {"turn_instance_id": "turn-2"}},
        ]
        with patch("runner.graph_runtime.nodes.prompt_nodes.load_events", return_value=events):
            delivery = replayable_ordinary_prompt(self.paths, 4, "plan")
        self.assertIsNotNone(delivery)
        assert delivery is not None
        self.assertEqual(delivery.turn_instance_id, "turn-2")
        self.assertIn('"turn_instance_id":"turn-2"', delivery.delivery_prompt)

    def test_requires_immutable_prompt_artifact(self) -> None:
        events = [
            {"event_type": "prompt_selected", "phase_id": 4, "turn": "plan", "payload": {"turn_instance_id": "missing"}},
        ]
        with patch("runner.graph_runtime.nodes.prompt_nodes.load_events", return_value=events):
            self.assertIsNone(replayable_ordinary_prompt(self.paths, 4, "plan"))

    def test_rejects_turn_with_later_execution_evidence(self) -> None:
        self.write_prompt("turn-2")
        events = [
            {"event_type": "prompt_selected", "phase_id": 4, "turn": "plan", "payload": {"turn_instance_id": "turn-2"}},
            {"event_type": "codex_turn_completed", "phase_id": 4, "turn": "plan", "payload": {"turn_instance_id": "turn-2"}},
        ]
        with patch("runner.graph_runtime.nodes.prompt_nodes.load_events", return_value=events):
            self.assertIsNone(replayable_ordinary_prompt(self.paths, 4, "plan"))


if __name__ == "__main__":
    unittest.main()

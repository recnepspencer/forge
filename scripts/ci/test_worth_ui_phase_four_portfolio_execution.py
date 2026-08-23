from __future__ import annotations

import sys
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

sys.path.insert(0, str(Path(__file__).resolve().parent))

from worth_ui_ledger_phase_four_portfolio import PhaseFourPortfolioExecution


class PhaseFourPortfolioExecutionTests(unittest.TestCase):
    def test_font_row_preserves_runner_bound_public_example(self) -> None:
        observation = {
            "requirement": "P4-FONT-COLLECTION-01",
            "source_revision": "revision",
            "source_state_digest": "state",
            "execution_receipts": [
                {"role": "main-test"},
                {"role": "public-example"},
            ],
            "public_example_command": ["cargo", "check", "text_platform"],
            "construction_cost": "compile-sessions=1",
        }
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            execution = PhaseFourPortfolioExecution(
                root,
                root / "ledger.csv",
                root,
                root / "candidate.csv",
                [{"requirement": "P4-FONT-COLLECTION-01"}],
                {},
                lambda *_args, **_options: observation,
                "compile.json",
                [],
            )
            with patch(
                "worth_ui_ledger_phase_four_portfolio.record_proved_execution"
            ):
                execution.execute()

        self.assertEqual(observation["execution_receipts"][-1]["role"], "public-example")
        self.assertEqual(observation["public_example_command"][-1], "text_platform")
        self.assertIn("compile-sessions=1", observation["construction_cost"])


if __name__ == "__main__":
    unittest.main()

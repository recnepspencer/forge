import json
import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

from worth_ui_ledger_phase_three_portfolio import PhaseThreePortfolioExecution


class PhaseThreePortfolioTests(unittest.TestCase):
    def test_loaded_predecessor_requires_the_explicit_thirty_row_prefix(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            temporary = root / "target" / "worth-ui-3141-verify-fixture"
            temporary.mkdir(parents=True)
            requirements = frozenset(
                [f"P1-FIXTURE-{index:02}" for index in range(20)]
                + [f"P2-FIXTURE-{index:02}" for index in range(10)]
            )
            retained = [
                {"requirement": requirement, "exit_posture": "passed"}
                for requirement in sorted(requirements)
            ]
            handoff = temporary / "p3-predecessor-handoff.json"
            handoff.write_text(
                json.dumps(
                    {
                        "schema": "worth-ui-phase-predecessor-handoff-v4",
                        "through_phase": 2,
                        "source_revision": "revision",
                        "source_state_digest": "state",
                        "rows": retained,
                    }
                ),
                encoding="utf-8",
            )
            observation = {
                "source_revision": "revision",
                "source_state_digest": "state",
                "source_identity": [handoff.relative_to(root).as_posix()],
            }
            execution = PhaseThreePortfolioExecution(
                root,
                root / "ledger.csv",
                temporary,
                temporary / "candidate.csv",
                [],
                {},
                lambda *_args, **_kwargs: {},
                "compile.json",
                list(retained),
                requirements,
            )

            execution.validate_loaded_predecessor(observation)

            execution.predecessor_requirements = frozenset()
            with self.assertRaisesRegex(RuntimeError, "incomplete or non-passing"):
                execution.validate_loaded_predecessor(observation)


if __name__ == "__main__":
    unittest.main()

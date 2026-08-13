import json
import sys
import tempfile
import unittest
from pathlib import Path
from unittest.mock import Mock, patch

sys.path.insert(0, str(Path(__file__).resolve().parent))

from worth_ui_ledger_phase_three_portfolio import PhaseThreePortfolioExecution
from worth_ui_ledger_phase_four_portfolio import PhaseFourPortfolioExecution
from worth_ui_ledger_phase_two_portfolio import PhaseTwoPortfolioExecution
import verify_worth_ui_3141_ledger as verifier


class PhaseThreePortfolioTests(unittest.TestCase):
    def test_phase_four_font_row_preserves_runner_bound_public_example(self) -> None:
        observation = {
            "requirement": "P4-FONT-COLLECTION-01",
            "source_revision": "revision",
            "source_state_digest": "state",
            "execution_receipts": [
                {"role": "main-test"}, {"role": "public-example"}
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

    def test_shared_rows_call_the_strict_executor_with_named_dependencies(self) -> None:
        calls = []

        def exact_executor(row, artifact, compile_artifact, **options):
            calls.append((row["requirement"], artifact, compile_artifact, options))
            return {"requirement": row["requirement"]}

        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            execution = PhaseTwoPortfolioExecution(
                root, root / "ledger.csv", root, root / "candidate.csv", [], {},
                exact_executor, lambda _root: "revision",
            )
            execution.fresh_compile = "compile.json"
            execution.record = Mock()
            execution.rerun_headless_cost(
                {"requirement": "P1-HEADLESS-COST-01"}, "mounted.json"
            )
            execution.rerun_dependent_phase_two(
                [{"requirement": "P2-PIXELS-01"}], "native.json"
            )

        self.assertEqual(
            [call[3] for call in calls],
            [
                {
                    "shared_world_artifact": "mounted.json",
                    "candidate_ledger": root / "candidate.csv",
                },
                {
                    "shared_world_artifact": "native.json",
                    "candidate_ledger": root / "candidate.csv",
                },
            ],
        )

    def test_phase_four_handoff_is_written_after_phase_three_and_before_phase_four(self) -> None:
        with tempfile.TemporaryDirectory(dir=verifier.TARGET) as directory:
            events = []
            governed = [
                {"phase": str(phase), "requirement": f"P{phase}-FIXTURE"}
                for phase in (2, 3, 4)
            ]
            phase_two = Mock(fresh_compile="compile.json")
            phase_two.execute.return_value = [
                {"requirement": "P2-FIXTURE", "exit_posture": "passed"}
            ]

            def phase_three(*args, **_kwargs):
                execution = Mock()
                execution.execute.side_effect = lambda: (
                    events.append("phase-three"),
                    args[8].append({"requirement": "P3-FIXTURE", "exit_posture": "passed"}),
                )
                return execution

            def phase_four(*_args, **_kwargs):
                execution = Mock()
                execution.execute.side_effect = lambda: events.append("phase-four")
                return execution

            def write_artifact(identity, _payload):
                events.append(Path(identity).name)

            with (
                patch.object(verifier, "rows", return_value=governed),
                patch.object(verifier, "successor_reopenings", return_value={}),
                patch.object(verifier, "prepared_open_rows", return_value={}),
                patch.object(verifier, "write_candidate_ledger"),
                patch.object(verifier, "PhaseTwoPortfolioExecution", return_value=phase_two),
                patch.object(verifier, "PhaseThreePortfolioExecution", side_effect=phase_three),
                patch.object(verifier, "PhaseFourPortfolioExecution", side_effect=phase_four),
                patch.object(verifier, "closure_tests", return_value=2),
                patch.object(verifier, "predecessor_artifact", return_value={}),
                patch.object(verifier, "write_artifact", side_effect=write_artifact),
            ):
                verifier.OperationalPortfolio(
                    Mock(through_phase=4)
                ).execute_at(Path(directory))

            self.assertEqual(
                events,
                [
                    "p3-predecessor-handoff.json",
                    "phase-three",
                    "p4-predecessor-handoff.json",
                    "phase-four",
                ],
            )

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
                        "schema": "worth-ui-phase-predecessor-handoff-v1",
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

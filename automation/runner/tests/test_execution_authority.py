import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

from runner.graph_runtime.authority import CurrentTurnAuthority, LoadedRunAuthority
from runner.graph_runtime.continuation import ordinary_turn_continuation
from runner.graph_runtime.execution_authority import (
    ExecutionReceipt,
    claim_execution,
    finish_execution,
    load_execution,
    record_process_launch,
)
from runner.graph_runtime.nodes.execution_nodes import execute_role_turn, execution_identity
from runner.graph_runtime.execution_authority.stability_canary import run_stability_canary
from runner.graph_runtime.state import (
    PROMPT_TURN_KEY,
    ROLE_SESSION_KEY,
    RUN_AUTHORITY_KEY,
    RUN_CONTEXT_KEY,
    TURN_CONTINUATION_KEY,
    PromptTurnDelivery,
    RoleSessionSelection,
    RunContext,
)


class FakePaths:
    def __init__(self, root: Path) -> None:
        self.executions = root / "executions"
        self.log = root / "provider.jsonl"


class ExecutionAuthorityTests(unittest.TestCase):
    def setUp(self) -> None:
        self.temporary = tempfile.TemporaryDirectory()
        self.paths = FakePaths(Path(self.temporary.name))

    def tearDown(self) -> None:
        self.temporary.cleanup()

    def test_claim_launch_finish_is_durable(self) -> None:
        claimed, created = claim_execution(self.paths, "exec-1", "turn-1")
        self.assertTrue(created)
        self.assertEqual(claimed.state, "claimed")
        record_process_launch(self.paths, "exec-1", 4321)
        finish_execution(self.paths, "exec-1", 0, {"agent_messages": ["done"]})

        finished = load_execution(self.paths, "exec-1")
        self.assertIsNotNone(finished)
        assert finished is not None
        self.assertEqual(finished.state, "finished")
        self.assertEqual(finished.provider_pid, 4321)
        self.assertEqual(finished.capture, {"agent_messages": ["done"]})

    def test_reclaim_reports_replay_without_mutating_receipt(self) -> None:
        first, created = claim_execution(self.paths, "exec-1", "turn-1")
        replay, replay_created = claim_execution(self.paths, "exec-1", "turn-1")

        self.assertTrue(created)
        self.assertFalse(replay_created)
        self.assertEqual(replay, first)

    def test_recovery_prompt_has_separate_execution_identity(self) -> None:
        ordinary = execution_identity("turn-1", "ordinary prompt")
        recovery = execution_identity("turn-1", "recovery prompt")
        self.assertNotEqual(ordinary, recovery)

    def test_unfinished_replay_never_launches_provider_again(self) -> None:
        state = execution_state()
        interrupted = ExecutionReceipt("exec-1", "turn-1", "launched", 12, 34)
        with (
            patch("runner.graph_runtime.nodes.execution_nodes.RuntimePaths", return_value=self.paths),
            patch("runner.graph_runtime.nodes.execution_nodes.claim_execution", return_value=(interrupted, False)),
            patch("runner.graph_runtime.nodes.execution_nodes.run_agent") as run_agent,
        ):
            result = execute_role_turn(state)

        run_agent.assert_not_called()
        self.assertEqual(result["turn_execution"].exit_code, 125)
        self.assertEqual(result["turn_execution"].capture["failure_family"], "interrupted_execution")

    def test_finished_replay_consumes_capture_without_provider_launch(self) -> None:
        state = execution_state()
        finished = ExecutionReceipt("exec-1", "turn-1", "finished", 12, 34, 0, {"agent_messages": ["done"]})
        with (
            patch("runner.graph_runtime.nodes.execution_nodes.RuntimePaths", return_value=self.paths),
            patch("runner.graph_runtime.nodes.execution_nodes.claim_execution", return_value=(finished, False)),
            patch("runner.graph_runtime.nodes.execution_nodes.run_agent") as run_agent,
        ):
            result = execute_role_turn(state)

        run_agent.assert_not_called()
        self.assertEqual(result["turn_execution"].capture["agent_messages"], ["done"])

    def test_provider_capture_exception_is_finalized_as_failure(self) -> None:
        state = execution_state()
        claimed = ExecutionReceipt("exec-1", "turn-1", "claimed", 12)
        with (
            patch("runner.graph_runtime.nodes.execution_nodes.RuntimePaths", return_value=self.paths),
            patch("runner.graph_runtime.nodes.execution_nodes.claim_execution", return_value=(claimed, True)),
            patch("runner.graph_runtime.nodes.execution_nodes.build_inflight_no_progress_watchdog"),
            patch(
                "runner.graph_runtime.nodes.execution_nodes.run_agent",
                side_effect=AttributeError("bad provider frame"),
            ),
            patch("runner.graph_runtime.nodes.execution_nodes.finish_execution") as finish,
        ):
            result = execute_role_turn(state)

        self.assertEqual(result["turn_execution"].exit_code, 125)
        self.assertIn("bad provider frame", result["turn_execution"].capture["failure_reason"])
        finish.assert_called_once()

    def test_three_turn_stability_canary(self) -> None:
        report = run_stability_canary()
        self.assertTrue(report["healthy"])


def execution_state() -> dict:
    return {
        RUN_CONTEXT_KEY: RunContext("run-1", Path("config.json"), None),
        RUN_AUTHORITY_KEY: LoadedRunAuthority({}, {"session": {}}),
        "current_turn_authority": CurrentTurnAuthority(4, "plan"),
        PROMPT_TURN_KEY: PromptTurnDelivery("turn-1", "ordinary prompt"),
        ROLE_SESSION_KEY: RoleSessionSelection(None),
        TURN_CONTINUATION_KEY: ordinary_turn_continuation(),
    }


if __name__ == "__main__":
    unittest.main()

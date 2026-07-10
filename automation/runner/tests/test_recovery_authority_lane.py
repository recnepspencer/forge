from __future__ import annotations

import unittest
from pathlib import Path
import tempfile
from unittest.mock import patch

from runner.authority.events import load_events
from runner.authority.run_identity import runtime_paths
from runner.authority.run_identity import RuntimePaths
from runner.graph_runtime.continuation.recovery_planning import plan_recovery_attempt
from runner.graph_runtime.continuation.requests import RecoveryTurnRequest, ordinary_turn_continuation
from runner.graph_runtime.authority import CurrentTurnAuthority, LoadedRunAuthority
from runner.graph_runtime.nodes.authority_event_nodes import append_runner_event
from runner.graph_runtime.recovery_disposition import execute_exhausted_recovery_disposition
from runner.graph_runtime.recovery_events import record_recovery_attempt
from runner.graph_runtime.qualifying_edits import qualifying_git_diff_exists
from runner.graph_runtime.recovery_runtime import maybe_handle_no_edit_stall, open_preflight_fault_exists
from runner.graph_runtime.orchestrator import drive_graph_run
from runner.phase_programs.policy_bindings import EscalationFamilyPolicy, EscalationStage, QualifyingEditPolicy, StallSignalPolicy
from runner.graph_runtime.state import (
    PROMPT_TURN_KEY,
    RUN_AUTHORITY_KEY,
    RUN_CONTEXT_KEY,
    TURN_CONTINUATION_KEY,
    TURN_EXECUTION_KEY,
    TURN_OUTCOME_KEY,
    TURN_TRANSITION_KEY,
    PreOutcomeFailure,
    PromptTurnDelivery,
    RunContext,
    TurnExecutionCapture,
)


class RecoveryAuthorityLaneTests(unittest.TestCase):
    def test_loop_ladder_walks_stages_and_forces_fresh_session(self) -> None:
        policy = EscalationFamilyPolicy(
            family_name="same_phase_loop_exceeded",
            stages=(EscalationStage("start_fresh_session"), EscalationStage("deep_reviewer_pass")),
            on_exhausted=EscalationStage("notify_and_pause"),
        )
        with patch(
            "runner.graph_runtime.continuation.recovery_planning.escalation_policy_for_failure_family",
            return_value=policy,
        ):
            request = plan_recovery_attempt(
                config={},
                events=[],
                phase_id=6,
                turn="review",
                reason="review loop exceeded",
                failure_family="same_phase_loop_exceeded",
                turn_instance_id=None,
            )
        # First attempt walks stage[0]; the loop no longer forces a single action.
        self.assertEqual(request.attempt_action, "start_fresh_session")
        self.assertTrue(request.force_fresh_session)

    def test_terminal_disposition_uses_the_same_authority_event_lane(self) -> None:
        request = RecoveryTurnRequest(
            reason="stall exhausted",
            failure_family="no_edit_stall",
            attempt_action="notify_and_pause",
            exhausted_disposition="notify_and_pause",
        )
        with (
            patch("runner.graph_runtime.recovery_events.append_runtime_event") as recovery_append,
            patch("runner.graph_runtime.recovery_disposition.append_runtime_event") as stop_append,
        ):
            execute_exhausted_recovery_disposition(
                RuntimePaths("recovery-test"), {"phase": 6, "turn": "review"}, request, None
            )
        self.assertEqual([call.args[1] for call in recovery_append.call_args_list], ["recovery_requested"])
        self.assertEqual([call.args[1] for call in stop_append.call_args_list], ["run_stopped"])

    def test_fresh_recovery_records_reset_before_its_request(self) -> None:
        request = RecoveryTurnRequest(
            reason="loop reset",
            failure_family="same_phase_loop_exceeded",
            attempt_action="start_fresh_session",
            force_fresh_session=True,
            session_reset_threshold=4,
            session_reset_cycle_count=4,
        )
        with patch("runner.graph_runtime.recovery_events.append_runtime_event") as append:
            record_recovery_attempt(
                RuntimePaths("recovery-test"),
                {"phase": 6, "turn": "review"},
                request,
                None,
                "recovery-turn",
            )
        self.assertEqual([call.args[1] for call in append.call_args_list], ["session_reset", "recovery_requested"])
        self.assertEqual(append.call_args_list[0].kwargs["payload"]["threshold"], 4)
        self.assertEqual(append.call_args_list[0].kwargs["payload"]["cycle_count"], 4)

    def test_fresh_recovery_appends_the_authoritative_history_in_order(self) -> None:
        request = RecoveryTurnRequest(
            reason="loop reset",
            failure_family="same_phase_loop_exceeded",
            attempt_action="start_fresh_session",
            force_fresh_session=True,
            session_reset_threshold=4,
            session_reset_cycle_count=4,
        )
        with tempfile.TemporaryDirectory() as temp_dir:
            original_root = runtime_paths.CANONICAL_RUNTIME_ROOT
            runtime_paths.CANONICAL_RUNTIME_ROOT = Path(temp_dir) / "runtime"
            try:
                paths = RuntimePaths("recovery-test")
                record_recovery_attempt(paths, {"phase": 6, "turn": "review"}, request, None, "recovery-turn")
                events = load_events(paths.events)
            finally:
                runtime_paths.CANONICAL_RUNTIME_ROOT = original_root
        self.assertEqual([event["event_type"] for event in events], ["session_reset", "recovery_requested"])
        self.assertEqual(events[0]["payload"]["threshold"], 4)
        self.assertEqual(events[1]["payload"]["turn_instance_id"], "recovery-turn")

    def test_git_confirmation_rejects_a_stall_when_a_qualifying_diff_exists(self) -> None:
        completed = type("Completed", (), {"returncode": 0, "stdout": "automation/runner/src/runner/x.py\n"})
        with patch("runner.graph_runtime.qualifying_edits.subprocess.run", return_value=completed):
            self.assertTrue(
                qualifying_git_diff_exists(
                    Path.cwd(), ("automation/runner/**",), ("**/__pycache__/**",)
                )
            )

    def test_unavailable_git_confirmation_cannot_prove_a_stall(self) -> None:
        with patch("runner.graph_runtime.qualifying_edits.subprocess.run", side_effect=OSError):
            self.assertIsNone(
                qualifying_git_diff_exists(Path.cwd(), ("automation/runner/**",), ())
            )

    def test_untracked_qualifying_file_is_progress_not_a_stall(self) -> None:
        tracked = type("Completed", (), {"returncode": 0, "stdout": ""})
        untracked = type("Completed", (), {"returncode": 0, "stdout": "automation/runner/new.py\n"})
        with patch("runner.graph_runtime.qualifying_edits.subprocess.run", side_effect=(tracked, untracked)):
            self.assertTrue(
                qualifying_git_diff_exists(Path.cwd(), ("automation/runner/**",), ())
            )

    def test_open_preflight_fault_blocks_a_second_recovery_family(self) -> None:
        events = [
            {
                "event_type": "runner_fault",
                "phase_id": 6,
                "turn": "review",
                "payload": {"failure_family": "no_edit_stall"},
            }
        ]
        self.assertTrue(open_preflight_fault_exists(events, 6, "review"))
        events.append(
            {
                "event_type": "prompt_selected",
                "phase_id": 6,
                "turn": "review",
                "payload": {"turn_instance_id": "recovery-1"},
            }
        )
        self.assertFalse(open_preflight_fault_exists(events, 6, "review"))

    def test_unprovable_stall_does_not_append_authority(self) -> None:
        config = {"project": {"cwd": str(Path.cwd())}}
        projection = {"current": {"phase": 6, "turn": "review"}}
        policy = StallSignalPolicy("no_edit_stall", True, 20, None)
        edits = QualifyingEditPolicy(("automation/runner/**",), (), "git_diff", "mtime")
        with (
            patch("runner.graph_runtime.recovery_runtime.stall_signal_policy", return_value=policy),
            patch("runner.graph_runtime.recovery_runtime.qualifying_edit_policy", return_value=edits),
            patch("runner.graph_runtime.recovery_runtime.latest_qualifying_edit_timestamp", return_value=None),
            patch("runner.graph_runtime.recovery_runtime.qualifying_git_diff_exists", return_value=None),
            patch("runner.graph_runtime.recovery_runtime.append_runtime_event") as append,
        ):
            admitted = maybe_handle_no_edit_stall(config, RuntimePaths("recovery-test"), projection, [])
        self.assertFalse(admitted)
        append.assert_not_called()

    def test_operator_stop_finishes_without_launching_recovery(self) -> None:
        state = {
            RUN_CONTEXT_KEY: RunContext("recovery-test", Path("config.json"), None),
            RUN_AUTHORITY_KEY: LoadedRunAuthority({}, {"session": {"thread_id": None}}),
            "current_turn_authority": CurrentTurnAuthority(6, "review"),
            TURN_CONTINUATION_KEY: ordinary_turn_continuation(),
            PROMPT_TURN_KEY: PromptTurnDelivery("turn-1", "prompt"),
            TURN_EXECUTION_KEY: TurnExecutionCapture(1, {"thread_id": "thread-1"}),
            TURN_OUTCOME_KEY: PreOutcomeFailure("operator stop requested"),
        }
        with (
            patch("runner.graph_runtime.nodes.authority_event_nodes.stop_requested", return_value=True),
            patch("runner.graph_runtime.nodes.authority_event_nodes.append_runtime_event") as append,
        ):
            result = append_runner_event(state)
        self.assertEqual(result[TURN_TRANSITION_KEY].__class__.__name__, "FinishTurnTransition")
        self.assertEqual([call.args[1] for call in append.call_args_list], ["codex_turn_failed"])

    def test_unhandled_graph_failure_becomes_an_authoritative_recovery_signal(self) -> None:
        active = {
            "completed_at": None, "stopped": False, "current": {"phase": 6, "turn": "review"},
            "session": {"thread_id": "thread-1"},
        }
        stopped = {**active, "stopped": True}
        with (
            patch("runner.graph_runtime.orchestrator.refresh_projection", side_effect=(active, active, stopped)),
            patch("runner.graph_runtime.orchestrator.apply_preflight_runtime_guards", return_value=False),
            patch("runner.graph_runtime.orchestrator.execute_graph_turn", side_effect=RuntimeError("checkpoint lost")),
            patch("runner.graph_runtime.orchestrator.append_runtime_event") as append,
            patch("runner.graph_runtime.orchestrator.time.sleep"),
        ):
            self.assertEqual(drive_graph_run(Path("config.json"), "recovery-test", True, 0, None), 0)
        self.assertEqual(append.call_args.args[1], "runner_fault")
        self.assertEqual(append.call_args.kwargs["payload"]["failure_family"], "provider_crash")


if __name__ == "__main__":
    unittest.main()

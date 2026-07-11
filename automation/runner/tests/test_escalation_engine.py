from __future__ import annotations

import unittest
from unittest.mock import patch

from runner.authority.run_identity import RuntimePaths
from runner.facade.lifecycle import parse_operator_custom_turn
from runner.graph_runtime.continuation.recovery_admission import pending_recovery_reason
from runner.graph_runtime.continuation.requests import RecoveryTurnRequest
from runner.graph_runtime.recovery_disposition import execute_exhausted_recovery_disposition
from runner.generation.scaffold_templates import scaffold_config
from runner.generation.scaffold_types import ScaffoldRequest
from runner.graph_runtime.continuation.recovery_planning import plan_recovery_attempt
from runner.phase_programs.policy_bindings import (
    EscalationFamilyPolicy,
    EscalationStage,
    validate_escalation_policy,
    validate_operator_custom_turn,
)
from runner.roles import apply_model_override, apply_operator_model, resolve_role_policy

CUSTOM_TURN_CONFIG = {
    "operator_custom_turn": {
        "aliases": {
            "codex": {"provider": "codex", "model": "gpt-5.6-sol", "reasoning_effort": "high"},
            "grok": {"provider": "grok", "model": "grok-4.5"},
        },
        "default_alias": "grok",
        "max_ladders_per_phase": 2,
    }
}

OVERSIGHT = {"provider": "codex", "model": "gpt-5.6-sol", "reasoning_effort": "high"}
LADDER = EscalationFamilyPolicy(
    family_name="same_phase_loop_exceeded",
    stages=(
        EscalationStage("start_fresh_session"),
        EscalationStage("override_model", {"turns": ["repair"], "model_policy": OVERSIGHT, "scope": "phase"}),
    ),
    on_exhausted=EscalationStage("notify_and_pause"),
)


def escalation_recovery_event(phase_id: int, turn: str) -> dict:
    return {
        "event_type": "recovery_requested",
        "phase_id": phase_id,
        "turn": turn,
        "payload": {"recovery_kind": "escalation_recovery", "failure_family": "same_phase_loop_exceeded"},
    }


def plan(events: list[dict]):
    with patch(
        "runner.graph_runtime.continuation.recovery_planning.escalation_policy_for_failure_family",
        return_value=LADDER,
    ):
        return plan_recovery_attempt(
            config={},
            events=events,
            phase_id=6,
            turn="review",
            reason="review loop exceeded",
            failure_family="same_phase_loop_exceeded",
            turn_instance_id=None,
        )


class LadderCadenceTests(unittest.TestCase):
    def test_stages_walk_in_order_then_exhaust(self) -> None:
        # 0 priors -> stage 0; 1 prior -> stage 1; 2 priors -> exhausted.
        first = plan([])
        self.assertEqual(first.attempt_action, "start_fresh_session")
        self.assertTrue(first.force_fresh_session)
        self.assertIsNone(first.exhausted_disposition)

        second = plan([escalation_recovery_event(6, "review")])
        self.assertEqual(second.attempt_action, "override_model")
        self.assertTrue(second.force_fresh_session)  # re-arms the loop window
        self.assertEqual(second.attempt_params, {"turns": ["repair"], "model_policy": OVERSIGHT, "scope": "phase"})

        third = plan([escalation_recovery_event(6, "review"), escalation_recovery_event(6, "review")])
        self.assertEqual(third.attempt_action, "notify_and_pause")
        self.assertEqual(third.exhausted_disposition, "notify_and_pause")


class ModelOverrideTests(unittest.TestCase):
    def role_policy(self, turn: str):
        config = scaffold_config(ScaffoldRequest("milestone", "demo", "C:/tmp/demo", "spec.md"))
        return resolve_role_policy(config, 1, turn)

    def projection(self, scope: str = "phase", phase_id: int = 1):
        return {"model_overrides": [{"phase_id": phase_id, "turns": ["repair"], "model_policy": OVERSIGHT, "scope": scope}]}

    def test_override_applies_to_named_turn_in_active_phase(self) -> None:
        overridden = apply_model_override(self.role_policy("repair"), self.projection(), 1, "repair")
        self.assertEqual(overridden.model_policy.provider, "codex")
        self.assertEqual(overridden.model_policy.model, "gpt-5.6-sol")

    def test_untargeted_turn_is_untouched(self) -> None:
        base = self.role_policy("implement")
        same = apply_model_override(base, self.projection(), 1, "implement")
        self.assertEqual(same.model_policy, base.model_policy)

    def test_phase_scope_does_not_leak_to_other_phases(self) -> None:
        base = self.role_policy("repair")
        same = apply_model_override(base, self.projection(scope="phase", phase_id=1), 2, "repair")
        self.assertEqual(same.model_policy, base.model_policy)

    def test_run_scope_applies_everywhere(self) -> None:
        overridden = apply_model_override(self.role_policy("repair"), self.projection(scope="run"), 5, "repair")
        self.assertEqual(overridden.model_policy.model, "gpt-5.6-sol")


class ValidationTests(unittest.TestCase):
    def test_override_model_requires_turns_and_model_policy(self) -> None:
        errors: list[str] = []
        validate_escalation_policy(
            {"same_phase_loop_exceeded": {"stages": [{"action": "override_model"}], "on_exhausted": "notify"}},
            errors,
        )
        self.assertTrue(any("turns" in e for e in errors))
        self.assertTrue(any("model_policy" in e for e in errors))

    def test_unknown_stage_action_is_rejected(self) -> None:
        errors: list[str] = []
        validate_escalation_policy(
            {"provider_crash": {"stages": ["teleport"], "on_exhausted": "notify"}},
            errors,
        )
        self.assertTrue(any("action must be one of" in e for e in errors))

    def test_clean_ladder_validates(self) -> None:
        errors: list[str] = []
        validate_escalation_policy(
            {"same_phase_loop_exceeded": {
                "stages": [
                    "start_fresh_session",
                    {"action": "override_model", "turns": ["repair"], "model_policy": OVERSIGHT, "scope": "phase"},
                ],
                "on_exhausted": {"action": "notify_and_pause"},
            }},
            errors,
        )
        self.assertEqual(errors, [])


def custom_turn_event(phase_id: int, turn: str) -> dict:
    return {
        "event_type": "operator_override",
        "phase_id": phase_id,
        "turn": turn,
        "payload": {"model_policy": {"provider": "grok", "model": "grok-4.5"}},
    }


def plan_with_config(events: list[dict], config: dict):
    with patch(
        "runner.graph_runtime.continuation.recovery_planning.escalation_policy_for_failure_family",
        return_value=LADDER,
    ):
        return plan_recovery_attempt(
            config=config,
            events=events,
            phase_id=6,
            turn="review",
            reason="review loop exceeded",
            failure_family="same_phase_loop_exceeded",
            turn_instance_id=None,
        )


class ResetCapTests(unittest.TestCase):
    def test_custom_turn_resets_the_ladder_to_stage_zero(self) -> None:
        events = [
            escalation_recovery_event(6, "review"),
            escalation_recovery_event(6, "review"),
            custom_turn_event(6, "review"),
        ]
        request = plan_with_config(events, CUSTOM_TURN_CONFIG)
        self.assertEqual(request.attempt_action, "start_fresh_session")

    def test_cap_keeps_the_ladder_exhausted(self) -> None:
        events = [
            escalation_recovery_event(6, "review"),
            escalation_recovery_event(6, "review"),
            custom_turn_event(6, "review"),
            escalation_recovery_event(6, "review"),
            escalation_recovery_event(6, "review"),
            custom_turn_event(6, "review"),  # second custom turn hits max_ladders_per_phase=2
        ]
        request = plan_with_config(events, CUSTOM_TURN_CONFIG)
        self.assertEqual(request.attempt_action, "notify_and_pause")


class AutoResumeTests(unittest.TestCase):
    def disposition_events(self, awaits_operator: bool) -> list[str]:
        request = RecoveryTurnRequest(
            reason="ladder exhausted",
            failure_family="same_phase_loop_exceeded",
            attempt_action="notify_and_pause",
            exhausted_disposition="notify_and_pause",
        )
        with (
            patch("runner.graph_runtime.recovery_events.append_runtime_event"),
            patch("runner.graph_runtime.recovery_disposition.append_runtime_event") as disp_append,
        ):
            execute_exhausted_recovery_disposition(
                RuntimePaths("auto-resume-test"), {"phase": 6, "turn": "review"}, request, None,
                awaits_operator=awaits_operator,
            )
        return [call.args[1] for call in disp_append.call_args_list]

    def test_awaiting_operator_pauses_resumably(self) -> None:
        # With operator custom turns available, exhaustion is a resumable pause.
        self.assertEqual(self.disposition_events(awaits_operator=True), ["operator_pause"])

    def test_without_operator_it_hard_stops(self) -> None:
        self.assertEqual(self.disposition_events(awaits_operator=False), ["run_stopped"])

    def test_operator_override_consumes_the_open_fault(self) -> None:
        fault = {
            "event_type": "runner_fault",
            "phase_id": 6,
            "turn": "review",
            "payload": {"failure_family": "same_phase_loop_exceeded"},
        }
        override = {
            "event_type": "operator_override",
            "phase_id": 6,
            "turn": "review",
            "payload": {"reason": "grok fix it", "model_policy": {"provider": "grok", "model": "grok-4.5"}},
        }
        # Fault alone -> a recovery is pending; an operator override after it -> none.
        self.assertIsNotNone(pending_recovery_reason([fault], {"phase": 6, "turn": "review"}, None))
        self.assertIsNone(pending_recovery_reason([fault, override], {"phase": 6, "turn": "review"}, None))


class OperatorCustomTurnTests(unittest.TestCase):
    def test_named_alias_selects_model_and_keeps_instructions(self) -> None:
        model, instructions = parse_operator_custom_turn(CUSTOM_TURN_CONFIG, "codex fix the import cycle")
        self.assertEqual(model["provider"], "codex")
        self.assertEqual(instructions, "fix the import cycle")

    def test_colon_after_alias_is_accepted(self) -> None:
        model, instructions = parse_operator_custom_turn(CUSTOM_TURN_CONFIG, "grok: retry the lowering")
        self.assertEqual(model["provider"], "grok")
        self.assertEqual(instructions, "retry the lowering")

    def test_no_alias_uses_default_and_full_text(self) -> None:
        model, instructions = parse_operator_custom_turn(CUSTOM_TURN_CONFIG, "just rerun the failing test")
        self.assertEqual(model["provider"], "grok")  # default_alias
        self.assertEqual(instructions, "just rerun the failing test")

    def test_bare_alias_without_instructions_is_rejected(self) -> None:
        with self.assertRaises(ValueError):
            parse_operator_custom_turn(CUSTOM_TURN_CONFIG, "codex")

    def test_no_config_is_plain_injection(self) -> None:
        model, instructions = parse_operator_custom_turn({}, "codex do the thing")
        self.assertIsNone(model)
        self.assertEqual(instructions, "codex do the thing")

    def test_operator_model_applies_only_at_matching_cursor(self) -> None:
        config = scaffold_config(ScaffoldRequest("milestone", "demo", "C:/tmp/demo", "spec.md"))
        base = resolve_role_policy(config, 1, "implement")
        projection = {
            "operator_intervention": {
                "current": {"phase": 1, "turn": "implement"},
                "model_policy": {"provider": "codex", "model": "gpt-5.6-sol", "reasoning_effort": "high"},
            }
        }
        applied = apply_operator_model(base, projection, 1, "implement")
        self.assertEqual(applied.model_policy.provider, "codex")
        # A different cursor is untouched.
        self.assertEqual(apply_operator_model(base, projection, 2, "implement").model_policy, base.model_policy)


class CustomTurnConfigValidationTests(unittest.TestCase):
    def test_default_alias_must_be_declared(self) -> None:
        errors: list[str] = []
        validate_operator_custom_turn({"aliases": {"grok": {"provider": "grok", "model": "grok-4.5"}}, "default_alias": "codex"}, errors)
        self.assertTrue(any("default_alias" in e for e in errors))

    def test_clean_custom_turn_validates(self) -> None:
        errors: list[str] = []
        validate_operator_custom_turn(CUSTOM_TURN_CONFIG["operator_custom_turn"], errors)
        self.assertEqual(errors, [])


if __name__ == "__main__":
    unittest.main()

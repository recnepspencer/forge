from __future__ import annotations

from dataclasses import dataclass
from typing import Any
from runner.recovery.failure_families import (
    MALFORMED_RUNNER_EVENT_FAMILY,
    MISSING_RUNNER_EVENT_FAMILY,
)

SUPPORTED_LOOP_ESCALATION_ACTIONS = {"start_fresh_session"}
SUPPORTED_ESCALATION_ATTEMPTS = {"same_session_recovery", "deep_reviewer_pass", "start_fresh_session"}
SUPPORTED_ESCALATION_EXHAUSTED_ACTIONS = {"notify", "notify_and_pause"}
SUPPORTED_OUTCOME_REPAIR_FIRST_ATTEMPTS = {"same_agent_event_repair_prompt"}
SUPPORTED_OUTCOME_REPAIR_EXHAUSTED_ACTIONS = {"route_to_recovery"}
SUPPORTED_OPERATOR_INJECTION_MODES = {"next_turn_preface"}
SUPPORTED_OPERATOR_POST_INJECTION_ROUTES = {"continue_current_phase"}
BLOCKER_SIGNAL = "blocker"
CRASH_SIGNAL = "crash"
NO_EDIT_STALL_SIGNAL = "no_edit_stall"
RUN_COMPLETED_SIGNAL = "run_completed"
SAME_PHASE_LOOP_SIGNAL = "same_phase_loop_exceeded"
OUTCOME_REPAIR_FAMILIES = {MISSING_RUNNER_EVENT_FAMILY, MALFORMED_RUNNER_EVENT_FAMILY}
INVALID_OUTCOME_ESCALATION_FAMILY = "invalid_outcome"


@dataclass(frozen=True)
class LoopEscalationFamilyPolicy:
    family_name: str
    turns: tuple[str, ...]
    threshold: int
    action: str

    def supports_turn(self, turn: str) -> bool:
        return turn in self.turns


@dataclass(frozen=True)
class EscalationFamilyPolicy:
    family_name: str
    attempts: tuple[str, ...]
    on_exhausted: str


@dataclass(frozen=True)
class OutcomeRepairFamilyPolicy:
    family_name: str
    max_attempts: int
    first_attempt: str
    on_exhausted: str


@dataclass(frozen=True)
class OperatorInterventionPolicy:
    allow_live_injection: bool
    default_injection_mode: str
    allow_immediate_interrupt: bool
    default_post_injection_route: str
    record_as_authority_event: bool


@dataclass(frozen=True)
class StallSignalPolicy:
    signal_name: str
    enabled: bool
    minutes_without_qualifying_edit: int | None
    minutes_without_phase_progress: int | None


@dataclass(frozen=True)
class QualifyingEditPolicy:
    include: tuple[str, ...]
    exclude: tuple[str, ...]
    proof_source: str | None
    early_detector: str | None


@dataclass(frozen=True)
class PhaseProgramPolicyBindings:
    loop_escalation: dict[str, LoopEscalationFamilyPolicy]
    escalation: dict[str, EscalationFamilyPolicy]
    outcome_repair: dict[str, OutcomeRepairFamilyPolicy]
    operator_intervention: OperatorInterventionPolicy
    stall_signals: dict[str, StallSignalPolicy]
    qualifying_edit: QualifyingEditPolicy | None


def validate_loop_escalation(loop_escalation: dict[str, Any], errors: list[str]) -> None:
    families = loop_escalation.get("families")
    if not isinstance(families, dict) or not families:
        errors.append("loop_escalation.families must be a non-empty object")
        return
    for family_name, family in families.items():
        prefix = f"loop_escalation.families.{family_name}"
        if not isinstance(family, dict):
            errors.append(f"{prefix} must be an object")
            continue
        turns = family.get("turns")
        if not isinstance(turns, list) or not turns or any(not isinstance(turn, str) or not turn for turn in turns):
            errors.append(f"{prefix}.turns must be a non-empty list of strings")
        threshold = family.get("threshold")
        if not isinstance(threshold, int) or threshold <= 0:
            errors.append(f"{prefix}.threshold must be a positive integer")
        action = family.get("action")
        if action not in SUPPORTED_LOOP_ESCALATION_ACTIONS:
            errors.append(f"{prefix}.action must be one of {sorted(SUPPORTED_LOOP_ESCALATION_ACTIONS)}")


def validate_escalation_policy(escalation_policy: dict[str, Any], errors: list[str]) -> None:
    for family_name, family in escalation_policy.items():
        prefix = f"escalation_policy.{family_name}"
        if not isinstance(family, dict):
            errors.append(f"{prefix} must be an object")
            continue
        attempts = family.get("attempts")
        if not isinstance(attempts, list) or any(attempt not in SUPPORTED_ESCALATION_ATTEMPTS for attempt in attempts):
            errors.append(f"{prefix}.attempts must be a list drawn from {sorted(SUPPORTED_ESCALATION_ATTEMPTS)}")
        on_exhausted = family.get("on_exhausted")
        if on_exhausted not in SUPPORTED_ESCALATION_EXHAUSTED_ACTIONS:
            errors.append(
                f"{prefix}.on_exhausted must be one of {sorted(SUPPORTED_ESCALATION_EXHAUSTED_ACTIONS)}"
            )


def validate_outcome_repair_policy(outcome_repair_policy: dict[str, Any], errors: list[str]) -> None:
    for family_name, family in outcome_repair_policy.items():
        prefix = f"outcome_repair_policy.{family_name}"
        if not isinstance(family, dict):
            errors.append(f"{prefix} must be an object")
            continue
        max_attempts = family.get("max_attempts")
        if not isinstance(max_attempts, int) or max_attempts < 0:
            errors.append(f"{prefix}.max_attempts must be a non-negative integer")
        first_attempt = family.get("first_attempt")
        if first_attempt not in SUPPORTED_OUTCOME_REPAIR_FIRST_ATTEMPTS:
            errors.append(
                f"{prefix}.first_attempt must be one of {sorted(SUPPORTED_OUTCOME_REPAIR_FIRST_ATTEMPTS)}"
            )
        on_exhausted = family.get("on_exhausted")
        if on_exhausted not in SUPPORTED_OUTCOME_REPAIR_EXHAUSTED_ACTIONS:
            errors.append(
                f"{prefix}.on_exhausted must be one of {sorted(SUPPORTED_OUTCOME_REPAIR_EXHAUSTED_ACTIONS)}"
            )


def validate_operator_intervention_policy(operator_intervention_policy: dict[str, Any], errors: list[str]) -> None:
    for key in ("allow_live_injection", "allow_immediate_interrupt", "record_as_authority_event"):
        if not isinstance(operator_intervention_policy.get(key), bool):
            errors.append(f"operator_intervention_policy.{key} must be a boolean")
    injection_mode = operator_intervention_policy.get("default_injection_mode")
    if injection_mode not in SUPPORTED_OPERATOR_INJECTION_MODES:
        errors.append(
            "operator_intervention_policy.default_injection_mode "
            f"must be one of {sorted(SUPPORTED_OPERATOR_INJECTION_MODES)}"
        )
    post_route = operator_intervention_policy.get("default_post_injection_route")
    if post_route not in SUPPORTED_OPERATOR_POST_INJECTION_ROUTES:
        errors.append(
            "operator_intervention_policy.default_post_injection_route "
            f"must be one of {sorted(SUPPORTED_OPERATOR_POST_INJECTION_ROUTES)}"
        )


def admit_phase_program_policy_bindings(config: dict[str, Any]) -> PhaseProgramPolicyBindings:
    loop_escalation = {
        family_name: LoopEscalationFamilyPolicy(
            family_name=family_name,
            turns=tuple(family["turns"]),
            threshold=family["threshold"],
            action=family["action"],
        )
        for family_name, family in config["loop_escalation"]["families"].items()
    }
    escalation = {
        family_name: EscalationFamilyPolicy(
            family_name=family_name,
            attempts=tuple(family["attempts"]),
            on_exhausted=family["on_exhausted"],
        )
        for family_name, family in config["escalation_policy"].items()
    }
    outcome_repair = {
        family_name: OutcomeRepairFamilyPolicy(
            family_name=family_name,
            max_attempts=family["max_attempts"],
            first_attempt=family["first_attempt"],
            on_exhausted=family["on_exhausted"],
        )
        for family_name, family in config["outcome_repair_policy"].items()
    }
    operator_intervention_config = config["operator_intervention_policy"]
    stall_signals = {
        signal_name: StallSignalPolicy(
            signal_name=signal_name,
            enabled=bool(signal_policy["enabled"]),
            minutes_without_qualifying_edit=signal_policy.get("minutes_without_qualifying_edit"),
            minutes_without_phase_progress=signal_policy.get("minutes_without_phase_progress"),
        )
        for signal_name, signal_policy in config.get("stall_policy", {}).get("signals", {}).items()
    }
    qualifying_edit_config = config.get("qualifying_edit_policy")
    return PhaseProgramPolicyBindings(
        loop_escalation=loop_escalation,
        escalation=escalation,
        outcome_repair=outcome_repair,
        operator_intervention=OperatorInterventionPolicy(
            allow_live_injection=operator_intervention_config["allow_live_injection"],
            default_injection_mode=operator_intervention_config["default_injection_mode"],
            allow_immediate_interrupt=operator_intervention_config["allow_immediate_interrupt"],
            default_post_injection_route=operator_intervention_config["default_post_injection_route"],
            record_as_authority_event=operator_intervention_config["record_as_authority_event"],
        ),
        stall_signals=stall_signals,
        qualifying_edit=None
        if not isinstance(qualifying_edit_config, dict)
        else QualifyingEditPolicy(
            include=tuple(qualifying_edit_config.get("include", ())),
            exclude=tuple(qualifying_edit_config.get("exclude", ())),
            proof_source=qualifying_edit_config.get("proof_source"),
            early_detector=qualifying_edit_config.get("early_detector"),
        ),
    )


def loop_escalation_policy_for_turn(
    config: dict[str, Any],
    continuity_family: str | None,
    turn: str,
) -> LoopEscalationFamilyPolicy | None:
    if not isinstance(continuity_family, str) or not continuity_family:
        return None
    policy = admit_phase_program_policy_bindings(config).loop_escalation.get(continuity_family)
    if policy is None or not policy.supports_turn(turn):
        return None
    return policy


def escalation_policy_for_failure_family(config: dict[str, Any], failure_family: str) -> EscalationFamilyPolicy | None:
    return admit_phase_program_policy_bindings(config).escalation.get(failure_family)


def outcome_repair_policy_for_failure_family(
    config: dict[str, Any],
    failure_family: str | None,
) -> OutcomeRepairFamilyPolicy | None:
    if failure_family not in OUTCOME_REPAIR_FAMILIES:
        return None
    return admit_phase_program_policy_bindings(config).outcome_repair.get(failure_family)


def operator_intervention_policy(config: dict[str, Any]) -> OperatorInterventionPolicy:
    return admit_phase_program_policy_bindings(config).operator_intervention


def stall_signal_policy(config: dict[str, Any], signal_name: str) -> StallSignalPolicy | None:
    return admit_phase_program_policy_bindings(config).stall_signals.get(signal_name)


def qualifying_edit_policy(config: dict[str, Any]) -> QualifyingEditPolicy | None:
    return admit_phase_program_policy_bindings(config).qualifying_edit


def signal_delivery_policy_for_kind(config: dict[str, Any], signal_kind: str) -> str:
    policies = admit_phase_program_policy_bindings(config)
    if signal_kind == RUN_COMPLETED_SIGNAL:
        return "final"
    if signal_kind == BLOCKER_SIGNAL:
        return "immediate" if policies.operator_intervention.allow_immediate_interrupt else "queued"
    if signal_kind == CRASH_SIGNAL:
        if "provider_crash" not in policies.escalation:
            raise ValueError("signal kind 'crash' is not configured in escalation_policy.provider_crash")
        return "immediate"
    if signal_kind == NO_EDIT_STALL_SIGNAL:
        if "no_edit_stall" not in policies.escalation:
            raise ValueError("signal kind 'no_edit_stall' is not configured in escalation_policy.no_edit_stall")
        return "immediate"
    if signal_kind == SAME_PHASE_LOOP_SIGNAL:
        if "same_phase_loop_exceeded" not in policies.escalation:
            raise ValueError(
                "signal kind 'same_phase_loop_exceeded' is not configured in escalation_policy.same_phase_loop_exceeded"
            )
        return "immediate"
    raise ValueError(f"unsupported signal kind {signal_kind!r}")

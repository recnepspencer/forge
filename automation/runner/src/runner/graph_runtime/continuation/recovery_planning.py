from __future__ import annotations

from typing import Any

from runner.graph_runtime.continuation.requests import (
    OutcomeRepairTurnRequest,
    RecoveryTurnRequest,
)
from runner.phase_programs.policy_bindings import (
    INVALID_OUTCOME_ESCALATION_FAMILY,
    escalation_policy_for_failure_family,
    outcome_repair_policy_for_failure_family,
)


def admit_pending_turn_recovery(
    *,
    config: dict[str, Any],
    events: list[dict[str, Any]],
    phase_id: int,
    turn: str,
    pending_reason: str,
    pending_failure_family: str | None,
    turn_instance_id: str | None,
    required_attempt_action: str | None = None,
    session_reset_threshold: int | None = None,
    session_reset_cycle_count: int | None = None,
) -> OutcomeRepairTurnRequest | RecoveryTurnRequest:
    outcome_repair = plan_outcome_repair_attempt(
        config=config,
        events=events,
        phase_id=phase_id,
        turn=turn,
        reason=pending_reason,
        failure_family=pending_failure_family,
        turn_instance_id=turn_instance_id,
    )
    if outcome_repair is not None:
        return outcome_repair
    return plan_recovery_attempt(
        config=config,
        events=events,
        phase_id=phase_id,
        turn=turn,
        reason=pending_reason,
        failure_family=broader_recovery_family(pending_failure_family),
        turn_instance_id=turn_instance_id,
        required_attempt_action=required_attempt_action,
        session_reset_threshold=session_reset_threshold,
        session_reset_cycle_count=session_reset_cycle_count,
    )


def plan_outcome_repair_attempt(
    *,
    config: dict[str, Any],
    events: list[dict[str, Any]],
    phase_id: int,
    turn: str,
    reason: str,
    failure_family: str | None,
    turn_instance_id: str | None,
) -> OutcomeRepairTurnRequest | None:
    policy = outcome_repair_policy_for_failure_family(config, failure_family)
    if policy is None:
        return None
    prior_attempts = count_recovery_requests(
        events,
        phase_id=phase_id,
        turn=turn,
        turn_instance_id=turn_instance_id,
        recovery_kind="outcome_repair",
    )
    if prior_attempts >= policy.max_attempts:
        return None
    return OutcomeRepairTurnRequest(
        reason=reason,
        failure_family=policy.family_name,
        turn_instance_id=turn_instance_id,
        attempt_index=prior_attempts + 1,
        attempt_action=policy.first_attempt,
    )


def plan_recovery_attempt(
    *,
    config: dict[str, Any],
    events: list[dict[str, Any]],
    phase_id: int,
    turn: str,
    reason: str,
    failure_family: str | None,
    turn_instance_id: str | None,
    required_attempt_action: str | None = None,
    session_reset_threshold: int | None = None,
    session_reset_cycle_count: int | None = None,
) -> RecoveryTurnRequest:
    if failure_family is None:
        failure_family = "provider_crash"
    policy = escalation_policy_for_failure_family(config, failure_family)
    if policy is None:
        raise ValueError(f"missing escalation_policy entry for failure family {failure_family!r}")
    prior_attempts = count_recovery_requests(
        events,
        phase_id=phase_id,
        turn=turn,
        turn_instance_id=turn_instance_id,
        recovery_kind="escalation_recovery",
    )
    if required_attempt_action is not None and required_attempt_action not in policy.attempts:
        raise ValueError(
            f"recovery action {required_attempt_action!r} is not admitted for failure family {failure_family!r}"
        )
    if prior_attempts >= len(policy.attempts):
        return RecoveryTurnRequest(
            reason=(
                f"recovery attempts exhausted for failure family {failure_family!r}; "
                f"configured on_exhausted={policy.on_exhausted!r}"
            ),
            failure_family=failure_family,
            turn_instance_id=turn_instance_id,
            attempt_index=prior_attempts + 1,
            attempt_action=policy.on_exhausted,
            exhausted_disposition=policy.on_exhausted,
            session_reset_threshold=session_reset_threshold,
            session_reset_cycle_count=session_reset_cycle_count,
        )
    attempt_action = required_attempt_action or policy.attempts[prior_attempts]
    role_route = "projection"
    force_fresh_session = False
    if attempt_action == "start_fresh_session":
        role_route = "current_turn"
        force_fresh_session = True
    elif attempt_action == "deep_reviewer_pass":
        role_route = "reviewer"
        force_fresh_session = True
    return RecoveryTurnRequest(
        reason=reason,
        failure_family=failure_family,
        turn_instance_id=turn_instance_id,
        attempt_index=prior_attempts + 1,
        attempt_action=attempt_action,
        role_route=role_route,
        force_fresh_session=force_fresh_session,
        session_reset_threshold=session_reset_threshold,
        session_reset_cycle_count=session_reset_cycle_count,
    )


def broader_recovery_family(failure_family: str | None) -> str | None:
    if failure_family in {"missing_runner_event", "malformed_runner_event"}:
        return INVALID_OUTCOME_ESCALATION_FAMILY
    return failure_family


def count_recovery_requests(
    events: list[dict[str, Any]],
    *,
    phase_id: int,
    turn: str,
    turn_instance_id: str | None,
    recovery_kind: str,
) -> int:
    count = 0
    for event in events:
        if event.get("event_type") != "recovery_requested":
            continue
        if event.get("phase_id") != phase_id or event.get("turn") != turn:
            continue
        payload = event.get("payload", {})
        if payload.get("recovery_kind") != recovery_kind:
            continue
        if turn_instance_id is not None and payload.get("turn_instance_id") != turn_instance_id:
            continue
        count += 1
    return count

from __future__ import annotations

from typing import Any

from runner.graph_runtime.continuation.requests import (
    OutcomeRepairTurnRequest,
    RecoveryTurnRequest,
)
from runner.phase_programs.policy_bindings import (
    INVALID_OUTCOME_ESCALATION_FAMILY,
    escalation_policy_for_failure_family,
    operator_custom_turn_config,
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
    session_reset_threshold: int | None = None,
    session_reset_cycle_count: int | None = None,
) -> RecoveryTurnRequest:
    if failure_family is None:
        failure_family = "provider_crash"
    policy = escalation_policy_for_failure_family(config, failure_family)
    if policy is None:
        raise ValueError(f"missing escalation_policy entry for failure family {failure_family!r}")
    prior_attempts = escalation_prior_attempts(events, config, phase_id, turn, turn_instance_id)
    # The stage ladder governs which action fires; a loop trigger no longer
    # forces a single action, so distinct stages actually progress.
    stage = policy.stage_for_attempt(prior_attempts)
    if stage is None:
        return RecoveryTurnRequest(
            reason=(
                f"Escalation ladder exhausted for {failure_family} at phase {phase_id} turn {turn!r}. "
                f"The run is paused (on_exhausted={policy.on_exhausted.action}). Reply to this alert with a "
                f"model and instructions to run a custom turn (e.g. 'codex <what to do>' or 'grok <what to do>'), "
                f"then the standard runner resumes."
            ),
            failure_family=failure_family,
            turn_instance_id=turn_instance_id,
            attempt_index=prior_attempts + 1,
            attempt_action=policy.on_exhausted.action,
            exhausted_disposition=policy.on_exhausted.action,
            session_reset_threshold=session_reset_threshold,
            session_reset_cycle_count=session_reset_cycle_count,
        )
    role_route = "projection"
    force_fresh_session = False
    if stage.action in ("start_fresh_session", "override_model"):
        role_route = "current_turn"
        force_fresh_session = True
    elif stage.action == "deep_reviewer_pass":
        role_route = "reviewer"
        force_fresh_session = True
    return RecoveryTurnRequest(
        reason=reason,
        failure_family=failure_family,
        turn_instance_id=turn_instance_id,
        attempt_index=prior_attempts + 1,
        attempt_action=stage.action,
        role_route=role_route,
        force_fresh_session=force_fresh_session,
        session_reset_threshold=session_reset_threshold,
        session_reset_cycle_count=session_reset_cycle_count,
        attempt_params=dict(stage.params) if stage.params else None,
    )


def broader_recovery_family(failure_family: str | None) -> str | None:
    if failure_family in {"missing_runner_event", "malformed_runner_event"}:
        return INVALID_OUTCOME_ESCALATION_FAMILY
    return failure_family


def escalation_prior_attempts(
    events: list[dict[str, Any]],
    config: dict[str, Any],
    phase_id: int,
    turn: str,
    turn_instance_id: str | None,
) -> int:
    """Escalation attempts consumed so far, honouring the operator-custom-turn
    reset with a hard per-phase cap. Each custom turn resets the ladder window
    so the phase gets a clean run, but once the phase has consumed
    max_ladders_per_phase custom turns the count stays cumulative, so the ladder
    re-exhausts immediately and the run stays paged and paused."""
    custom = operator_custom_turn_config(config)
    max_ladders = custom.get("max_ladders_per_phase") if isinstance(custom, dict) else None
    ladders_used = count_operator_custom_turns(events, phase_id)
    if not isinstance(max_ladders, int) or ladders_used == 0 or ladders_used >= max_ladders:
        return count_recovery_requests(
            events,
            phase_id=phase_id,
            turn=turn,
            turn_instance_id=turn_instance_id,
            recovery_kind="escalation_recovery",
        )
    return count_escalation_since_last_custom_turn(events, phase_id, turn)


def count_operator_custom_turns(events: list[dict[str, Any]], phase_id: int) -> int:
    return sum(
        1
        for event in events
        if event.get("event_type") == "operator_override"
        and event.get("phase_id") == phase_id
        and event.get("payload", {}).get("model_policy")
    )


def count_escalation_since_last_custom_turn(events: list[dict[str, Any]], phase_id: int, turn: str) -> int:
    count = 0
    for event in reversed(events):
        if event.get("phase_id") != phase_id:
            continue
        event_type = event.get("event_type")
        if event_type == "operator_override" and event.get("payload", {}).get("model_policy"):
            break
        if (
            event_type == "recovery_requested"
            and event.get("turn") == turn
            and event.get("payload", {}).get("recovery_kind") == "escalation_recovery"
        ):
            count += 1
    return count


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

from __future__ import annotations

from dataclasses import dataclass

TURN_CONTINUATION_KEY = "turn_continuation"


@dataclass(frozen=True)
class OutcomeRepairTurnRequest:
    reason: str
    failure_family: str
    turn_instance_id: str | None = None
    attempt_index: int = 1
    attempt_action: str = "same_agent_event_repair_prompt"


@dataclass(frozen=True)
class RecoveryTurnRequest:
    reason: str
    failure_family: str | None = None
    turn_instance_id: str | None = None
    attempt_index: int = 1
    attempt_action: str = "same_session_recovery"
    role_route: str = "projection"
    force_fresh_session: bool = False
    exhausted_disposition: str | None = None


@dataclass(frozen=True)
class OrdinaryTurnContinuation:
    mode: str = "ordinary"


@dataclass(frozen=True)
class OutcomeRepairTurnContinuation:
    outcome_repair: OutcomeRepairTurnRequest
    mode: str = "outcome_repair"


@dataclass(frozen=True)
class RecoveryTurnContinuation:
    recovery: RecoveryTurnRequest
    mode: str = "recovery"


TurnContinuation = OrdinaryTurnContinuation | OutcomeRepairTurnContinuation | RecoveryTurnContinuation


def ordinary_turn_continuation() -> OrdinaryTurnContinuation:
    return OrdinaryTurnContinuation()


def outcome_repair_turn_continuation(
    outcome_repair: OutcomeRepairTurnRequest,
) -> OutcomeRepairTurnContinuation:
    return OutcomeRepairTurnContinuation(outcome_repair=outcome_repair)


def recovery_turn_continuation(recovery: RecoveryTurnRequest) -> RecoveryTurnContinuation:
    return RecoveryTurnContinuation(recovery=recovery)


def is_outcome_repair_continuation(continuation: TurnContinuation) -> bool:
    return isinstance(continuation, OutcomeRepairTurnContinuation)


def is_recovery_continuation(continuation: TurnContinuation) -> bool:
    return isinstance(continuation, RecoveryTurnContinuation)


def continuation_outcome_repair(
    continuation: TurnContinuation,
) -> OutcomeRepairTurnRequest | None:
    if isinstance(continuation, OutcomeRepairTurnContinuation):
        return continuation.outcome_repair
    return None


def continuation_recovery(continuation: TurnContinuation) -> RecoveryTurnRequest | None:
    if isinstance(continuation, RecoveryTurnContinuation):
        return continuation.recovery
    return None

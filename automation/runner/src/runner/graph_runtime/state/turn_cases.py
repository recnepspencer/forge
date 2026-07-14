from __future__ import annotations

from dataclasses import dataclass
from typing import Any

from runner.recovery.failure_families import (
    MALFORMED_RUNNER_EVENT_FAMILY,
    MISSING_RUNNER_EVENT_FAMILY,
    PROVIDER_CRASH_FAMILY,
)

FINISH_TRANSITION = "finish"
MISSING_EVENT_REPAIR_TRANSITION = "repair_missing_runner_event"
MALFORMED_EVENT_REPAIR_TRANSITION = "repair_malformed_runner_event"
PRE_OUTCOME_FAILURE_RECOVERY_TRANSITION = "recover_pre_outcome_failure"
RELOAD_CURRENT_TURN_TRANSITION = "reload_current_turn"

RECOVERY_TRANSITIONS = frozenset(
    {
        MISSING_EVENT_REPAIR_TRANSITION,
        MALFORMED_EVENT_REPAIR_TRANSITION,
        PRE_OUTCOME_FAILURE_RECOVERY_TRANSITION,
    }
)


@dataclass(frozen=True)
class ValidTurnOutcome:
    outcome: dict[str, Any]


@dataclass(frozen=True)
class MissingRunnerEventOutcome:
    reason: str
    failure_family: str = MISSING_RUNNER_EVENT_FAMILY


@dataclass(frozen=True)
class MalformedRunnerEventOutcome:
    reason: str
    failure_family: str = MALFORMED_RUNNER_EVENT_FAMILY


@dataclass(frozen=True)
class PreOutcomeFailure:
    reason: str
    failure_family: str | None = PROVIDER_CRASH_FAMILY


TurnOutcomeCase = (
    ValidTurnOutcome
    | MissingRunnerEventOutcome
    | MalformedRunnerEventOutcome
    | PreOutcomeFailure
)


@dataclass(frozen=True)
class FinishTurnTransition:
    result_code: int = 0
    projection_updated: bool = False


@dataclass(frozen=True)
class RepairMissingRunnerEventTransition:
    result_code: int = 0
    projection_updated: bool = False


@dataclass(frozen=True)
class RepairMalformedRunnerEventTransition:
    result_code: int = 0
    projection_updated: bool = False


@dataclass(frozen=True)
class RecoverPreOutcomeFailureTransition:
    result_code: int = 0
    projection_updated: bool = False


@dataclass(frozen=True)
class ReloadCurrentTurnTransition:
    result_code: int = 0
    projection_updated: bool = True


TurnTransitionCase = (
    FinishTurnTransition
    | RepairMissingRunnerEventTransition
    | RepairMalformedRunnerEventTransition
    | RecoverPreOutcomeFailureTransition
    | ReloadCurrentTurnTransition
)


def transition_name(transition: TurnTransitionCase) -> str:
    if isinstance(transition, FinishTurnTransition):
        return FINISH_TRANSITION
    if isinstance(transition, RepairMissingRunnerEventTransition):
        return MISSING_EVENT_REPAIR_TRANSITION
    if isinstance(transition, RepairMalformedRunnerEventTransition):
        return MALFORMED_EVENT_REPAIR_TRANSITION
    if isinstance(transition, ReloadCurrentTurnTransition):
        return RELOAD_CURRENT_TURN_TRANSITION
    return PRE_OUTCOME_FAILURE_RECOVERY_TRANSITION


def transition_requires_recovery(transition: TurnTransitionCase) -> bool:
    return transition_name(transition) in RECOVERY_TRANSITIONS

from __future__ import annotations

from dataclasses import dataclass
from typing import Any

from runner.graph_runtime.continuation.requests import (
    TurnContinuation,
)
from runner.graph_runtime.outcome_parsing import (
    MalformedRunnerEventError,
    MissingRunnerEventError,
    extract_runner_event,
)
from runner.graph_runtime.state.turn_cases import (
    FinishTurnTransition,
    MalformedRunnerEventOutcome,
    MissingRunnerEventOutcome,
    PreOutcomeFailure,
    RecoverPreOutcomeFailureTransition,
    RepairMalformedRunnerEventTransition,
    RepairMissingRunnerEventTransition,
    TurnOutcomeCase,
    TurnTransitionCase,
    ValidTurnOutcome,
)
from runner.recovery.failure_families import (
    classify_pre_outcome_failure_family,
    MALFORMED_RUNNER_EVENT_FAMILY,
    MISSING_RUNNER_EVENT_FAMILY,
)


@dataclass(frozen=True)
class OutcomeRoutingDecision:
    transition: TurnTransitionCase
    continuation: TurnContinuation


def classify_outcome_route(
    *,
    exit_code: int,
    capture: dict[str, Any],
    expected_turn_instance_id: str | None,
) -> TurnOutcomeCase:
    if exit_code != 0:
        return PreOutcomeFailure(
            reason=capture.get("failure_reason") or f"agent exited with {exit_code}",
            failure_family=classify_pre_outcome_failure_family(capture),
        )
    try:
        return ValidTurnOutcome(
            outcome=extract_runner_event(
                capture.get("agent_messages", []),
                expected_turn_instance_id,
            )
        )
    except MissingRunnerEventError as error:
        return MissingRunnerEventOutcome(
            reason=str(error),
            failure_family=error.failure_family or MISSING_RUNNER_EVENT_FAMILY,
        )
    except MalformedRunnerEventError as error:
        return MalformedRunnerEventOutcome(
            reason=str(error),
            failure_family=error.failure_family or MALFORMED_RUNNER_EVENT_FAMILY,
        )


def decide_outcome_routing(
    continuation: TurnContinuation,
    outcome: TurnOutcomeCase,
    turn_instance_id: str | None,
) -> OutcomeRoutingDecision:
    del turn_instance_id
    if isinstance(outcome, ValidTurnOutcome):
        return OutcomeRoutingDecision(
            transition=FinishTurnTransition(),
            continuation=continuation,
        )
    return OutcomeRoutingDecision(
        transition=_transition_for_failure(outcome),
        continuation=continuation,
    )


def _transition_for_failure(outcome: TurnOutcomeCase) -> TurnTransitionCase:
    if isinstance(outcome, MissingRunnerEventOutcome):
        return RepairMissingRunnerEventTransition()
    if isinstance(outcome, MalformedRunnerEventOutcome):
        return RepairMalformedRunnerEventTransition()
    if isinstance(outcome, PreOutcomeFailure):
        return RecoverPreOutcomeFailureTransition()
    raise TypeError(f"unsupported turn outcome case {type(outcome)!r}")

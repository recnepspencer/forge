from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
from typing import Any, TypedDict

from runner.graph_runtime.authority import CURRENT_TURN_AUTHORITY_KEY, CurrentTurnAuthority, LoadedRunAuthority
from runner.graph_runtime.continuation.requests import (
    TURN_CONTINUATION_KEY,
    TurnContinuation,
    ordinary_turn_continuation,
)
from runner.graph_runtime.state.turn_cases import FinishTurnTransition, TurnOutcomeCase, TurnTransitionCase

RUN_CONTEXT_KEY = "run_context"
RUN_AUTHORITY_KEY = "run_authority"
LOWERED_PHASE_PROGRAM_KEY = "lowered_phase_program"
ROLE_SESSION_KEY = "role_session"
PROMPT_TURN_KEY = "prompt_turn"
TURN_EXECUTION_KEY = "turn_execution"
TURN_OUTCOME_KEY = "turn_outcome"
TURN_TRANSITION_KEY = "turn_transition"


@dataclass(frozen=True)
class RunContext:
    run_id: str
    config_path: Path
    log_path: Path | None


@dataclass(frozen=True)
class LoweredGraphPhaseProgram:
    phase_id: int
    turn: str
    program_id: str
    prompt_binding_mode: str
    prompt_topology_id: str
    supported_outcomes: frozenset[str]


@dataclass(frozen=True)
class RoleSessionSelection:
    role_policy: Any | None


@dataclass(frozen=True)
class PromptTurnDelivery:
    turn_instance_id: str | None
    delivery_prompt: str


@dataclass(frozen=True)
class TurnExecutionCapture:
    exit_code: int
    capture: dict[str, Any]


class GraphState(TypedDict, total=False):
    run_context: RunContext
    run_authority: LoadedRunAuthority
    current_turn_authority: CurrentTurnAuthority
    lowered_phase_program: LoweredGraphPhaseProgram
    role_session: RoleSessionSelection
    turn_continuation: TurnContinuation
    prompt_turn: PromptTurnDelivery
    turn_execution: TurnExecutionCapture
    turn_outcome: TurnOutcomeCase
    turn_transition: TurnTransitionCase


def build_graph_state(
    *,
    run_id: str,
    config_path: Path,
    log_path: Path | None,
) -> GraphState:
    return {
        RUN_CONTEXT_KEY: RunContext(run_id=run_id, config_path=config_path, log_path=log_path),
        TURN_CONTINUATION_KEY: ordinary_turn_continuation(),
        TURN_TRANSITION_KEY: FinishTurnTransition(),
    }

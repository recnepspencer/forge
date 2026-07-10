from __future__ import annotations

from dataclasses import dataclass, replace
from typing import Any, Callable

PHASE_ASSET_PROMPT_BINDING = "phase_asset"
TURN_ASSEMBLY_PROMPT_BINDING = "turn_assembly"
PhaseProgressApplier = Callable[[dict[str, Any], dict[str, Any], int, dict[str, Any]], None]


@dataclass(frozen=True)
class LoweredPhaseProgram:
    program_id: str
    first_turn: str
    role_binding_turns: tuple[str, ...]
    required_turn_template_turns: tuple[str, ...]
    supported_outcomes_by_turn: dict[str, frozenset[str]]
    prompt_binding_mode_by_turn: dict[str, str]
    apply_phase_progress: PhaseProgressApplier

    def supports_turn(self, turn: str) -> bool:
        return turn in self.supported_outcomes_by_turn

    def supported_outcomes_for_turn(self, turn: str) -> frozenset[str]:
        outcomes = self.supported_outcomes_by_turn.get(turn)
        if outcomes is None:
            raise ValueError(f"program {self.program_id!r} does not support turn {turn!r}")
        return outcomes

    def prompt_binding_mode_for_turn(self, turn: str) -> str:
        binding_mode = self.prompt_binding_mode_by_turn.get(turn)
        if binding_mode is None:
            raise ValueError(f"program {self.program_id!r} does not define prompt binding mode for turn {turn!r}")
        return binding_mode

    def recognizes_event_type(self, event_type: str) -> bool:
        return any(event_type in outcomes for outcomes in self.supported_outcomes_by_turn.values())

    def without_turn(self, turn: str, *, first_turn: str | None = None) -> "LoweredPhaseProgram":
        return replace(
            self,
            first_turn=first_turn or self.first_turn,
            role_binding_turns=tuple(existing_turn for existing_turn in self.role_binding_turns if existing_turn != turn),
            required_turn_template_turns=tuple(
                existing_turn for existing_turn in self.required_turn_template_turns if existing_turn != turn
            ),
            supported_outcomes_by_turn={
                existing_turn: outcomes
                for existing_turn, outcomes in self.supported_outcomes_by_turn.items()
                if existing_turn != turn
            },
            prompt_binding_mode_by_turn={
                existing_turn: binding_mode
                for existing_turn, binding_mode in self.prompt_binding_mode_by_turn.items()
                if existing_turn != turn
            },
        )

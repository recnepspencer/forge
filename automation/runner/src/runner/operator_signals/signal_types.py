from __future__ import annotations

from dataclasses import dataclass
from typing import Any

BLOCKER_SIGNAL = "blocker"
CRASH_SIGNAL = "crash"
NO_EDIT_STALL_SIGNAL = "no_edit_stall"
RUN_COMPLETED_SIGNAL = "run_completed"
SAME_PHASE_LOOP_SIGNAL = "same_phase_loop_exceeded"
INVALID_OUTCOME_SIGNAL = "invalid_outcome"
WALL_TIMEOUT_SIGNAL = "wall_timeout"
IDLE_TIMEOUT_SIGNAL = "idle_timeout"
COMPLETION_HANDOFF_FAILED_SIGNAL = "completion_handoff_failed"

SIGNAL_KINDS = (
    BLOCKER_SIGNAL,
    CRASH_SIGNAL,
    NO_EDIT_STALL_SIGNAL,
    SAME_PHASE_LOOP_SIGNAL,
    RUN_COMPLETED_SIGNAL,
    INVALID_OUTCOME_SIGNAL,
    WALL_TIMEOUT_SIGNAL,
    IDLE_TIMEOUT_SIGNAL,
    COMPLETION_HANDOFF_FAILED_SIGNAL,
)


@dataclass(frozen=True)
class CanonicalSignal:
    signal_id: str
    signal_kind: str
    source_sequence: int
    run_id: str
    phase_id: int | None
    turn: str | None
    summary: str
    details: dict[str, Any]

    def payload(self, delivery: str) -> dict[str, Any]:
        return {
            "signal_id": self.signal_id, "signal_kind": self.signal_kind, "delivery": delivery,
            "run_id": self.run_id, "phase_id": self.phase_id, "turn": self.turn,
            "summary": self.summary, "details": self.details,
        }

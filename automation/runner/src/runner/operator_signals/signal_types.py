from __future__ import annotations

BLOCKER_SIGNAL = "blocker"
CRASH_SIGNAL = "crash"
NO_EDIT_STALL_SIGNAL = "no_edit_stall"
RUN_COMPLETED_SIGNAL = "run_completed"
SAME_PHASE_LOOP_SIGNAL = "same_phase_loop_exceeded"

SIGNAL_KINDS = (
    BLOCKER_SIGNAL,
    CRASH_SIGNAL,
    NO_EDIT_STALL_SIGNAL,
    SAME_PHASE_LOOP_SIGNAL,
    RUN_COMPLETED_SIGNAL,
)

from __future__ import annotations

from runner.phase_programs.lowered_program import LoweredPhaseProgram, TURN_ASSEMBLY_PROMPT_BINDING
from runner.phase_programs.program_ids import IMPLEMENT_REVIEW_LOOP_PROGRAM_ID

IMPLEMENT_REVIEW_LOOP_TURNS = ("plan", "implement", "review", "repair")


def apply_implement_review_progress(
    projection: dict[str, object],
    config: dict[str, object],
    phase_id: int,
    event: dict[str, object],
) -> None:
    from runner.phase_programs.transition_rules import advance_after_phase_close, phase_by_id

    phase = phase_by_id(projection, phase_id)
    event_type = event["event_type"]
    if event_type == "implementation_completed":
        phase["status"] = "complete"
        phase["qa_status"] = "needed"
        projection["current"] = {"phase": phase_id, "turn": "review"}
        return
    if event_type == "review_failed":
        phase["status"] = "regressed"
        phase["qa_status"] = "failed"
        projection["current"] = {"phase": phase_id, "turn": "repair"}
        return
    if event_type == "repair_completed":
        phase["status"] = "complete"
        phase["qa_status"] = "needed"
        projection["current"] = {"phase": phase_id, "turn": "review"}
        return
    if event_type == "review_passed":
        phase["status"] = "complete"
        phase["qa_status"] = "passed"
        advance_after_phase_close(projection, config, phase_id)


PROGRAM_DEFINITION = LoweredPhaseProgram(
    program_id=IMPLEMENT_REVIEW_LOOP_PROGRAM_ID,
    first_turn="plan",
    role_binding_turns=IMPLEMENT_REVIEW_LOOP_TURNS,
    required_turn_template_turns=IMPLEMENT_REVIEW_LOOP_TURNS,
    supported_outcomes_by_turn={
        "plan": frozenset({"plan_posted"}),
        "implement": frozenset({"implementation_completed"}),
        "review": frozenset({"review_failed", "review_passed"}),
        "repair": frozenset({"repair_completed"}),
    },
    prompt_binding_mode_by_turn={turn: TURN_ASSEMBLY_PROMPT_BINDING for turn in IMPLEMENT_REVIEW_LOOP_TURNS},
    apply_phase_progress=apply_implement_review_progress,
)

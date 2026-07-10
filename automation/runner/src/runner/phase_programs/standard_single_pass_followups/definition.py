from __future__ import annotations

from runner.phase_programs.lowered_program import LoweredPhaseProgram, TURN_ASSEMBLY_PROMPT_BINDING
from runner.phase_programs.program_ids import STANDARD_SINGLE_PASS_FOLLOWUPS_PROGRAM_ID

STANDARD_SINGLE_PASS_FOLLOWUPS_TURNS = (
    "boundary_review",
    "plan",
    "implement",
    "review",
    "repair",
    "test_review",
    "test_repair_implement",
    "code_quality_review",
    "code_quality_repair",
)


def apply_single_pass_followups_progress(
    projection: dict[str, object],
    config: dict[str, object],
    phase_id: int,
    event: dict[str, object],
) -> None:
    from runner.phase_programs.standard_loop.definition import apply_standard_loop_progress
    from runner.phase_programs.transition_rules import advance_after_phase_close, phase_by_id

    phase = phase_by_id(projection, phase_id)
    if event["event_type"] == "code_quality_repair_completed":
        phase["status"] = "complete"
        phase["qa_status"] = "passed"
        advance_after_phase_close(projection, config, phase_id)
        return
    apply_standard_loop_progress(projection, config, phase_id, event)


PROGRAM_DEFINITION = LoweredPhaseProgram(
    program_id=STANDARD_SINGLE_PASS_FOLLOWUPS_PROGRAM_ID,
    first_turn="boundary_review",
    role_binding_turns=STANDARD_SINGLE_PASS_FOLLOWUPS_TURNS,
    required_turn_template_turns=STANDARD_SINGLE_PASS_FOLLOWUPS_TURNS,
    supported_outcomes_by_turn={
        "boundary_review": frozenset({"boundary_review_completed"}),
        "plan": frozenset({"plan_posted"}),
        "implement": frozenset({"implementation_completed"}),
        "review": frozenset({"review_failed", "review_passed"}),
        "repair": frozenset({"repair_completed"}),
        "test_review": frozenset({"test_review_failed", "test_review_passed"}),
        "test_repair_implement": frozenset({"test_repair_completed"}),
        "code_quality_review": frozenset({"code_quality_review_failed", "code_quality_review_passed"}),
        "code_quality_repair": frozenset({"code_quality_repair_completed"}),
    },
    prompt_binding_mode_by_turn={turn: TURN_ASSEMBLY_PROMPT_BINDING for turn in STANDARD_SINGLE_PASS_FOLLOWUPS_TURNS},
    apply_phase_progress=apply_single_pass_followups_progress,
)

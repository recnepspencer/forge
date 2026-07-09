from __future__ import annotations

from runner.phase_programs.lowered_program import LoweredPhaseProgram, TURN_ASSEMBLY_PROMPT_BINDING
from runner.phase_programs.program_ids import STANDARD_LOOP_PROGRAM_ID

STANDARD_LOOP_TURNS = (
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


def apply_standard_loop_progress(
    projection: dict[str, object],
    config: dict[str, object],
    phase_id: int,
    event: dict[str, object],
) -> None:
    from runner.phase_programs.transition_rules import advance_after_phase_close, phase_by_id

    phase = phase_by_id(projection, phase_id)
    event_type = event["event_type"]
    if event_type == "boundary_review_completed":
        phase["status"] = "not_started"
        phase["qa_status"] = "not_started"
        projection["current"] = {"phase": phase_id, "turn": "plan"}
        return
    if event_type == "plan_posted":
        phase["status"] = "in_progress"
        phase["qa_status"] = "not_started"
        projection["current"] = {"phase": phase_id, "turn": "implement"}
        return
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
        phase["qa_status"] = "needed"
        projection["current"] = {"phase": phase_id, "turn": "test_review"}
        return
    if event_type == "test_review_failed":
        phase["status"] = "regressed"
        phase["qa_status"] = "failed"
        projection["current"] = {"phase": phase_id, "turn": "test_repair_implement"}
        return
    if event_type == "test_review_passed":
        phase["status"] = "complete"
        phase["qa_status"] = "needed"
        projection["current"] = {"phase": phase_id, "turn": "code_quality_review"}
        return
    if event_type == "test_repair_completed":
        phase["status"] = "complete"
        phase["qa_status"] = "needed"
        projection["current"] = {"phase": phase_id, "turn": "code_quality_review"}
        return
    if event_type == "code_quality_review_failed":
        phase["status"] = "regressed"
        phase["qa_status"] = "failed"
        projection["current"] = {"phase": phase_id, "turn": "code_quality_repair"}
        return
    if event_type == "code_quality_repair_completed":
        phase["status"] = "complete"
        phase["qa_status"] = "needed"
        projection["current"] = {"phase": phase_id, "turn": "code_quality_review"}
        return
    if event_type == "code_quality_review_passed":
        phase["status"] = "complete"
        phase["qa_status"] = "passed"
        advance_after_phase_close(projection, config, phase_id)


PROGRAM_DEFINITION = LoweredPhaseProgram(
    program_id=STANDARD_LOOP_PROGRAM_ID,
    first_turn="boundary_review",
    role_binding_turns=STANDARD_LOOP_TURNS,
    required_turn_template_turns=STANDARD_LOOP_TURNS,
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
    prompt_binding_mode_by_turn={turn: TURN_ASSEMBLY_PROMPT_BINDING for turn in STANDARD_LOOP_TURNS},
    apply_phase_progress=apply_standard_loop_progress,
)

from __future__ import annotations

from runner.phase_programs.lowered_program import LoweredPhaseProgram, PHASE_ASSET_PROMPT_BINDING
from runner.phase_programs.program_ids import SINGLE_PROMPT_PROGRAM_ID


def apply_single_prompt_progress(
    projection: dict[str, object],
    config: dict[str, object],
    phase_id: int,
    event: dict[str, object],
) -> None:
    from runner.phase_programs.transition_rules import advance_after_phase_close, phase_by_id

    if event["event_type"] != "single_prompt_completed":
        return
    phase = phase_by_id(projection, phase_id)
    phase["status"] = "complete"
    phase["qa_status"] = "passed"
    advance_after_phase_close(projection, config, phase_id)


PROGRAM_DEFINITION = LoweredPhaseProgram(
    program_id=SINGLE_PROMPT_PROGRAM_ID,
    first_turn="single_prompt",
    role_binding_turns=("single_prompt",),
    required_turn_template_turns=(),
    supported_outcomes_by_turn={"single_prompt": frozenset({"single_prompt_completed"})},
    prompt_binding_mode_by_turn={"single_prompt": PHASE_ASSET_PROMPT_BINDING},
    apply_phase_progress=apply_single_prompt_progress,
)

from __future__ import annotations

from runner.authority.config import load_config
from runner.authority.events import load_events
from runner.authority.run_identity import RuntimePaths
from runner.graph_runtime.authority import (
    CURRENT_TURN_AUTHORITY_KEY,
    LoadedRunAuthority,
    current_turn_authority_from_state,
    current_turn_authority_from_projection,
    current_turn_phase_config,
    projection_current_turn_instance_id,
)
from runner.graph_runtime.continuation import (
    TURN_CONTINUATION_KEY,
    RecoveryTurnRequest,
    OutcomeRepairTurnRequest,
    admit_pending_turn_recovery,
    continuation_recovery,
    is_outcome_repair_continuation,
    ordinary_turn_continuation,
    outcome_repair_turn_continuation,
    pending_recovery_reason,
    recovery_turn_continuation,
)
from runner.graph_runtime.runtime_lane import refresh_projection
from runner.graph_runtime.state import (
    LOWERED_PHASE_PROGRAM_KEY,
    RUN_AUTHORITY_KEY,
    RUN_CONTEXT_KEY,
    GraphState,
    LoweredGraphPhaseProgram,
    RoleSessionSelection,
    ROLE_SESSION_KEY,
)
from runner.phase_programs import lower_phase_program
from runner.phase_programs.lowered_program import PHASE_ASSET_PROMPT_BINDING
from runner.roles import apply_model_override, resolve_role_policy


def load_run_authority(state: GraphState) -> GraphState:
    run_context = state[RUN_CONTEXT_KEY]
    config = load_config(run_context.config_path)
    projection = refresh_projection(run_context.config_path, run_context.run_id)
    current_turn = current_turn_authority_from_projection(projection)
    if current_turn is None:
        return {
            RUN_AUTHORITY_KEY: LoadedRunAuthority(config=config, projection=projection),
            TURN_CONTINUATION_KEY: ordinary_turn_continuation(),
        }
    events = load_events(RuntimePaths(run_context.run_id).events)
    recovery = pending_recovery_reason(
        events,
        projection["current"],
        projection.get("current_turn_instance_id"),
    )
    turn_continuation = ordinary_turn_continuation()
    if recovery is not None:
        admitted = admit_pending_turn_recovery(
            config=config,
            events=events,
            phase_id=current_turn.phase_id,
            turn=current_turn.turn,
            pending_reason=recovery.reason,
            pending_failure_family=recovery.failure_family,
            turn_instance_id=projection_current_turn_instance_id(
                LoadedRunAuthority(config=config, projection=projection)
            ),
            session_reset_threshold=recovery.session_reset_threshold,
            session_reset_cycle_count=recovery.session_reset_cycle_count,
        )
        if isinstance(admitted, OutcomeRepairTurnRequest):
            turn_continuation = outcome_repair_turn_continuation(admitted)
        else:
            turn_continuation = recovery_turn_continuation(admitted)
    return {
        RUN_AUTHORITY_KEY: LoadedRunAuthority(config=config, projection=projection),
        CURRENT_TURN_AUTHORITY_KEY: current_turn,
        TURN_CONTINUATION_KEY: turn_continuation,
    }


def lower_phase_program_node(state: GraphState) -> GraphState:
    run_authority = state[RUN_AUTHORITY_KEY]
    current_turn = current_turn_authority_from_state(state)
    if current_turn is None:
        raise ValueError("phase-program lowering requires current turn authority")
    current_phase = current_turn_phase_config(run_authority, current_turn)
    lowered_program = lower_phase_program(run_authority.config, current_phase)
    return {
        LOWERED_PHASE_PROGRAM_KEY: LoweredGraphPhaseProgram(
            phase_id=current_turn.phase_id,
            turn=current_turn.turn,
            program_id=lowered_program.program_id,
            prompt_binding_mode=lowered_program.prompt_binding_mode_for_turn(current_turn.turn),
            prompt_topology_id=lowered_prompt_topology_id(lowered_program, current_turn.turn),
            supported_outcomes=lowered_program.supported_outcomes_for_turn(current_turn.turn),
        )
    }


def resolved_role_policy_with_override(run_authority, phase_id: int, turn: str):
    """Resolve a turn's role policy, then apply any escalation-activated model
    override that covers it. This is the single execution-path chokepoint, so a
    scoped model escalation (e.g. repair turns -> stronger model) takes effect
    without touching the static role bindings."""
    return apply_model_override(
        resolve_role_policy(run_authority.config, phase_id, turn),
        run_authority.projection,
        phase_id,
        turn,
    )


def select_role_session(state: GraphState) -> GraphState:
    run_authority = state[RUN_AUTHORITY_KEY]
    current_turn = current_turn_authority_from_state(state)
    if current_turn is None:
        raise ValueError("role-session selection requires current turn authority")
    if TURN_CONTINUATION_KEY in state:
        continuation = state[TURN_CONTINUATION_KEY]
        if is_outcome_repair_continuation(continuation):
            return {
                ROLE_SESSION_KEY: RoleSessionSelection(
                    role_policy=resolved_role_policy_with_override(
                        run_authority, current_turn.phase_id, current_turn.turn
                    )
                )
            }
        if continuation.mode == "recovery":
            recovery = continuation_recovery(continuation)
            if recovery is None or recovery.role_route == "projection":
                return {ROLE_SESSION_KEY: RoleSessionSelection(role_policy=None)}
            if recovery.role_route == "reviewer":
                return {
                    ROLE_SESSION_KEY: RoleSessionSelection(
                        role_policy=resolved_role_policy_with_override(
                            run_authority,
                            current_turn.phase_id,
                            recovery_turn_for_phase(run_authority.config, current_turn.phase_id),
                        )
                    )
                }
            return {
                ROLE_SESSION_KEY: RoleSessionSelection(
                    role_policy=resolved_role_policy_with_override(
                        run_authority, current_turn.phase_id, current_turn.turn
                    )
                )
            }
    return {
        ROLE_SESSION_KEY: RoleSessionSelection(
            role_policy=resolved_role_policy_with_override(
                run_authority, current_turn.phase_id, current_turn.turn
            )
        )
    }


def lowered_prompt_topology_id(lowered_program, turn: str) -> str:
    prompt_binding_mode = lowered_program.prompt_binding_mode_for_turn(turn)
    if prompt_binding_mode == PHASE_ASSET_PROMPT_BINDING:
        return PHASE_ASSET_PROMPT_BINDING
    return prompt_binding_mode


def recovery_turn_for_phase(config: dict[str, object], phase_id: int) -> str:
    for preferred_turn in ("review", "code_quality_review", "test_review"):
        try:
            resolve_role_policy(config, phase_id, preferred_turn)
        except ValueError:
            continue
        return preferred_turn
    raise ValueError(f"phase {phase_id} does not admit a reviewer recovery turn")

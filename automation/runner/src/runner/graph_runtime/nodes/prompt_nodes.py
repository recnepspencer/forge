from __future__ import annotations

from typing import Any

from runner.authority.run_identity import RuntimePaths, new_run_id
from runner.graph_runtime.authority import (
    CURRENT_TURN_AUTHORITY_KEY,
    admitted_run_authority_from_state,
    current_turn_payload,
    current_turn_authority_from_state,
    projection_session_thread_id,
)
from runner.graph_runtime.continuation import (
    TURN_CONTINUATION_KEY,
    continuation_outcome_repair,
    continuation_recovery,
)
from runner.graph_runtime.prompt_turns import build_recovery_prompt, prepare_execution_prompt_turn
from runner.graph_runtime.runtime_lane import append_runtime_event
from runner.graph_runtime.state import (
    LOWERED_PHASE_PROGRAM_KEY,
    PROMPT_TURN_KEY,
    ROLE_SESSION_KEY,
    RUN_AUTHORITY_KEY,
    RUN_CONTEXT_KEY,
    GraphState,
    PromptTurnDelivery,
)
from runner.phase_programs.lowered_program import PHASE_ASSET_PROMPT_BINDING, TURN_ASSEMBLY_PROMPT_BINDING
from runner.prompt_library.registry import prompt_registry


def materialize_turn_assembly_prompt(state: GraphState) -> GraphState:
    return materialize_ordinary_prompt(state, expected_binding_mode=TURN_ASSEMBLY_PROMPT_BINDING)


def materialize_phase_asset_prompt(state: GraphState) -> GraphState:
    return materialize_ordinary_prompt(state, expected_binding_mode=PHASE_ASSET_PROMPT_BINDING)


def materialize_ordinary_prompt(state: GraphState, *, expected_binding_mode: str) -> GraphState:
    run_authority = admitted_run_authority_from_state(state)
    if run_authority is None:
        raise ValueError("ordinary prompt materialization requires run authority")
    lowered_program = state[LOWERED_PHASE_PROGRAM_KEY]
    current_turn = current_turn_authority_from_state(state)
    if current_turn is None:
        raise ValueError("ordinary prompt materialization requires current turn authority")
    if state[TURN_CONTINUATION_KEY].mode != "ordinary":
        return materialize_recovery_prompt(state)
    if lowered_program.prompt_binding_mode != expected_binding_mode:
        raise ValueError(
            f"program {lowered_program.program_id!r} turn {lowered_program.turn!r} requires "
            f"{lowered_program.prompt_binding_mode!r} prompt binding, not {expected_binding_mode!r}"
        )
    run_context = state[RUN_CONTEXT_KEY]
    role_session = state[ROLE_SESSION_KEY]
    turn_instance_id = new_run_id()
    prepared_prompt = prepare_execution_prompt_turn(
        run_authority.config,
        run_authority.projection,
        run_context.config_path,
        RuntimePaths(run_context.run_id).projection,
        RuntimePaths(run_context.run_id).events,
        prompt_template_override=role_session.role_policy.prompt_template_override,
        expected_turn_instance_id=turn_instance_id,
        record_run_id=run_context.run_id,
        record_turn_instance_id=turn_instance_id,
    )
    append_runtime_event(
        RuntimePaths(run_context.run_id),
        "prompt_selected",
        phase_id=current_turn.phase_id,
        turn=current_turn.turn,
        payload=build_prompt_selected_payload(
            current_turn_payload(current_turn),
            turn_instance_id,
            prepared_prompt,
            role_session.role_policy,
            prompt_registry(run_authority.config),
        ),
        thread_id=projection_session_thread_id(run_authority),
    )
    prompt_state = {
        PROMPT_TURN_KEY: PromptTurnDelivery(
            turn_instance_id=turn_instance_id,
            delivery_prompt=prepared_prompt.delivery_prompt,
        )
    }
    if CURRENT_TURN_AUTHORITY_KEY not in state:
        prompt_state[CURRENT_TURN_AUTHORITY_KEY] = current_turn
    if RUN_AUTHORITY_KEY not in state or state.get(RUN_AUTHORITY_KEY) is None:
        prompt_state[RUN_AUTHORITY_KEY] = run_authority
    return prompt_state


def materialize_recovery_prompt(state: GraphState) -> GraphState:
    run_context = state[RUN_CONTEXT_KEY]
    run_authority = admitted_run_authority_from_state(state)
    if run_authority is None:
        raise ValueError("recovery prompt materialization requires run authority")
    current_turn = current_turn_authority_from_state(state)
    if current_turn is None:
        raise ValueError("recovery prompt materialization requires current turn authority")
    continuation = state[TURN_CONTINUATION_KEY]
    outcome_repair = continuation_outcome_repair(continuation)
    recovery = continuation_recovery(continuation)
    if outcome_repair is None and recovery is None:
        raise ValueError("recovery prompt requested without recovery request state")
    request = outcome_repair if outcome_repair is not None else recovery
    assert request is not None
    if recovery is not None and recovery.exhausted_disposition is not None:
        # The terminal disposition is published by the authority-event node
        # together with the failure that exhausted the policy.  It is never an
        # executable prompt.
        raise ValueError("exhausted recovery disposition must not materialize a prompt")
    # A preflight signal has no displaced in-flight turn to reuse.  The runtime
    # therefore mints its execution identity here, at the prompt boundary.
    turn_instance_id = request.turn_instance_id or new_run_id()
    if recovery is not None and recovery.force_fresh_session:
        append_runtime_event(
            RuntimePaths(run_context.run_id),
            "session_reset",
            phase_id=current_turn.phase_id,
            turn=current_turn.turn,
            payload={
                "reason": f"fresh session for {recovery.attempt_action} recovery attempt {recovery.attempt_index}",
                "cycle_count": recovery.attempt_index,
                "threshold": recovery.attempt_index,
                "turn_instance_id": turn_instance_id,
            },
            thread_id=projection_session_thread_id(run_authority),
        )
    append_runtime_event(
        RuntimePaths(run_context.run_id),
        "recovery_requested",
        phase_id=current_turn.phase_id,
        turn=current_turn.turn,
        payload={
            "reason": request.reason,
            "turn_instance_id": turn_instance_id,
            "failure_family": request.failure_family,
            "recovery_kind": "outcome_repair" if outcome_repair is not None else "escalation_recovery",
            "attempt_index": request.attempt_index,
            "attempt_action": request.attempt_action,
        },
        thread_id=projection_session_thread_id(run_authority),
    )
    prompt_state = {
        PROMPT_TURN_KEY: PromptTurnDelivery(
            turn_instance_id=turn_instance_id,
            delivery_prompt=build_recovery_prompt(
                run_authority.config,
                run_authority.projection,
                RuntimePaths(run_context.run_id),
                request.reason,
                turn_instance_id,
                failure_family=request.failure_family,
                recovery_kind="outcome_repair" if outcome_repair is not None else "escalation_recovery",
                recovery_route_guidance=recovery_route_guidance(continuation),
            ),
        )
    }
    if CURRENT_TURN_AUTHORITY_KEY not in state:
        prompt_state[CURRENT_TURN_AUTHORITY_KEY] = current_turn
    if RUN_AUTHORITY_KEY not in state or state.get(RUN_AUTHORITY_KEY) is None:
        prompt_state[RUN_AUTHORITY_KEY] = run_authority
    return prompt_state


def recovery_route_guidance(continuation) -> str:
    outcome_repair = continuation_outcome_repair(continuation)
    if outcome_repair is not None:
        return "This is an event-repair attempt. Reconstruct the missing or malformed runner outcome without redoing the work."
    recovery = continuation_recovery(continuation)
    if recovery is None:
        return ""
    if recovery.attempt_action == "deep_reviewer_pass":
        return "This recovery attempt is an escalated reviewer pass. Rebuild context independently and correct the turn honestly."
    if recovery.attempt_action == "start_fresh_session":
        return "This recovery attempt is running in a fresh session. Rebuild context before continuing the same turn."
    return "This recovery attempt should continue in the same session when it is still healthy."


def build_prompt_selected_payload(
    current: dict[str, Any],
    turn_instance_id: str,
    prepared_prompt,
    role_policy: Any,
    registry,
) -> dict[str, Any]:
    contract_details = registry.asset_details(prepared_prompt.contract_asset_id)
    payload = {
        "summary": f"phase {current['phase']} {current['turn']}",
        "turn_instance_id": turn_instance_id,
        "role_id": role_policy.role.role_id,
        "session_family": role_policy.session_policy.continuity_family,
        "escalation_posture": role_policy.handoff_policy.escalation_posture,
        "contract_asset_id": prepared_prompt.contract_asset_id,
        "contract_root_kind": contract_details.root_kind,
        "contract_source_path": str(contract_details.source_path),
    }
    if prepared_prompt.prompt_asset_id is not None:
        prompt_asset_details = registry.asset_details(prepared_prompt.prompt_asset_id)
        payload["prompt_asset_id"] = prepared_prompt.prompt_asset_id
        payload["prompt_asset_root_kind"] = prompt_asset_details.root_kind
        payload["prompt_asset_source_path"] = str(prompt_asset_details.source_path)
    if prepared_prompt.prompt_assembly_id is not None:
        prompt_assembly_details = registry.assembly_details(prepared_prompt.prompt_assembly_id)
        payload["prompt_assembly_id"] = prepared_prompt.prompt_assembly_id
        payload["prompt_assembly_root_kind"] = prompt_assembly_details.root_kind
        payload["prompt_assembly_source_path"] = str(prompt_assembly_details.source_path)
    return payload

from __future__ import annotations

import os
import hashlib

from runner.adapters.agent_cli import run_agent
from runner.authority.run_identity import RuntimePaths, stop_requested
from runner.graph_runtime.authority import CURRENT_TURN_AUTHORITY_KEY, current_turn_authority_from_state, current_turn_payload
from runner.graph_runtime.continuation import (
    TURN_CONTINUATION_KEY,
    continuation_recovery,
)
from runner.graph_runtime.recovery_runtime import build_inflight_no_progress_watchdog
from runner.graph_runtime.execution_authority import (
    claim_execution,
    finish_execution,
    record_process_launch,
)
from runner.graph_runtime.state import (
    PROMPT_TURN_KEY,
    ROLE_SESSION_KEY,
    RUN_AUTHORITY_KEY,
    RUN_CONTEXT_KEY,
    TURN_EXECUTION_KEY,
    GraphState,
    TurnExecutionCapture,
)
from runner.roles import project_current_session


def execute_role_turn(state: GraphState) -> GraphState:
    run_context = state[RUN_CONTEXT_KEY]
    run_authority = state[RUN_AUTHORITY_KEY]
    current_turn = current_turn_authority_from_state(state)
    if current_turn is None:
        raise ValueError("turn execution requires current turn authority")
    prompt_turn = state[PROMPT_TURN_KEY]
    if prompt_turn.turn_instance_id is None:
        raise ValueError("turn execution requires a turn instance id")
    paths = RuntimePaths(run_context.run_id)
    execution_id = execution_identity(prompt_turn.turn_instance_id, prompt_turn.delivery_prompt)
    receipt, claimed_now = claim_execution(paths, execution_id, prompt_turn.turn_instance_id)
    if receipt.state == "finished":
        if receipt.exit_code is None or receipt.capture is None:
            raise ValueError("finished execution receipt is incomplete")
        return execution_capture_state(state, current_turn, receipt.exit_code, receipt.capture)
    if not claimed_now:
        return execution_capture_state(
            state,
            current_turn,
            125,
            {
                "agent_messages": [],
                "failure_reason": "provider execution was interrupted before a durable completion receipt",
                "failure_family": "interrupted_execution",
                "provider_pid": receipt.provider_pid,
            },
        )
    role_session = state[ROLE_SESSION_KEY]
    execution_projection = dict(run_authority.projection)
    if role_session.role_policy is not None:
        execution_projection["session"] = role_session.role_policy.execution_session(
            project_current_session(
                run_authority.config,
                current_turn_payload(current_turn),
                run_authority.projection["session"],
            )
        )
    recovery = continuation_recovery(state[TURN_CONTINUATION_KEY])
    if recovery is not None and recovery.force_fresh_session:
        execution_projection["session"] = dict(execution_projection["session"])
        execution_projection["session"]["reuse_session"] = False
        execution_projection["session"]["thread_id"] = None
    try:
        exit_code, capture = run_agent(
            execution_projection,
            prompt_turn.delivery_prompt,
            run_context.log_path or RuntimePaths(run_context.run_id).log,
            stop_requested_fn=lambda: stop_requested(paths),
            progress_watchdog_fn=build_inflight_no_progress_watchdog(
                run_authority.config,
                run_context.run_id,
                current_turn_payload(current_turn),
                prompt_turn.turn_instance_id,
            ),
            process_started_fn=lambda pid: record_process_launch(paths, execution_id, pid),
        )
    except Exception as error:
        exit_code = 125
        capture = {
            "agent_messages": [],
            "failure_reason": f"provider capture failed: {type(error).__name__}: {error}",
            "failure_family": "provider_crash",
        }
    finish_execution(paths, execution_id, exit_code, capture)
    return execution_capture_state(state, current_turn, exit_code, capture)


def execution_capture_state(state, current_turn, exit_code, capture) -> GraphState:
    execution_state = {TURN_EXECUTION_KEY: TurnExecutionCapture(exit_code=exit_code, capture=capture)}
    if CURRENT_TURN_AUTHORITY_KEY not in state:
        execution_state[CURRENT_TURN_AUTHORITY_KEY] = current_turn
    return execution_state


def execution_identity(turn_instance_id: str, delivery_prompt: str) -> str:
    prompt_digest = hashlib.sha256(delivery_prompt.encode("utf-8")).hexdigest()[:16]
    return f"{turn_instance_id}-{prompt_digest}"

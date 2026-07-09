from __future__ import annotations

import shutil
from typing import Callable

from langgraph.checkpoint.sqlite import SqliteSaver
from runner.authority.events.run_authority import load_admitted_run_projection_inputs
from runner.graph_runtime.continuation.recovery_admission import pending_recovery_reason
from langgraph.graph import END, START, StateGraph
from runner.graph_runtime.graph_plan import GraphExecutionPlan
from runner.graph_runtime.nodes import GRAPH_NODE_IDS
from runner.graph_runtime.nodes.authority_event_nodes import append_runner_event
from runner.graph_runtime.nodes.authority_nodes import load_run_authority, lower_phase_program_node, select_role_session
from runner.graph_runtime.nodes.continuation_nodes import prepare_next_step_state
from runner.graph_runtime.nodes.escalation_nodes import evaluate_escalation
from runner.graph_runtime.nodes.execution_nodes import execute_role_turn
from runner.graph_runtime.nodes.node_ids import (
    APPEND_RUNNER_EVENT_NODE_ID,
    CLASSIFY_TURN_OUTCOME_NODE_ID,
    EVALUATE_ESCALATION_NODE_ID,
    EXECUTE_ROLE_TURN_NODE_ID,
    LOAD_RUN_AUTHORITY_NODE_ID,
    MATERIALIZE_PHASE_ASSET_PROMPT_NODE_ID,
    MATERIALIZE_TURN_ASSEMBLY_PROMPT_NODE_ID,
    LOWER_PHASE_PROGRAM_NODE_ID,
    PUBLISH_PROJECTION_NODE_ID,
    ROUTE_NEXT_STEP_NODE_ID,
    ROUTE_OUTCOME_REPAIR_OR_RECOVERY_NODE_ID,
    SELECT_ROLE_SESSION_NODE_ID,
)
from runner.graph_runtime.nodes.outcome_nodes import classify_turn_outcome, route_outcome_repair_or_recovery
from runner.graph_runtime.nodes.prompt_nodes import materialize_phase_asset_prompt, materialize_turn_assembly_prompt
from runner.graph_runtime.nodes.projection_nodes import publish_projection
from runner.graph_runtime.routes.continuation_routes import next_graph_destination, next_prompt_materialization_node
from runner.graph_runtime.state import (
    CURRENT_TURN_AUTHORITY_KEY,
    FINISH_TRANSITION,
    GraphState,
    LOWERED_PHASE_PROGRAM_KEY,
    PROMPT_TURN_KEY,
    ROLE_SESSION_KEY,
    RUN_AUTHORITY_KEY,
    RUN_CONTEXT_KEY,
    TURN_CONTINUATION_KEY,
    TURN_EXECUTION_KEY,
    TURN_OUTCOME_KEY,
    TURN_TRANSITION_KEY,
)
from runner.graph_runtime.subgraphs import prompt_materialization_destinations, prompt_materialization_edges

GraphNodeHandler = Callable[[GraphState], GraphState]


def execute_graph_plan(plan: GraphExecutionPlan, graph_state: GraphState) -> GraphState:
    plan.checkpoint_surface.root.mkdir(parents=True, exist_ok=True)
    if pending_checkpoint_requires_reset(plan):
        reset_checkpoint_surface(plan)
    with SqliteSaver.from_conn_string(str(plan.checkpoint_surface.database_path)) as saver:
        compiled_graph = compile_graph_plan(plan, saver)
        config = {"configurable": {"thread_id": plan.run_id}}
        graph_input = graph_state if not graph_has_pending_checkpoint(compiled_graph, config) else None
        return compiled_graph.invoke(graph_input, config=config)


def compile_graph_plan(plan: GraphExecutionPlan, saver: SqliteSaver):
    plan.checkpoint_surface.root.mkdir(parents=True, exist_ok=True)
    builder = StateGraph(GraphState)
    for node_id, handler in graph_node_handlers(plan.node_ids).items():
        builder.add_node(node_id, state_node(handler))
    for start_node, end_node in plan.prompt_edges:
        builder.add_edge(start_node, end_node)
    builder.add_conditional_edges(
        SELECT_ROLE_SESSION_NODE_ID,
        next_prompt_materialization_node,
        plan.prompt_destinations,
    )
    builder.add_conditional_edges(
        ROUTE_NEXT_STEP_NODE_ID,
        next_graph_destination,
        {
            "finish": END,
            **plan.prompt_destinations,
        },
    )
    builder.add_edge(START, plan.node_ids[0])
    for start_node, end_node in plan.edges:
        builder.add_edge(start_node, end_node)
    return builder.compile(checkpointer=saver, name=plan.graph_name)


def state_node(handler: GraphNodeHandler):
    def wrapped(state: GraphState) -> GraphState:
        return {**state, **handler(state)}  # type: ignore[arg-type]

    return wrapped


def graph_node_handlers(node_ids: tuple[str, ...]) -> dict[str, GraphNodeHandler]:
    handlers = {
        "load_run_authority": load_run_authority,
        "lower_phase_program": lower_phase_program_node,
        "select_role_session": select_role_session,
        MATERIALIZE_TURN_ASSEMBLY_PROMPT_NODE_ID: materialize_turn_assembly_prompt,
        MATERIALIZE_PHASE_ASSET_PROMPT_NODE_ID: materialize_phase_asset_prompt,
        "execute_role_turn": execute_role_turn,
        "classify_turn_outcome": classify_turn_outcome,
        ROUTE_OUTCOME_REPAIR_OR_RECOVERY_NODE_ID: route_outcome_repair_or_recovery,
        "append_runner_event": append_runner_event,
        "publish_projection": publish_projection,
        "evaluate_escalation": evaluate_escalation,
        ROUTE_NEXT_STEP_NODE_ID: prepare_next_step_state,
    }
    return {node_id: handlers[node_id] for node_id in node_ids if node_id in handlers}


def graph_has_pending_checkpoint(compiled_graph, config: dict[str, object]) -> bool:
    snapshot = compiled_graph.get_state(config)
    return bool(getattr(snapshot, "next", ()))


def pending_checkpoint_requires_reset(plan: GraphExecutionPlan) -> bool:
    with SqliteSaver.from_conn_string(str(plan.checkpoint_surface.database_path)) as saver:
        compiled_graph = compile_graph_plan(plan, saver)
        snapshot = compiled_graph.get_state({"configurable": {"thread_id": plan.run_id}})
    pending_nodes = tuple(getattr(snapshot, "next", ()))
    if not pending_nodes:
        return False
    resume_node_id = pending_nodes[0]
    required_keys = required_checkpoint_keys_for_node(resume_node_id)
    if not required_keys:
        return False
    values = getattr(snapshot, "values", {}) or {}
    if any(values.get(key) is None for key in required_keys):
        return True
    return authoritative_pending_turn_requires_checkpoint_reset(plan.run_id)


def authoritative_pending_turn_requires_checkpoint_reset(run_id: str) -> bool:
    _, _, events = load_admitted_run_projection_inputs(run_id)
    if not events:
        return False
    current = current_turn_for_events(events)
    if current is None:
        return False
    current_turn_instance_id = current_turn_instance_id_for_events(events, current["phase"], current["turn"])
    if current_turn_instance_id is None:
        return False
    return pending_recovery_reason(events, current, current_turn_instance_id) is not None


def current_turn_for_events(events: tuple[dict[str, object], ...]) -> dict[str, object] | None:
    for event in reversed(events):
        phase_id = event.get("phase_id")
        turn = event.get("turn")
        if isinstance(phase_id, int) and isinstance(turn, str):
            return {"phase": phase_id, "turn": turn}
    return None


def current_turn_instance_id_for_events(
    events: tuple[dict[str, object], ...],
    phase_id: int,
    turn: str,
) -> str | None:
    for event in reversed(events):
        if event.get("phase_id") != phase_id or event.get("turn") != turn:
            continue
        payload = event.get("payload", {})
        if not isinstance(payload, dict):
            continue
        turn_instance_id = payload.get("turn_instance_id")
        if isinstance(turn_instance_id, str) and turn_instance_id:
            return turn_instance_id
    return None


def reset_checkpoint_surface(plan: GraphExecutionPlan) -> None:
    if plan.checkpoint_surface.root.exists():
        shutil.rmtree(plan.checkpoint_surface.root)
    plan.checkpoint_surface.root.mkdir(parents=True, exist_ok=True)


def required_checkpoint_keys_for_node(node_id: str) -> tuple[str, ...]:
    required_keys = {
        LOAD_RUN_AUTHORITY_NODE_ID: (RUN_CONTEXT_KEY,),
        LOWER_PHASE_PROGRAM_NODE_ID: (RUN_CONTEXT_KEY, RUN_AUTHORITY_KEY, CURRENT_TURN_AUTHORITY_KEY),
        SELECT_ROLE_SESSION_NODE_ID: (RUN_CONTEXT_KEY, RUN_AUTHORITY_KEY, CURRENT_TURN_AUTHORITY_KEY),
        MATERIALIZE_TURN_ASSEMBLY_PROMPT_NODE_ID: (
            RUN_CONTEXT_KEY,
            RUN_AUTHORITY_KEY,
            CURRENT_TURN_AUTHORITY_KEY,
            LOWERED_PHASE_PROGRAM_KEY,
            ROLE_SESSION_KEY,
            TURN_CONTINUATION_KEY,
        ),
        MATERIALIZE_PHASE_ASSET_PROMPT_NODE_ID: (
            RUN_CONTEXT_KEY,
            RUN_AUTHORITY_KEY,
            CURRENT_TURN_AUTHORITY_KEY,
            LOWERED_PHASE_PROGRAM_KEY,
            ROLE_SESSION_KEY,
            TURN_CONTINUATION_KEY,
        ),
        EXECUTE_ROLE_TURN_NODE_ID: (
            RUN_CONTEXT_KEY,
            RUN_AUTHORITY_KEY,
            CURRENT_TURN_AUTHORITY_KEY,
            ROLE_SESSION_KEY,
            TURN_CONTINUATION_KEY,
            PROMPT_TURN_KEY,
        ),
        CLASSIFY_TURN_OUTCOME_NODE_ID: (PROMPT_TURN_KEY, TURN_EXECUTION_KEY),
        ROUTE_OUTCOME_REPAIR_OR_RECOVERY_NODE_ID: (
            TURN_CONTINUATION_KEY,
            TURN_OUTCOME_KEY,
            PROMPT_TURN_KEY,
        ),
        APPEND_RUNNER_EVENT_NODE_ID: (
            RUN_CONTEXT_KEY,
            RUN_AUTHORITY_KEY,
            CURRENT_TURN_AUTHORITY_KEY,
            TURN_CONTINUATION_KEY,
            PROMPT_TURN_KEY,
            TURN_EXECUTION_KEY,
            TURN_OUTCOME_KEY,
        ),
        PUBLISH_PROJECTION_NODE_ID: (
            RUN_CONTEXT_KEY,
            RUN_AUTHORITY_KEY,
            TURN_CONTINUATION_KEY,
            TURN_TRANSITION_KEY,
        ),
        EVALUATE_ESCALATION_NODE_ID: (RUN_AUTHORITY_KEY, TURN_TRANSITION_KEY),
        ROUTE_NEXT_STEP_NODE_ID: (TURN_TRANSITION_KEY,),
    }
    return required_keys.get(node_id, ())

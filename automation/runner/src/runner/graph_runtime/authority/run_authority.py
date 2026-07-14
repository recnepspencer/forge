from __future__ import annotations

from dataclasses import dataclass
from typing import Any

CURRENT_TURN_AUTHORITY_KEY = "current_turn_authority"


@dataclass(frozen=True)
class CurrentTurnAuthority:
    phase_id: int
    turn: str


@dataclass(frozen=True)
class LoadedRunAuthority:
    config: dict[str, Any]
    projection: dict[str, Any]


def current_turn_authority_from_projection(projection: dict[str, Any]) -> CurrentTurnAuthority | None:
    current = projection.get("current")
    if not isinstance(current, dict):
        return None
    return CurrentTurnAuthority(phase_id=current["phase"], turn=current["turn"])


def admitted_run_authority_from_state(state: dict[str, Any]) -> LoadedRunAuthority | None:
    run_authority = state.get("run_authority")
    if isinstance(run_authority, LoadedRunAuthority):
        return run_authority
    config = getattr(run_authority, "config", None)
    projection = getattr(run_authority, "projection", None)
    if isinstance(config, dict) and isinstance(projection, dict):
        return LoadedRunAuthority(config=config, projection=projection)
    run_context = state.get("run_context")
    run_id = getattr(run_context, "run_id", None)
    config_path = getattr(run_context, "config_path", None)
    if not isinstance(run_id, str) or config_path is None:
        return None
    from runner.authority.config import load_config
    from runner.graph_runtime.runtime_lane import refresh_projection

    loaded_config = load_config(config_path)
    projection = refresh_projection(config_path, run_id)
    return LoadedRunAuthority(config=loaded_config, projection=projection)


def current_turn_authority_from_state(state: dict[str, Any]) -> CurrentTurnAuthority | None:
    current_turn = state.get(CURRENT_TURN_AUTHORITY_KEY)
    if isinstance(current_turn, CurrentTurnAuthority):
        return current_turn
    phase_id = getattr(current_turn, "phase_id", None)
    turn = getattr(current_turn, "turn", None)
    if isinstance(phase_id, int) and isinstance(turn, str):
        return CurrentTurnAuthority(phase_id=phase_id, turn=turn)
    run_authority = admitted_run_authority_from_state(state)
    if run_authority is not None:
        return current_turn_authority_from_projection(run_authority.projection)
    return None


def current_turn_phase_config(
    authority: LoadedRunAuthority,
    current_turn: CurrentTurnAuthority,
) -> dict[str, Any]:
    return next(
        phase
        for phase in authority.config["phases"]
        if phase["id"] == current_turn.phase_id
    )


def current_turn_payload(current_turn: CurrentTurnAuthority) -> dict[str, Any]:
    return {"phase": current_turn.phase_id, "turn": current_turn.turn}


def projection_session_thread_id(authority: LoadedRunAuthority) -> str | None:
    thread_id = authority.projection.get("session", {}).get("thread_id")
    return thread_id if isinstance(thread_id, str) and thread_id else None


def projection_current_turn_instance_id(authority: LoadedRunAuthority) -> str | None:
    turn_instance_id = authority.projection.get("current_turn_instance_id")
    return turn_instance_id if isinstance(turn_instance_id, str) and turn_instance_id else None

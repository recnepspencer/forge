from __future__ import annotations

from pathlib import Path

from runner.authority.config import load_config
from runner.authority.events import load_events
from runner.authority.run_identity import (
    RuntimePaths,
    acquire_active_run_lock,
    clear_stop_requested,
    mark_stop_requested,
)
from runner.graph_runtime.resume_runtime import resume_run_with_reason as resume_graph_run_with_reason
from runner.facade.runtime_state import (
    append_runtime_event,
    config_path_for_run,
    refresh_projection,
    refresh_projection_for_run,
    initialize_runtime_events,
)
from runner.facade.plan_revision import adopt_plan_payload
from runner.generation.legacy_importer import import_legacy_run as import_legacy_run_impl
from runner.phase_programs.policy_bindings import operator_custom_turn_config, operator_intervention_policy


def start_run(
    config_path: Path,
    run_id: str | None,
    loop: bool,
    sleep_seconds: int,
    log_path: Path | None,
) -> int:
    config = load_config(config_path)
    from runner.authority.run_identity import new_run_id
    from runner.facade.turn_runtime.run_loop import drive_run

    active_run_id = run_id or new_run_id()
    paths = RuntimePaths(active_run_id)
    with acquire_active_run_lock(paths):
        clear_stop_requested(paths)
        if paths.events.exists():
            raise ValueError(f"run {active_run_id!r} already exists")
        initialize_runtime_events(
            paths,
            [
                ("run_started", {"config_path": str(config_path.resolve())}),
                ("plan_adopted", adopt_plan_payload(config_path, config)),
            ],
        )
        return drive_run(config_path, active_run_id, loop, sleep_seconds, log_path)


def resume_run(run_id: str, loop: bool, sleep_seconds: int, log_path: Path | None) -> int:
    return resume_run_with_reason(run_id, loop, sleep_seconds, log_path, "operator resume")


def resume_run_with_reason(
    run_id: str,
    loop: bool,
    sleep_seconds: int,
    log_path: Path | None,
    reason: str,
) -> int:
    return resume_graph_run_with_reason(run_id, loop, sleep_seconds, log_path, reason)


def stop_run(run_id: str, reason: str) -> None:
    paths = RuntimePaths(run_id)
    mark_stop_requested(paths)
    append_runtime_event(paths, "run_stopped", payload={"reason": reason})
    refresh_projection_for_run(run_id)


def inject_operator_override(
    run_id: str,
    message: str,
    *,
    phase_id: int | None = None,
    turn: str | None = None,
    source_id: str | None = None,
) -> None:
    config_path = config_path_for_run(run_id)
    config = load_config(config_path)
    policy = operator_intervention_policy(config)
    if not policy.allow_live_injection:
        raise ValueError(f"run {run_id!r} does not allow live operator injection")
    if not policy.record_as_authority_event:
        raise ValueError(f"run {run_id!r} does not record operator injection as an authority event")
    projection = refresh_projection_for_run(run_id)
    if source_id is not None and operator_override_source_exists(RuntimePaths(run_id), source_id):
        return
    model_policy, instructions = parse_operator_custom_turn(config, message)
    target = operator_override_target(projection, phase_id, turn)
    append_runtime_event(
        RuntimePaths(run_id),
        "operator_override",
        phase_id=target["phase"],
        turn=target["turn"],
        payload={
            "current": target,
            "reason": instructions,
            "injection_mode": policy.default_injection_mode,
            "post_injection_route": policy.default_post_injection_route,
            "source_id": source_id,
            "model_policy": model_policy,
        },
        thread_id=projection["session"]["thread_id"],
    )
    refresh_projection_for_run(run_id)


def parse_operator_custom_turn(config: dict, message: str) -> tuple[dict | None, str]:
    """A custom-turn reply is `<model alias> <instructions>`. The first token,
    if it names a configured alias, picks the model; the remainder is the turn
    instructions. Absent an alias prefix, the configured default_alias is used
    and the whole reply is instructions. Without an operator_custom_turn config,
    the reply is a plain injection at the current model."""
    custom = operator_custom_turn_config(config)
    text = message.strip()
    if custom is None:
        return None, text
    aliases = custom.get("aliases", {})
    token, _, rest = text.partition(" ")
    alias = token.rstrip(":").lower()
    if alias in aliases:
        instructions = rest.strip()
        if not instructions:
            raise ValueError("operator custom turn requires instructions after the model name")
        return aliases[alias], instructions
    default_alias = custom.get("default_alias")
    if default_alias in aliases:
        return aliases[default_alias], text
    return None, text


def operator_override_source_exists(paths: RuntimePaths, source_id: str) -> bool:
    return any(
        event.get("event_type") == "operator_override"
        and isinstance(event.get("payload"), dict)
        and event["payload"].get("source_id") == source_id
        for event in load_events(paths.events)
    )


def import_legacy_run(old_state_path: Path, config_path: Path, run_id: str | None) -> str:
    from runner.authority.run_identity import new_run_id

    return import_legacy_run_impl(
        old_state_path,
        config_path,
        run_id,
        append_runtime_event,
        refresh_projection,
        new_run_id,
    )


def operator_override_target(
    projection: dict[str, object],
    phase_id: int | None,
    turn: str | None,
) -> dict[str, object]:
    current = projection.get("current")
    if not isinstance(current, dict):
        raise ValueError("operator injection requires an active current cursor")
    if phase_id is None and turn is None:
        return {"phase": current["phase"], "turn": current["turn"]}
    if phase_id != current["phase"] or turn != current["turn"]:
        raise ValueError("operator injection may only target the admitted active cursor")
    return {"phase": phase_id, "turn": turn}

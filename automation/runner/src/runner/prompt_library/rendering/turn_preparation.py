from __future__ import annotations

import json
from dataclasses import dataclass
from pathlib import Path
from typing import Any

from runner.prompt_library.assets.loader import load_prompt_asset
from runner.prompt_library.bindings.binding_references import (
    AssemblyBindingReference,
    AssetBindingReference,
)
from runner.prompt_library.bindings.resolver import _resolve_binding_ids
from runner.prompt_library.instantiations.recorder import _record_prompt_instantiation
from runner.prompt_library.registry import prompt_registry
from runner.prompt_library.rendering.contract_rendering import render_contract_text
from runner.prompt_library.rendering.context import build_render_context
from runner.prompt_library.rendering.interpolation import render_template
from runner.prompt_library.rendering.overlays import (
    fresh_recovery_overlay_asset_id,
    operator_injection_overlay_asset_id,
)
from runner.prompt_library.rendering.prompt_rendering import render_prompt_text


@dataclass(frozen=True)
class PreparedPromptTurn:
    contract_asset_id: str
    prompt_asset_id: str | None
    prompt_assembly_id: str | None
    rendered_prompt: str
    contract_text: str
    delivery_prompt: str


def prepare_prompt_turn(
    config: dict[str, Any],
    projection: dict[str, Any],
    config_path: Path,
    projection_path: Path,
    event_log_path: Path,
    expected_turn_instance_id: str | None = None,
) -> PreparedPromptTurn:
    return _prepare_prompt_turn_internal(
        config,
        projection,
        config_path,
        projection_path,
        event_log_path,
        expected_turn_instance_id=expected_turn_instance_id,
    )


def _prepare_prompt_turn_internal(
    config: dict[str, Any],
    projection: dict[str, Any],
    config_path: Path,
    projection_path: Path,
    event_log_path: Path,
    *,
    prompt_template_override: AssemblyBindingReference | None = None,
    contract_template_override: AssetBindingReference | None = None,
    prompt_binding_override: AssetBindingReference | AssemblyBindingReference | None = None,
    expected_turn_instance_id: str | None = None,
    record_run_id: str | None = None,
    record_turn_instance_id: str | None = None,
    context_updates: dict[str, Any] | None = None,
) -> PreparedPromptTurn:
    phase = resolved_phase_context(config, projection)
    turn = current_turn_required(projection)
    registry = prompt_registry(config)
    context = build_render_context(config, projection, config_path, projection_path, event_log_path, phase, turn)
    context["current_turn_instance_id"] = expected_turn_instance_id
    if isinstance(context_updates, dict):
        context.update(context_updates)
    contract_asset_id, prompt_asset_id, prompt_assembly_id = _resolve_binding_ids(
        config,
        phase,
        turn,
        prompt_template_override=prompt_template_override,
        contract_template_override=contract_template_override,
        prompt_binding_override=prompt_binding_override or active_prompt_override(projection, phase, turn),
    )
    contract_text = render_contract_text(registry, contract_asset_id, context)
    context["contract"] = contract_text
    rendered_prompt = render_overlay_prefix(registry, projection, context) + render_prompt_text(
        registry,
        prompt_asset_id,
        prompt_assembly_id,
        context,
    )
    prepared = PreparedPromptTurn(
        contract_asset_id=contract_asset_id,
        prompt_asset_id=prompt_asset_id,
        prompt_assembly_id=prompt_assembly_id,
        rendered_prompt=rendered_prompt,
        contract_text=contract_text,
        delivery_prompt=append_turn_instance_requirement(rendered_prompt, expected_turn_instance_id),
    )
    if record_run_id is not None and record_turn_instance_id is not None:
        _record_prompt_instantiation(
            record_run_id,
            record_turn_instance_id,
            registry,
            prepared,
        )
    return prepared


def render_prompt(
    config: dict[str, Any],
    projection: dict[str, Any],
    config_path: Path,
    projection_path: Path,
    event_log_path: Path,
    expected_turn_instance_id: str | None = None,
) -> str:
    prepared = prepare_prompt_turn(
        config,
        projection,
        config_path,
        projection_path,
        event_log_path,
        expected_turn_instance_id=expected_turn_instance_id,
    )
    return prepared.delivery_prompt


def append_turn_instance_requirement(rendered_prompt: str, turn_instance_id: str | None) -> str:
    if not turn_instance_id:
        return rendered_prompt
    return (
        rendered_prompt
        + "\n\nRunner turn instance id: "
        + turn_instance_id
        + "\nYour RUNNER_EVENT payload must include exactly "
        + json_turn_instance_snippet(turn_instance_id)
        + "\n"
    )


def render_overlay_prefix(registry, projection: dict[str, Any], context: dict[str, Any]) -> str:
    overlay_ids = [
        overlay_asset_id
        for overlay_asset_id in (
            fresh_recovery_overlay_asset_id(projection),
            operator_injection_overlay_asset_id(projection),
        )
        if overlay_asset_id is not None
    ]
    rendered_overlays = []
    for overlay_asset_id in overlay_ids:
        overlay_markdown = load_prompt_asset(registry, overlay_asset_id)
        rendered_overlays.append(render_template(overlay_markdown, context))
    if not rendered_overlays:
        return ""
    return "\n".join(rendered_overlays).rstrip() + "\n\n"


def json_turn_instance_snippet(turn_instance_id: str) -> str:
    return f'"turn_instance_id":"{turn_instance_id}"'


def current_phase_required(projection: dict[str, Any]) -> dict[str, Any]:
    current = projection.get("current")
    if not isinstance(current, dict):
        raise ValueError("current phase is not set")
    for phase in projection["phases"]:
        if phase["id"] == current["phase"]:
            return phase
    raise ValueError("current phase is not set")


def resolved_phase_context(config: dict[str, Any], projection: dict[str, Any]) -> dict[str, Any]:
    phase_state = current_phase_required(projection)
    for phase in config.get("phases", []):
        if phase.get("id") == phase_state["id"]:
            return {**phase, **phase_state}
    return phase_state


def current_turn_required(projection: dict[str, Any]) -> str:
    current = projection.get("current")
    if isinstance(current, dict) and isinstance(current.get("turn"), str):
        return current["turn"]
    raise ValueError("current turn is not set")


def active_prompt_override(
    projection: dict[str, Any],
    phase: dict[str, Any],
    turn: str,
) -> AssetBindingReference | AssemblyBindingReference | None:
    phase_key = phase.get("phase_key") or f"phase_{phase['id']}"
    overrides = projection.get("prompt_overrides")
    if not isinstance(overrides, list):
        return None
    for override in reversed(overrides):
        if not isinstance(override, dict) or override.get("phase_key") != phase_key:
            continue
        override_turn = override.get("turn")
        if override_turn is not None and override_turn != turn:
            continue
        binding = override.get("binding")
        if not isinstance(binding, dict):
            continue
        if isinstance(binding.get("asset_id"), str):
            return AssetBindingReference(binding["asset_id"])
        if isinstance(binding.get("assembly_id"), str):
            return AssemblyBindingReference(binding["assembly_id"])
    return None

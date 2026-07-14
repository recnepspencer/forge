from __future__ import annotations

import json
from pathlib import Path
from typing import Any

from runner.authority.run_identity import RuntimePaths
from runner.phase_programs.policy_bindings import outcome_repair_policy_for_failure_family
from runner.prompt_library.bindings.binding_references import AssemblyBindingReference, AssetBindingReference
from runner.prompt_library.rendering.turn_preparation import PreparedPromptTurn, _prepare_prompt_turn_internal

RECOVERY_CONTRACT_ASSET = AssetBindingReference(asset_id="contracts/default")
RECOVERY_ASSEMBLY = AssemblyBindingReference(assembly_id="recovery/recovery_turn")
MISSING_EVENT_REPAIR_ASSEMBLY = AssemblyBindingReference(assembly_id="recovery/missing_runner_event_repair")


def prepare_execution_prompt_turn(
    config: dict[str, Any],
    projection: dict[str, Any],
    config_path: Path,
    projection_path: Path,
    event_log_path: Path,
    *,
    prompt_template_override: AssemblyBindingReference | None,
    expected_turn_instance_id: str,
    record_run_id: str,
    record_turn_instance_id: str,
) -> PreparedPromptTurn:
    return _prepare_prompt_turn_internal(
        config,
        projection,
        config_path,
        projection_path,
        event_log_path,
        prompt_template_override=prompt_template_override,
        expected_turn_instance_id=expected_turn_instance_id,
        record_run_id=record_run_id,
        record_turn_instance_id=record_turn_instance_id,
    )


def prepare_recovery_prompt_turn(
    config: dict[str, Any],
    projection: dict[str, Any],
    paths: RuntimePaths,
    reason: str,
    turn_instance_id: str | None,
    *,
    failure_family: str | None = None,
    recovery_kind: str = "escalation_recovery",
    recovery_route_guidance: str = "",
    prompt_override: str | None = None,
    supported_event_types: frozenset[str] = frozenset(),
) -> PreparedPromptTurn:
    prompt_artifact_path = ""
    contract_artifact_path = ""
    if isinstance(turn_instance_id, str) and turn_instance_id:
        instantiation_root = paths.instantiations / turn_instance_id
        prompt_path = instantiation_root / "prompt.md"
        contract_path = instantiation_root / "contract.md"
        if prompt_path.exists():
            prompt_artifact_path = str(prompt_path.resolve())
        if contract_path.exists():
            contract_artifact_path = str(contract_path.resolve())
    return _prepare_prompt_turn_internal(
        config,
        projection,
        Path(projection["config_path"]),
        paths.projection,
        paths.events,
        contract_template_override=RECOVERY_CONTRACT_ASSET,
        prompt_binding_override=recovery_prompt_binding_for_failure(
            config, failure_family, recovery_kind, prompt_override
        ),
        record_run_id=projection["run_id"],
        record_turn_instance_id=recovery_prompt_instantiation_id(turn_instance_id),
        context_updates={
            "current_cursor": current_cursor_text(projection),
            "failure_reason": reason,
            "expected_turn_instance_id": turn_instance_id or "",
            "artifact_block": recovery_artifact_block(prompt_artifact_path, contract_artifact_path),
            "recovery_route_guidance": recovery_route_guidance,
            "expected_runner_event_markers": expected_runner_event_markers(
                supported_event_types, turn_instance_id
            ),
        },
    )


def build_recovery_prompt(
    config: dict[str, Any],
    projection: dict[str, Any],
    paths: RuntimePaths,
    reason: str,
    turn_instance_id: str | None,
    *,
    failure_family: str | None = None,
    recovery_kind: str = "escalation_recovery",
    recovery_route_guidance: str = "",
    prompt_override: str | None = None,
    supported_event_types: frozenset[str] = frozenset(),
) -> str:
    prepared = prepare_recovery_prompt_turn(
        config,
        projection,
        paths,
        reason,
        turn_instance_id,
        failure_family=failure_family,
        recovery_kind=recovery_kind,
        recovery_route_guidance=recovery_route_guidance,
        prompt_override=prompt_override,
        supported_event_types=supported_event_types,
    )
    return prepared.rendered_prompt


def recovery_prompt_binding_for_failure(
    config: dict[str, Any],
    failure_family: str | None,
    recovery_kind: str,
    prompt_override: str | None = None,
) -> AssemblyBindingReference:
    if isinstance(prompt_override, str) and prompt_override:
        # A stage may name its own escalated recovery prompt (e.g. one that
        # explains the escalation) instead of the default recovery assembly.
        return AssemblyBindingReference(assembly_id=prompt_override)
    policy = outcome_repair_policy_for_failure_family(config, failure_family)
    if (
        recovery_kind == "outcome_repair"
        and policy is not None
        and policy.first_attempt == "same_agent_event_repair_prompt"
    ):
        return MISSING_EVENT_REPAIR_ASSEMBLY
    return RECOVERY_ASSEMBLY


def recovery_artifact_block(prompt_artifact_path: str, contract_artifact_path: str) -> str:
    artifact_lines: list[str] = []
    if prompt_artifact_path:
        artifact_lines.append(f"Prompt artifact: {prompt_artifact_path}")
    if contract_artifact_path:
        artifact_lines.append(f"Contract artifact: {contract_artifact_path}")
    return "\n".join(artifact_lines) + ("\n" if artifact_lines else "")


def recovery_prompt_instantiation_id(turn_instance_id: str | None) -> str:
    base = turn_instance_id or "recovery"
    return f"{base}-recovery"


def current_cursor_text(projection: dict[str, Any]) -> str:
    current = projection.get("current")
    if not isinstance(current, dict):
        return "not set"
    return f"phase {current.get('phase')}, turn {current.get('turn')}"


def expected_runner_event_markers(
    supported_event_types: frozenset[str], turn_instance_id: str | None
) -> str:
    if not supported_event_types:
        raise ValueError("recovery prompt requires at least one supported runner outcome")
    payload = {"turn_instance_id": turn_instance_id or ""}
    return "\n".join(
        "RUNNER_EVENT: "
        + json.dumps(
            {"event_type": event_type, "payload": payload},
            separators=(",", ":"),
        )
        for event_type in sorted(supported_event_types)
    )

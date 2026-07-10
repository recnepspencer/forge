from __future__ import annotations

from dataclasses import dataclass
from typing import Any

from runner.prompt_library.bindings.binding_references import (
    AssemblyBindingReference,
    parse_assembly_binding_reference,
)
from runner.roles.model_policy import RoleModelPolicySeed
from runner.roles.role_ids import (
    IMPLEMENTER_ROLE_ID,
    REVIEWER_ROLE_ID,
    require_supported_role_id,
)
from runner.roles.session_policy import RoleSessionPolicySeed


@dataclass(frozen=True)
class RoleDefinition:
    role_id: str
    execution_purpose: str


@dataclass(frozen=True)
class TurnRoleBinding:
    turn: str
    role: RoleDefinition
    model_policy_seed: RoleModelPolicySeed
    session_policy_seed: RoleSessionPolicySeed
    prompt_template_override: AssemblyBindingReference | None


DEFAULT_ROLE_REGISTRY = {
    IMPLEMENTER_ROLE_ID: RoleDefinition(
        role_id=IMPLEMENTER_ROLE_ID,
        execution_purpose="carry implementation and repair turns",
    ),
    REVIEWER_ROLE_ID: RoleDefinition(
        role_id=REVIEWER_ROLE_ID,
        execution_purpose="carry review-family turns",
    ),
}


def resolve_turn_role_binding(
    config: dict[str, Any],
    phase_id: int,
    turn: str,
) -> TurnRoleBinding:
    phase = configured_phase(config, phase_id)
    bindings = phase.get("role_bindings")
    if not isinstance(bindings, dict):
        raise ValueError(f"phase {phase_id} is missing role_bindings")
    binding = bindings.get(turn)
    if not isinstance(binding, dict):
        raise ValueError(f"phase {phase_id} role_bindings.{turn} is required")

    role_id = require_supported_role_id(
        binding.get("role_id"),
        f"phase {phase_id} role_bindings.{turn}.role_id",
    )
    role = DEFAULT_ROLE_REGISTRY.get(role_id)
    if role is None:
        raise ValueError(f"phase {phase_id} role_bindings.{turn}.role_id must be an admitted role")

    model_policy = binding.get("model_policy")
    if not isinstance(model_policy, dict):
        raise ValueError(f"phase {phase_id} role_bindings.{turn}.model_policy must be an object")
    session_policy = binding.get("session_policy")
    if not isinstance(session_policy, dict):
        raise ValueError(f"phase {phase_id} role_bindings.{turn}.session_policy must be an object")

    return TurnRoleBinding(
        turn=turn,
        role=role,
        model_policy_seed=RoleModelPolicySeed.from_mapping(
            merged_seed_mapping(config.get("session_defaults", {}), model_policy),
            f"phase {phase_id} role_bindings.{turn}.model_policy",
        ),
        session_policy_seed=RoleSessionPolicySeed.from_mapping(
            merged_seed_mapping(config.get("session_defaults", {}), session_policy),
            f"phase {phase_id} role_bindings.{turn}.session_policy",
        ),
        prompt_template_override=parse_role_prompt_override(binding, phase_id, turn),
    )


def configured_phase(config: dict[str, Any], phase_id: int) -> dict[str, Any]:
    for phase in config.get("phases", []):
        if phase.get("id") == phase_id:
            return phase
    raise ValueError(f"unknown phase id {phase_id}")


def merged_seed_mapping(session_defaults: dict[str, Any], override: Any) -> dict[str, Any]:
    seed = dict(session_defaults) if isinstance(session_defaults, dict) else {}
    if isinstance(override, dict):
        seed.update(override)
    return seed


def parse_role_prompt_override(binding: dict[str, Any], phase_id: int, turn: str) -> AssemblyBindingReference | None:
    prompt_template = binding.get("prompt_template")
    if prompt_template is None:
        return None
    return parse_assembly_binding_reference(
        prompt_template,
        f"phase {phase_id} role_bindings.{turn}.prompt_template",
    )

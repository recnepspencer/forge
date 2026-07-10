from __future__ import annotations

from dataclasses import dataclass, replace
from typing import Any

from runner.prompt_library.bindings.binding_references import AssemblyBindingReference
from runner.roles.handoff_policy import RoleHandoffPolicy, handoff_policy_for_role
from runner.roles.model_policy import RoleModelPolicy, RoleModelPolicySeed, role_model_policy_from_seed
from runner.roles.registry import RoleDefinition, resolve_turn_role_binding
from runner.roles.session_policy import (
    RoleSessionPolicy,
    role_session_policy_from_seed,
    session_state_for_execution,
)


@dataclass(frozen=True)
class ResolvedRolePolicy:
    role: RoleDefinition
    model_policy: RoleModelPolicy
    session_policy: RoleSessionPolicy
    handoff_policy: RoleHandoffPolicy
    prompt_template_override: AssemblyBindingReference | None

    def execution_session(self, projection_session: dict[str, Any]) -> dict[str, Any]:
        force_fresh_session = bool(projection_session.get("fresh_recovery"))
        execution_session = session_state_for_execution(
            projection_session,
            self.session_policy,
            force_fresh_session or self.handoff_policy.force_fresh_session_on_escalation,
        )
        execution_session["provider"] = self.model_policy.provider
        execution_session["command"] = self.model_policy.command
        execution_session["command_args"] = list(self.model_policy.command_args)
        execution_session["model"] = self.model_policy.model
        execution_session["reasoning_effort"] = self.model_policy.reasoning_effort
        execution_session["config"] = dict(self.model_policy.config)
        execution_session["env"] = dict(self.model_policy.env)
        execution_session["role_id"] = self.role.role_id
        execution_session["session_family"] = self.session_policy.continuity_family
        execution_session["escalation_posture"] = self.handoff_policy.escalation_posture
        return execution_session


def resolve_role_policy(
    config: dict[str, Any],
    phase_id: int,
    turn: str,
) -> ResolvedRolePolicy:
    binding = resolve_turn_role_binding(config, phase_id, turn)
    model_policy = role_model_policy_from_seed(binding.model_policy_seed)
    session_policy = role_session_policy_from_seed(binding.session_policy_seed)
    handoff_policy = handoff_policy_for_role(binding.role)
    return ResolvedRolePolicy(
        role=binding.role,
        model_policy=model_policy,
        session_policy=session_policy,
        handoff_policy=handoff_policy,
        prompt_template_override=binding.prompt_template_override,
    )


def active_model_override(
    projection: dict[str, Any],
    phase_id: int,
    turn: str,
) -> dict[str, Any] | None:
    """The most recent escalation-activated model override that covers this turn.

    Phase-scoped overrides apply only to their phase; run-scoped overrides apply
    everywhere from the point they were activated. The override is authority
    derived from `model_escalation_activated` events, not from static config.
    """
    match = None
    for override in projection.get("model_overrides", []) or []:
        if turn not in override.get("turns", []):
            continue
        if override.get("scope") == "run" or override.get("phase_id") == phase_id:
            match = override
    return match


def apply_model_override(
    role_policy: ResolvedRolePolicy,
    projection: dict[str, Any],
    phase_id: int,
    turn: str,
) -> ResolvedRolePolicy:
    override = active_model_override(projection, phase_id, turn)
    if override is None:
        return role_policy
    seed = RoleModelPolicySeed.from_mapping(override["model_policy"], "model_override.model_policy")
    return replace(role_policy, model_policy=role_model_policy_from_seed(seed))


def project_current_session(config: dict[str, Any], current: dict[str, Any] | None, session: dict[str, Any]) -> dict[str, Any]:
    if not isinstance(current, dict):
        return session
    resolved_policy = resolve_role_policy(config, current["phase"], current["turn"])
    return resolved_policy.execution_session(session)

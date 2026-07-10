from __future__ import annotations

from dataclasses import dataclass, field
from typing import Any

from runner.authority.config.schema import SUPPORTED_PROVIDERS


@dataclass(frozen=True)
class RoleModelPolicySeed:
    provider: str
    model: str
    command: str | None = None
    command_args: tuple[str, ...] = ()
    reasoning_effort: str | None = None
    config: dict[str, Any] = field(default_factory=dict)
    env: dict[str, str] = field(default_factory=dict)

    @classmethod
    def from_mapping(cls, seed: dict[str, Any], field_name: str) -> "RoleModelPolicySeed":
        provider = seed.get("provider", "codex")
        if provider not in SUPPORTED_PROVIDERS:
            raise ValueError(f"{field_name}.provider must be one of {sorted(SUPPORTED_PROVIDERS)}")
        model = seed.get("model")
        if not isinstance(model, str) or not model:
            raise ValueError(f"{field_name}.model is required")
        command = seed.get("command")
        if command is not None and (not isinstance(command, str) or not command):
            raise ValueError(f"{field_name}.command must be a non-empty string when present")
        command_args = seed.get("command_args", [])
        if command_args is not None and (
            not isinstance(command_args, list)
            or not all(isinstance(item, str) and item for item in command_args)
        ):
            raise ValueError(f"{field_name}.command_args must be an array of non-empty strings")
        reasoning_effort = seed.get("reasoning_effort")
        if provider == "codex" and (not isinstance(reasoning_effort, str) or not reasoning_effort):
            raise ValueError(f"{field_name}.reasoning_effort is required for codex provider")
        if reasoning_effort is not None and (not isinstance(reasoning_effort, str) or not reasoning_effort):
            raise ValueError(f"{field_name}.reasoning_effort must be a non-empty string when present")
        config_map = seed.get("config", {})
        if not isinstance(config_map, dict):
            raise ValueError(f"{field_name}.config must be an object")
        env_map = seed.get("env", {})
        if not isinstance(env_map, dict) or not all(
            isinstance(key, str) and isinstance(value, str) for key, value in env_map.items()
        ):
            raise ValueError(f"{field_name}.env must be an object with string keys and values")
        return cls(
            provider=provider,
            model=model,
            command=command if isinstance(command, str) and command else None,
            command_args=tuple(command_args) if isinstance(command_args, list) else (),
            reasoning_effort=reasoning_effort if isinstance(reasoning_effort, str) and reasoning_effort else None,
            config=dict(config_map),
            env=dict(env_map),
        )


@dataclass(frozen=True)
class RoleModelPolicy:
    provider: str
    command: str | None
    command_args: tuple[str, ...]
    model: str
    reasoning_effort: str | None
    config: dict[str, Any]
    env: dict[str, str]


def role_model_policy_from_seed(model_policy_seed: RoleModelPolicySeed) -> RoleModelPolicy:
    return RoleModelPolicy(
        provider=model_policy_seed.provider,
        command=model_policy_seed.command,
        command_args=model_policy_seed.command_args,
        model=model_policy_seed.model,
        reasoning_effort=model_policy_seed.reasoning_effort,
        config=dict(model_policy_seed.config),
        env=dict(model_policy_seed.env),
    )


def validate_role_model_policy_binding(model_policy: dict[str, Any], errors: list[str], field_name: str) -> None:
    provider = model_policy.get("provider")
    if not isinstance(provider, str) or provider not in SUPPORTED_PROVIDERS:
        errors.append(f"{field_name}.provider must be one of {sorted(SUPPORTED_PROVIDERS)}")

    model = model_policy.get("model")
    if not isinstance(model, str) or not model:
        errors.append(f"{field_name}.model is required")

    effort = model_policy.get("reasoning_effort")
    if provider == "codex" and (not isinstance(effort, str) or not effort):
        errors.append(f"{field_name}.reasoning_effort is required for codex provider")
    if effort is not None and (not isinstance(effort, str) or not effort):
        errors.append(f"{field_name}.reasoning_effort must be a non-empty string when present")


def require_role_model_policy_binding(model_policy: dict[str, Any], field_name: str) -> None:
    errors: list[str] = []
    validate_role_model_policy_binding(model_policy, errors, field_name)
    if errors:
        raise ValueError(errors[0])


def validate_model_policy_seed(session_defaults: dict[str, Any], errors: list[str], field_name: str = "session_defaults") -> None:
    try:
        RoleModelPolicySeed.from_mapping(session_defaults, field_name)
    except ValueError as error:
        errors.append(str(error))

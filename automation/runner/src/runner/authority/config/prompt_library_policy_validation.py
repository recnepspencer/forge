from __future__ import annotations

from typing import Any


def validate_prompt_library_policy(policy: dict[str, Any], errors: list[str]) -> None:
    for key in (
        "runner_asset_roots",
        "runner_assembly_roots",
        "consumer_asset_roots",
        "consumer_assembly_roots",
    ):
        validate_root_list(policy, key, errors)

    for key in ("allow_consumer_prompts", "allow_direct_file_binding"):
        value = policy.get(key)
        if not isinstance(value, bool):
            errors.append(f"prompt_library_policy.{key} must be a boolean")
    if policy.get("allow_direct_file_binding") is True:
        errors.append("prompt_library_policy.allow_direct_file_binding must remain false")


def validate_root_list(policy: dict[str, Any], key: str, errors: list[str]) -> None:
    values = policy.get(key)
    if not isinstance(values, list) or not values:
        errors.append(f"prompt_library_policy.{key} must be a non-empty list")
        return
    for index, value in enumerate(values):
        if not isinstance(value, str) or not value:
            errors.append(f"prompt_library_policy.{key}[{index}] must be a non-empty string")

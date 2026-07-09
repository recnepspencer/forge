from __future__ import annotations

from pathlib import Path
from typing import Any


DEFAULT_RUNNER_ASSET_ROOT = "automation/runner/prompts/assets"
DEFAULT_RUNNER_ASSEMBLY_ROOT = "automation/runner/prompts/assemblies"
DEFAULT_CONSUMER_ASSET_ROOT = "automation/project_prompts/assets"
DEFAULT_CONSUMER_ASSEMBLY_ROOT = "automation/project_prompts/assemblies"


def prompt_root_policy(config: dict[str, Any]) -> dict[str, object]:
    project_cwd = Path(config.get("project", {}).get("cwd", ".")).resolve()
    policy = config.get("prompt_library_policy", {})
    return {
        "runner_asset_roots": resolve_root_list(
            project_cwd,
            policy.get("runner_asset_roots", [DEFAULT_RUNNER_ASSET_ROOT]),
        ),
        "runner_assembly_roots": resolve_root_list(
            project_cwd,
            policy.get("runner_assembly_roots", [DEFAULT_RUNNER_ASSEMBLY_ROOT]),
        ),
        "consumer_asset_roots": resolve_root_list(
            project_cwd,
            policy.get("consumer_asset_roots", [DEFAULT_CONSUMER_ASSET_ROOT]),
        ),
        "consumer_assembly_roots": resolve_root_list(
            project_cwd,
            policy.get("consumer_assembly_roots", [DEFAULT_CONSUMER_ASSEMBLY_ROOT]),
        ),
        "allow_consumer_prompts": bool(policy.get("allow_consumer_prompts", True)),
        "allow_direct_file_binding": bool(policy.get("allow_direct_file_binding", False)),
    }


def resolve_root_list(project_cwd: Path, values: list[str]) -> tuple[Path, ...]:
    resolved: list[Path] = []
    for value in values:
        path = Path(value)
        if not path.is_absolute():
            path = project_cwd / path
        resolved.append(path.resolve())
    return tuple(resolved)

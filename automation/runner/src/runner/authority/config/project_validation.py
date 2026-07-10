from __future__ import annotations

from pathlib import Path
from typing import Any

from runner.authority.config.paths import resolve_project_path


def validate_project_section(project: dict[str, Any], errors: list[str]) -> None:
    cwd = project.get("cwd")
    if not isinstance(cwd, str) or not cwd:
        errors.append("project.cwd is required")
    elif not Path(cwd).exists():
        errors.append(f"project.cwd does not exist: {cwd}")

    name = project.get("name")
    if not isinstance(name, str) or not name:
        errors.append("project.name is required")

    spec_file = project.get("spec_file")
    if not isinstance(spec_file, str) or not spec_file:
        errors.append("project.spec_file is required")
    else:
        spec_path = resolve_project_path(project, spec_file)
        if not spec_path.exists():
            errors.append(f"project.spec_file does not exist: {spec_path}")

    context_files = project.get("context_files")
    if not isinstance(context_files, list) or not context_files:
        errors.append("project.context_files must be a non-empty list")
        return

    for context_file in context_files:
        if not isinstance(context_file, str) or not context_file:
            errors.append("project.context_files entries must be non-empty strings")
            continue
        context_path = resolve_project_path(project, context_file)
        if not context_path.exists():
            errors.append(f"project.context_files entry does not exist: {context_path}")

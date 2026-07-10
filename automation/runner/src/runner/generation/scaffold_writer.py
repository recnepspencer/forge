from __future__ import annotations

import json
from pathlib import Path

from runner.authority.run_identity import CANONICAL_RUNTIME_ROOT
from runner.generation.scaffold_templates import prompt_readme, scaffold_config
from runner.generation.scaffold_types import ScaffoldRequest, ScaffoldResult


def generate_scaffold(request: ScaffoldRequest) -> ScaffoldResult:
    if request.project_root.resolve().is_relative_to(CANONICAL_RUNTIME_ROOT.resolve()):
        raise ValueError("generation project root may not be inside runtime authority")
    config_path = request.project_root / "automation" / "runner" / "config" / f"{request.name}.json"
    prompt_root = request.project_root / "automation" / "project_prompts"
    protected = request.project_root / "automation" / "runner" / "runtime"
    if protected.exists() and config_path.is_relative_to(protected):
        raise ValueError("generation may not target runtime authority")
    if config_path.exists() and not request.force:
        raise FileExistsError(f"scaffold already exists: {config_path}")
    config_path.parent.mkdir(parents=True, exist_ok=True)
    prompt_root.mkdir(parents=True, exist_ok=True)
    (prompt_root / "assets" / "contracts").mkdir(parents=True, exist_ok=True)
    (prompt_root / "assets" / "recovery").mkdir(parents=True, exist_ok=True)
    (prompt_root / "assets" / "turns").mkdir(parents=True, exist_ok=True)
    (prompt_root / "assemblies" / "turns").mkdir(parents=True, exist_ok=True)
    config_path.write_text(json.dumps(scaffold_config(request), indent=2) + "\n", encoding="utf-8")
    (prompt_root / "README.md").write_text(prompt_readme(request), encoding="utf-8")
    (prompt_root / "assets" / "contracts" / "default.md").write_text("# Contract\n", encoding="utf-8")
    (prompt_root / "assets" / "recovery" / "fresh_session_overlay.md").write_text(
        fresh_session_overlay_prompt(),
        encoding="utf-8",
    )
    (prompt_root / "assets" / "recovery" / "operator_injection_overlay.md").write_text(
        operator_injection_overlay_prompt(),
        encoding="utf-8",
    )
    (prompt_root / "assets" / "turns" / "default.md").write_text("# Turn\n", encoding="utf-8")
    (prompt_root / "assemblies" / "turns" / "default.json").write_text('{"kind":"assembly","parts":[{"asset_id":"turns/default"}]}\n', encoding="utf-8")
    return ScaffoldResult(config_path, prompt_root)


def fresh_session_overlay_prompt() -> str:
    return "\n".join(
        (
            "Fresh recovery session context:",
            "",
            "Reason: {fresh_recovery.reason}",
            "Cycle count: {fresh_recovery.cycle_count} of {fresh_recovery.threshold}",
            "",
            "Rebuild context from the spec, projection, event log, and current phase before continuing.",
            "",
        )
    )


def operator_injection_overlay_prompt() -> str:
    return "\n".join(
        (
            "Operator injection for this active run:",
            "",
            "{operator_intervention.reason}",
            "",
            "Treat this as authoritative runner direction for the next turn only.",
            "",
        )
    )

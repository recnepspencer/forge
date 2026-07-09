from __future__ import annotations

from typing import Any

from runner.prompt_library.assemblies.composition import render_prompt_assembly
from runner.prompt_library.assemblies.loader import load_prompt_assembly
from runner.prompt_library.assets.loader import load_prompt_asset
from runner.prompt_library.rendering.interpolation import render_template


def render_prompt_text(
    registry,
    prompt_asset_id: str | None,
    prompt_assembly_id: str | None,
    context: dict[str, Any],
) -> str:
    if prompt_asset_id is not None:
        prompt_markdown = load_prompt_asset(registry, prompt_asset_id)
        return render_template(prompt_markdown, context)
    if prompt_assembly_id is None:
        raise ValueError("prompt binding must name either prompt_asset_id or prompt_assembly_id")
    part_asset_ids = load_prompt_assembly(registry, prompt_assembly_id)
    parts = render_prompt_assembly(registry, part_asset_ids)
    return "\n\n".join(render_template(part, context) for part in parts)

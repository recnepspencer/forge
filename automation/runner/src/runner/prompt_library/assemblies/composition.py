from __future__ import annotations

from runner.prompt_library.assets.loader import load_prompt_asset


def render_prompt_assembly(registry, part_asset_ids: tuple[str, ...]) -> tuple[str, ...]:
    parts: list[str] = []
    for part_asset_id in part_asset_ids:
        markdown = load_prompt_asset(registry, part_asset_id)
        parts.append(markdown)
    return tuple(parts)

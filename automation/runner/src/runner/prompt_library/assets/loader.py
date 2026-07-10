from __future__ import annotations

from runner.prompt_library.assets.metadata import strip_asset_frontmatter


def load_prompt_asset(registry, asset_id: str) -> str:
    asset_record = registry._resolve_asset_record(asset_id)
    markdown = asset_record.source_path.read_text(encoding="utf-8-sig")
    return strip_asset_frontmatter(markdown)

from __future__ import annotations

from runner.prompt_library.assets.loader import load_prompt_asset
from runner.prompt_library.rendering.interpolation import render_template


def render_contract_text(
    registry,
    contract_asset_id: str,
    context: dict,
) -> str:
    contract_markdown = load_prompt_asset(registry, contract_asset_id)
    return render_template(contract_markdown, context)

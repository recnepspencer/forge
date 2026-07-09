from __future__ import annotations

import json

from runner.prompt_library.registry_types import PromptAssemblyPart


def load_prompt_assembly(registry, assembly_id: str) -> tuple[str, ...]:
    assembly_record = registry._resolve_assembly_record(assembly_id)
    payload = json.loads(assembly_record.source_path.read_text(encoding="utf-8-sig"))
    parts = payload.get("parts")
    if not isinstance(parts, list) or not parts:
        raise ValueError(f"assembly {assembly_record.assembly_id!r} must contain a non-empty parts list")
    return tuple(parse_part(assembly_record.assembly_id, item).asset_id for item in parts)


def parse_part(assembly_id: str, value: object) -> PromptAssemblyPart:
    if not isinstance(value, dict):
        raise ValueError(f"assembly {assembly_id!r} contains a non-object part")
    asset_id = value.get("asset_id")
    if not isinstance(asset_id, str) or not asset_id:
        raise ValueError(f"assembly {assembly_id!r} contains a part without asset_id")
    return _new_prompt_assembly_part(asset_id)


def _new_prompt_assembly_part(asset_id: str) -> PromptAssemblyPart:
    part = object.__new__(PromptAssemblyPart)
    object.__setattr__(part, "asset_id", asset_id)
    return part

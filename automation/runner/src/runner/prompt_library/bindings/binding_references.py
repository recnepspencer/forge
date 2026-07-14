from __future__ import annotations

from dataclasses import dataclass
from typing import Any


@dataclass(frozen=True)
class AssetBindingReference:
    asset_id: str


@dataclass(frozen=True)
class AssemblyBindingReference:
    assembly_id: str


def parse_asset_binding_reference(value: Any, field_name: str) -> AssetBindingReference:
    if not isinstance(value, dict):
        raise ValueError(f"{field_name} must be an object with asset_id")
    asset_id = value.get("asset_id")
    if set(value.keys()) != {"asset_id"} or not isinstance(asset_id, str) or not asset_id:
        raise ValueError(f"{field_name} must be an object with a non-empty asset_id")
    return AssetBindingReference(asset_id=asset_id)


def parse_assembly_binding_reference(value: Any, field_name: str) -> AssemblyBindingReference:
    if not isinstance(value, dict):
        raise ValueError(f"{field_name} must be an object with assembly_id")
    assembly_id = value.get("assembly_id")
    if set(value.keys()) != {"assembly_id"} or not isinstance(assembly_id, str) or not assembly_id:
        raise ValueError(f"{field_name} must be an object with a non-empty assembly_id")
    return AssemblyBindingReference(assembly_id=assembly_id)

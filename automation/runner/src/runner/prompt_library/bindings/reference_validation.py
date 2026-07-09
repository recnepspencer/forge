from __future__ import annotations

from typing import Any

from runner.prompt_library.bindings.binding_references import (
    parse_asset_binding_reference,
    parse_assembly_binding_reference,
)
from runner.prompt_library.registry import prompt_registry


def validate_asset_binding_reference(
    config: dict[str, Any],
    value: Any,
    field_name: str,
    errors: list[str],
) -> None:
    try:
        binding = parse_asset_binding_reference(value, field_name)
        prompt_registry(config).asset_details(binding.asset_id)
    except Exception as error:
        errors.append(str(error))


def validate_assembly_binding_reference(
    config: dict[str, Any],
    value: Any,
    field_name: str,
    errors: list[str],
) -> None:
    try:
        binding = parse_assembly_binding_reference(value, field_name)
        prompt_registry(config).assembly_details(binding.assembly_id)
    except Exception as error:
        errors.append(str(error))

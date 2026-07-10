from __future__ import annotations

from typing import Any

from runner.prompt_library.bindings.binding_references import (
    AssemblyBindingReference,
    AssetBindingReference,
)
from runner.prompt_library.bindings.phase_bindings import (
    contract_binding_for_phase,
    prompt_binding_for_cursor,
)


def _resolve_binding_ids(
    config: dict[str, Any],
    phase: dict[str, Any],
    turn: str,
    *,
    prompt_template_override: AssemblyBindingReference | None,
    contract_template_override: AssetBindingReference | None,
    prompt_binding_override: AssetBindingReference | AssemblyBindingReference | None,
) -> tuple[str, str | None, str | None]:
    contract_binding = contract_template_override or contract_binding_for_phase(config, phase)
    prompt_binding = prompt_binding_override or prompt_binding_for_cursor(config, phase, turn)
    if prompt_template_override is not None:
        prompt_binding = prompt_template_override
    if isinstance(prompt_binding, AssetBindingReference):
        return contract_binding.asset_id, prompt_binding.asset_id, None
    return contract_binding.asset_id, None, prompt_binding.assembly_id


__all__: list[str] = []

from __future__ import annotations

import json
from typing import TYPE_CHECKING

from runner.authority.run_identity import RuntimePaths
from runner.prompt_library.instantiations.record import PromptInstantiationRecord

if TYPE_CHECKING:
    from runner.prompt_library.rendering.turn_preparation import PreparedPromptTurn


def _record_prompt_instantiation(
    run_id: str,
    turn_instance_id: str,
    registry,
    prepared: PreparedPromptTurn,
) -> PromptInstantiationRecord:
    instantiation_root = RuntimePaths(run_id).instantiations / turn_instance_id
    instantiation_root.mkdir(parents=True, exist_ok=True)
    prompt_path = instantiation_root / "prompt.md"
    contract_path = instantiation_root / "contract.md"
    record_path = instantiation_root / "record.json"
    prompt_path.write_text(prepared.rendered_prompt, encoding="utf-8")
    contract_path.write_text(prepared.contract_text, encoding="utf-8")
    contract_details = registry.asset_details(prepared.contract_asset_id)
    prompt_asset_details = (
        registry.asset_details(prepared.prompt_asset_id)
        if prepared.prompt_asset_id is not None
        else None
    )
    prompt_assembly_details = (
        registry.assembly_details(prepared.prompt_assembly_id)
        if prepared.prompt_assembly_id is not None
        else None
    )
    record = PromptInstantiationRecord(
        turn_instance_id=turn_instance_id,
        contract_asset_id=prepared.contract_asset_id,
        contract_root_kind=contract_details.root_kind,
        contract_source_path=str(contract_details.source_path),
        prompt_asset_id=prepared.prompt_asset_id,
        prompt_asset_root_kind=prompt_asset_details.root_kind if prompt_asset_details is not None else None,
        prompt_asset_source_path=str(prompt_asset_details.source_path) if prompt_asset_details is not None else None,
        prompt_assembly_id=prepared.prompt_assembly_id,
        prompt_assembly_root_kind=prompt_assembly_details.root_kind if prompt_assembly_details is not None else None,
        prompt_assembly_source_path=(
            str(prompt_assembly_details.source_path) if prompt_assembly_details is not None else None
        ),
        prompt_file=prompt_path.name,
        contract_file=contract_path.name,
    )
    record_path.write_text(json.dumps(record.as_json(), indent=2), encoding="utf-8")
    return record


__all__: list[str] = []

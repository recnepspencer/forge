from __future__ import annotations

from dataclasses import asdict, dataclass


@dataclass(frozen=True)
class PromptInstantiationRecord:
    turn_instance_id: str
    contract_asset_id: str
    contract_root_kind: str
    contract_source_path: str
    prompt_asset_id: str | None
    prompt_asset_root_kind: str | None
    prompt_asset_source_path: str | None
    prompt_assembly_id: str | None
    prompt_assembly_root_kind: str | None
    prompt_assembly_source_path: str | None
    prompt_file: str
    contract_file: str

    def as_json(self) -> dict:
        return asdict(self)

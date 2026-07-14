from __future__ import annotations

from pathlib import Path
from typing import Any

from runner.prompt_library.asset_roots import prompt_root_policy
from runner.prompt_library.registry_types import (
    PromptArtifactDetails,
    PromptAssemblyRecord,
    PromptAssetRecord,
)


def prompt_registry(config: dict) -> Any:
    root_policy = prompt_root_policy(config)

    class PromptRegistryState:
        def asset_details(self, asset_id: str) -> PromptArtifactDetails:
            asset_record = self._resolve_asset_record(asset_id)
            return PromptArtifactDetails(asset_record.root_kind, asset_record.source_path)

        def assembly_details(self, assembly_id: str) -> PromptArtifactDetails:
            assembly_record = self._resolve_assembly_record(assembly_id)
            return PromptArtifactDetails(assembly_record.root_kind, assembly_record.source_path)

        def _resolve_asset_record(self, asset_id: str) -> PromptAssetRecord:
            return resolve_unique_reference(
                asset_id,
                self._asset_candidates(asset_id),
                "asset",
            )

        def _resolve_assembly_record(self, assembly_id: str) -> PromptAssemblyRecord:
            return resolve_unique_reference(
                assembly_id,
                self._assembly_candidates(assembly_id),
                "assembly",
            )

        def _asset_candidates(self, asset_id: str) -> list[PromptAssetRecord]:
            candidates = asset_candidates(asset_id, root_policy["runner_asset_roots"], "runner")
            if root_policy["allow_consumer_prompts"]:
                candidates.extend(asset_candidates(asset_id, root_policy["consumer_asset_roots"], "consumer"))
            return candidates

        def _assembly_candidates(self, assembly_id: str) -> list[PromptAssemblyRecord]:
            candidates = assembly_candidates(assembly_id, root_policy["runner_assembly_roots"], "runner")
            if root_policy["allow_consumer_prompts"]:
                candidates.extend(assembly_candidates(assembly_id, root_policy["consumer_assembly_roots"], "consumer"))
            return candidates

    return PromptRegistryState()


def resolve_unique_reference(reference_id: str, candidates: list, kind: str):
    if not candidates:
        raise ValueError(f"unknown prompt {kind} id {reference_id!r}")
    if len(candidates) > 1:
        source_paths = sorted(str(candidate.source_path) for candidate in candidates)
        raise ValueError(f"ambiguous prompt {kind} id {reference_id!r}: {source_paths}")
    return candidates[0]


def asset_candidates(asset_id: str, roots: tuple[Path, ...], root_kind: str) -> list[PromptAssetRecord]:
    candidates: list[PromptAssetRecord] = []
    for root in roots:
        source_path = root / f"{asset_id}.md"
        if source_path.exists():
            candidates.append(_new_prompt_asset_record(asset_id, root_kind, source_path))
    return candidates


def assembly_candidates(assembly_id: str, roots: tuple[Path, ...], root_kind: str) -> list[PromptAssemblyRecord]:
    candidates: list[PromptAssemblyRecord] = []
    for root in roots:
        source_path = root / f"{assembly_id}.json"
        if source_path.exists():
            candidates.append(_new_prompt_assembly_record(assembly_id, root_kind, source_path, ()))
    return candidates


def _new_prompt_asset_record(asset_id: str, root_kind: str, source_path: Path) -> PromptAssetRecord:
    record = object.__new__(PromptAssetRecord)
    object.__setattr__(record, "asset_id", asset_id)
    object.__setattr__(record, "root_kind", root_kind)
    object.__setattr__(record, "source_path", source_path)
    return record


def _new_prompt_assembly_record(
    assembly_id: str,
    root_kind: str,
    source_path: Path,
    parts: tuple,
) -> PromptAssemblyRecord:
    record = object.__new__(PromptAssemblyRecord)
    object.__setattr__(record, "assembly_id", assembly_id)
    object.__setattr__(record, "root_kind", root_kind)
    object.__setattr__(record, "source_path", source_path)
    object.__setattr__(record, "parts", parts)
    return record

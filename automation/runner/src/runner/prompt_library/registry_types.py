from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path


@dataclass(frozen=True, init=False)
class PromptAssetRecord:
    asset_id: str
    root_kind: str
    source_path: Path


@dataclass(frozen=True, init=False)
class PromptAssemblyPart:
    asset_id: str


@dataclass(frozen=True, init=False)
class PromptAssemblyRecord:
    assembly_id: str
    root_kind: str
    source_path: Path
    parts: tuple[PromptAssemblyPart, ...]


@dataclass(frozen=True)
class PromptArtifactDetails:
    root_kind: str
    source_path: Path

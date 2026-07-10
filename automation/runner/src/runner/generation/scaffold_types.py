from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path

SUPPORTED_SCAFFOLD_KINDS = {"milestone", "single_prompt", "handoff"}


@dataclass(frozen=True)
class ScaffoldRequest:
    kind: str
    name: str
    project_root: Path
    spec_file: str
    force: bool = False
    telegram: bool = False

    def __post_init__(self) -> None:
        if self.kind not in SUPPORTED_SCAFFOLD_KINDS:
            raise ValueError(f"unsupported scaffold kind {self.kind!r}")
        if not self.name.replace("-", "").replace("_", "").isalnum():
            raise ValueError("scaffold name must be alphanumeric, dash, or underscore")


@dataclass(frozen=True)
class ScaffoldResult:
    config_path: Path
    prompt_root: Path

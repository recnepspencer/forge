from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path


RUNTIME_ROOT = Path("automation/phase_runner/runtime")


@dataclass(frozen=True)
class RuntimePaths:
    run_id: str

    @property
    def events(self) -> Path:
        return RUNTIME_ROOT / "events" / f"{self.run_id}.jsonl"

    @property
    def projection(self) -> Path:
        return RUNTIME_ROOT / "projections" / f"{self.run_id}.json"

    @property
    def log(self) -> Path:
        return RUNTIME_ROOT / "logs" / f"{self.run_id}.jsonl"


def ensure_runtime_dirs() -> None:
    for relative in ("events", "projections", "logs"):
        (RUNTIME_ROOT / relative).mkdir(parents=True, exist_ok=True)

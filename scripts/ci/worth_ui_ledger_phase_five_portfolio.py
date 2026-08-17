from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
from typing import Callable

from worth_ui_ledger_operational_successors import record_proved_execution, relative


RerunRow = Callable[..., dict[str, object]]


@dataclass
class PhaseFivePortfolioExecution:
    root: Path
    ledger: Path
    temporary: Path
    candidate: Path
    rows: list[dict[str, str]]
    replacements: dict[str, dict[str, str]]
    rerun_row: RerunRow
    compile_artifact: str
    observations: list[dict[str, object]]

    def execute(self) -> None:
        by_requirement = {row["requirement"]: row for row in self.rows}
        order = ["P5-PREDECESSOR-01", "P5-ATLAS-01", "P5-ATLAS-PINNING-01"]
        close = by_requirement.pop("P5-CLOSE-01", None)
        ordered = [by_requirement.pop(name) for name in order if name in by_requirement]
        ordered.extend(by_requirement.values())
        if close is not None:
            ordered.append(close)
        atlas_artifact: str | None = None
        for index, row in enumerate(ordered):
            artifact = self.temporary / f"p5-{index:02}.json"
            values: dict[str, object] = {"candidate_ledger": self.candidate}
            if row["requirement"] == "P5-ATLAS-PINNING-01":
                if atlas_artifact is None:
                    raise RuntimeError("pinning proof requires the proved atlas artifact")
                values["supporting_world_artifact"] = atlas_artifact
            observation = self.rerun_row(
                row, artifact, self.compile_artifact, **values
            )
            self.observations.append(observation)
            record_proved_execution(
                row,
                relative(self.root, artifact),
                observation,
                self.replacements,
                self.ledger,
                self.candidate,
            )
            if row["requirement"] == "P5-ATLAS-01":
                atlas_artifact = relative(self.root, artifact)

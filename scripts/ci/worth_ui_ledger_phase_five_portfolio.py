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
        order = ["P5-PREDECESSOR-01"]
        close = by_requirement.pop("P5-CLOSE-01", None)
        ordered = [by_requirement.pop(name) for name in order if name in by_requirement]
        ordered.extend(by_requirement.values())
        if close is not None:
            ordered.append(close)
        for index, row in enumerate(ordered):
            artifact = self.temporary / f"p5-{index:02}.json"
            observation = self.rerun_row(
                row, artifact, self.compile_artifact, candidate_ledger=self.candidate
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

from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
from typing import Callable

from worth_ui_ledger_operational_successors import (
    find,
    predecessor_observations,
    record_proved_execution,
    relative,
    shared_artifact,
)


RerunRow = Callable[..., dict[str, object]]


@dataclass
class PhaseThreePortfolioExecution:
    root: Path
    ledger: Path
    temporary: Path
    candidate: Path
    rows: list[dict[str, str]]
    replacements: dict[str, dict[str, str]]
    rerun_row: RerunRow
    compile_artifact: str
    observations: list[dict[str, object]]
    predecessor_requirements: frozenset[str]

    def execute(self) -> None:
        predecessor = find(self.rows, "P3-PREDECESSOR-01")
        if predecessor is not None:
            observation = self.rerun_named(predecessor, "p3-predecessor.json")
            self.validate_loaded_predecessor(observation)
        delta_world = find(self.rows, "P3-DELTA-SOURCE-01")
        native_world = find(self.rows, "P3-HP02-WORLD-01")
        fresh_delta = self.world(delta_world, "p3-delta-world.json", None)
        fresh_native = self.world(native_world, "p3-native-world.json", fresh_delta)
        close = find(self.rows, "P3-CLOSE-01")
        for index, row in enumerate(self.rows):
            if row in (predecessor, delta_world, native_world, close):
                continue
            shared = shared_artifact(row["requirement"], fresh_delta, fresh_native)
            artifact = self.temporary / f"p3-{index:02}.json"
            observation = self.rerun_row(
                row,
                artifact,
                self.compile_artifact,
                shared_world_artifact=shared,
                candidate_ledger=self.candidate,
            )
            self.record(row, artifact, observation)
        if close is not None:
            self.rerun_named(close, "p3-close.json")

    def rerun_named(self, row: dict[str, str], name: str) -> dict[str, object]:
        artifact = self.temporary / name
        observation = self.rerun_row(
            row,
            artifact,
            self.compile_artifact,
            candidate_ledger=self.candidate,
        )
        self.record(row, artifact, observation)
        return observation

    def world(
        self, row: dict[str, str] | None, name: str, supporting: str | None
    ) -> str | None:
        if row is None:
            return None
        artifact = self.temporary / name
        observation = self.rerun_row(
            row,
            artifact,
            self.compile_artifact,
            candidate_ledger=self.candidate,
            supporting_world_artifact=supporting,
        )
        self.record(row, artifact, observation)
        return relative(self.root, artifact)

    def record(
        self, row: dict[str, str], artifact: Path, observation: dict[str, object]
    ) -> None:
        self.observations.append(observation)
        record_proved_execution(
            row, relative(self.root, artifact), observation, self.replacements,
            self.ledger, self.candidate,
        )

    def validate_loaded_predecessor(self, observation: dict[str, object]) -> None:
        imported = predecessor_observations(
            observation, self.temporary, self.root, self.predecessor_requirements
        )
        loaded = {
            row.get("requirement") for row in self.observations
            if isinstance(row, dict)
            and str(row.get("requirement", "")).startswith(("P1-", "P2-"))
        }
        imported_requirements = {row.get("requirement") for row in imported}
        if loaded != imported_requirements:
            raise RuntimeError("loaded predecessor executions differ from their handoff")

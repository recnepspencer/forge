from __future__ import annotations

from dataclasses import dataclass, field
from pathlib import Path
from typing import Callable

from worth_ui_compile_artifact_cache import materialize as materialize_compile_artifact
from worth_ui_ledger_operational_successors import record_proved_execution
from worth_ui_ledger_portfolio_snapshot import source_state_for_row


RerunRow = Callable[..., dict[str, object]]
SourceRevision = Callable[[Path], str]


@dataclass
class PhaseTwoPortfolioExecution:
    root: Path
    ledger: Path
    temporary: Path
    candidate: Path
    governed: list[dict[str, str]]
    replacements: dict[str, dict[str, str]]
    rerun_row: RerunRow
    source_revision: SourceRevision
    observations: list[dict[str, object]] = field(default_factory=list)
    fresh_compile: str = ""

    def execute(self) -> list[dict[str, object]]:
        self.fresh_compile = self.materialize_compile_contracts()
        phase_one, mounted, headless, close, world, phase_two = self.rows_by_role()
        self.rerun_independent_phase_one(phase_one)
        mounted_identity = self.rerun_mounted_world(mounted)
        self.rerun_headless_cost(headless, mounted_identity)
        self.rerun_phase_one_close(close)
        world_identity = self.rerun_native_world(world)
        self.rerun_dependent_phase_two(phase_two, world_identity)
        return self.observations

    def materialize_compile_contracts(self) -> str:
        identity = self.temporary / "compile-contracts.json"
        revision = self.source_revision(self.root)
        materialize_compile_artifact(
            self.root, identity, revision, source_state_for_row(revision)
        )
        return identity.relative_to(self.root).as_posix()

    def rows_by_role(self) -> tuple[
        list[dict[str, str]],
        dict[str, str],
        dict[str, str],
        dict[str, str],
        dict[str, str],
        list[dict[str, str]],
    ]:
        by_requirement = {row["requirement"]: row for row in self.governed}
        excluded = {"P1-WORLDS-01", "P1-HEADLESS-COST-01", "P1-CLOSE-01"}
        phase_one = [
            row for row in self.governed
            if row["phase"] == "1" and row["requirement"] not in excluded
        ]
        phase_two = [
            row for row in self.governed
            if row["phase"] == "2" and row["requirement"] != "P2-WORLD-01"
        ]
        return (
            phase_one,
            by_requirement["P1-WORLDS-01"],
            by_requirement["P1-HEADLESS-COST-01"],
            by_requirement["P1-CLOSE-01"],
            by_requirement["P2-WORLD-01"],
            phase_two,
        )

    def rerun_independent_phase_one(self, rows: list[dict[str, str]]) -> None:
        for index, row in enumerate(rows):
            artifact = self.temporary / f"p1-{index:02}.json"
            observation = self.rerun_row(
                row, artifact, self.fresh_compile, candidate_ledger=self.candidate
            )
            self.record(row, artifact.relative_to(self.root).as_posix(), observation)

    def rerun_mounted_world(self, row: dict[str, str]) -> str:
        identity = (self.temporary / "mounted-world.json").relative_to(self.root).as_posix()
        observation = self.rerun_row(
            row, self.root / identity, self.fresh_compile,
            candidate_ledger=self.candidate,
        )
        self.record(row, identity, observation)
        return identity

    def rerun_headless_cost(self, row: dict[str, str], mounted: str) -> None:
        artifact = self.temporary / "p1-headless-cost.json"
        observation = self.rerun_row(
            row,
            artifact,
            self.fresh_compile,
            shared_world_artifact=mounted,
            candidate_ledger=self.candidate,
        )
        self.record(row, artifact.relative_to(self.root).as_posix(), observation)

    def rerun_phase_one_close(self, row: dict[str, str]) -> None:
        artifact = self.temporary / "p1-close.json"
        observation = self.rerun_row(
            row, artifact, self.fresh_compile, candidate_ledger=self.candidate
        )
        self.record(row, artifact.relative_to(self.root).as_posix(), observation)

    def rerun_native_world(self, row: dict[str, str]) -> str:
        identity = (self.temporary / "native-world.json").relative_to(self.root).as_posix()
        observation = self.rerun_row(
            row, self.root / identity, self.fresh_compile,
            candidate_ledger=self.candidate,
        )
        self.record(row, identity, observation)
        return identity

    def rerun_dependent_phase_two(self, rows: list[dict[str, str]], world: str) -> None:
        for index, row in enumerate(rows):
            artifact = self.temporary / f"p2-{index:02}.json"
            observation = self.rerun_row(
                row,
                artifact,
                self.fresh_compile,
                shared_world_artifact=world,
                candidate_ledger=self.candidate,
            )
            self.record(row, artifact.relative_to(self.root).as_posix(), observation)

    def record(
        self, row: dict[str, str], artifact: str, observation: dict[str, object]
    ) -> None:
        self.observations.append(observation)
        record_proved_execution(
            row,
            artifact,
            observation,
            self.replacements,
            self.ledger,
            self.candidate,
        )

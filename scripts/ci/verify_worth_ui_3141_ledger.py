from __future__ import annotations

import csv
import json
import os
import subprocess
import sys
import tempfile
from pathlib import Path

from worth_ui_ledger_source_state import source_state_digest
from worth_ui_ledger_portfolio_snapshot import operational_source_snapshot
from worth_ui_ledger_operational_successors import (
    prepared_open_rows,
    write_candidate_ledger,
)
from worth_ui_predecessor_handoff import predecessor_artifact, write_artifact
from worth_ui_ledger_verifier_invocation import parse_args, source_revision
from worth_ui_ledger_execution_cache import CACHE_ENV
from worth_ui_ledger_runner_authentication import RunnerProvenanceUnavailable
from worth_ui_ledger_retained_portfolio import (
    persist_referenced_receipts,
    portfolio_identity,
    validate as validate_retained_portfolio,
)
from worth_ui_ledger_phase_two_portfolio import PhaseTwoPortfolioExecution
from worth_ui_ledger_phase_three_portfolio import PhaseThreePortfolioExecution
from worth_ui_ledger_phase_four_portfolio import PhaseFourPortfolioExecution
from worth_ui_ledger_phase_five_portfolio import PhaseFivePortfolioExecution
from worth_ui_ledger_portfolio_row import PortfolioRowExecutor
from worth_ui_ledger_verifier_rebinding import (
    COMPILE_ARTIFACT,
    MOUNTED_WORLD_ARTIFACT,
    NATIVE_WORLD_ARTIFACT,
    P3_DELTA_ARTIFACT,
    P3_PREDECESSOR_HANDOFF,
    P3_WORLD_ARTIFACT,
    P4_PREDECESSOR_HANDOFF,
    bind_fresh_compile_artifact,
    bind_fresh_shared_world,
    bind_fresh_supporting_world,
)


ROOT = Path(__file__).resolve().parents[2]
LEDGER = ROOT / "_docs/worth-ui/milestone-3.14.1-proof-ledger.csv"
TARGET = ROOT / "workspaces/worth-ui/target"


def retained_source_binding(through_phase: int) -> tuple[str, str]:
    identity = ROOT / portfolio_identity(through_phase)
    retained = json.loads(identity.read_text(encoding="utf-8"))
    revision = retained.get("source_revision")
    state_digest = retained.get("source_state_digest")
    if not isinstance(revision, str) or not isinstance(state_digest, str):
        raise RuntimeError("retained closure portfolio omits its source binding")
    return revision, state_digest


def ledger_identity() -> Path:
    configured = os.environ.get("WORTH_UI_MILESTONE_3141_LEDGER")
    return Path(configured).resolve() if configured else LEDGER


def rows(through_phase: int) -> list[dict[str, str]]:
    with ledger_identity().open(encoding="utf-8", newline="") as stream:
        complete = list(csv.DictReader(stream))
    result = [row for row in complete if int(row["phase"]) <= through_phase]
    expected = {2: 30, 3: 47, 4: 68, 5: 80}[through_phase]
    if len(result) != expected or any(
        row["result"] != "PROVED" or row["final_source"] != "true" for row in result
    ):
        raise RuntimeError(
            f"operational verification requires {expected} final-source proved rows"
        )
    return result


def successor_reopenings(through_phase: int) -> dict[str, dict[str, str]]:
    with ledger_identity().open(encoding="utf-8", newline="") as stream:
        successors = [
            dict(row) for row in csv.DictReader(stream)
            if int(row["phase"]) > through_phase
        ]
    for row in successors:
        row["result"] = "OPEN"
        row["final_source"] = "false"
    return {row["requirement"]: row for row in successors}


def operational_reopenings(
    through_phase: int, governed: list[dict[str, str]]
) -> dict[str, dict[str, str]]:
    replacements = successor_reopenings(through_phase)
    replacements.update(prepared_open_rows(governed))
    return replacements


def run(command: list[str], candidate_ledger: Path | None = None) -> None:
    environment = dict(os.environ)
    if candidate_ledger is not None:
        environment["WORTH_UI_MILESTONE_3141_LEDGER"] = str(candidate_ledger.resolve())
    completed = subprocess.run(
        command, cwd=ROOT, env=environment, capture_output=True, text=True, check=False
    )
    if completed.returncode != 0:
        sys.stderr.write(completed.stdout)
        sys.stderr.write(completed.stderr)
        raise RuntimeError(f"operational verification failed: {' '.join(command)}")


rerun_row = PortfolioRowExecutor(ROOT, TARGET)


def closure_tests(through_phase: int, candidate_ledger: Path) -> int:
    prefix = [
        "cargo", "test", "--manifest-path", "workspaces/worth-ui/Cargo.toml",
        "-p", "worth-ui-certification", "--test", "topology_contracts",
    ]
    run(prefix + [
        "milestone_3141_phase1_ledger::mutation_tests::milestone_ledger_has_exact_schema_inventory_and_honest_posture",
        "--", "--exact", "--nocapture",
    ], candidate_ledger)
    closure = {
        2: "phase_two_closure_requires_every_phase_one_and_two_row",
        3: "phase_three_closure_requires_every_predecessor_and_phase_three_row",
        4: "phase_four_closure_requires_every_predecessor_and_phase_four_row",
        5: "phase_five_closure_requires_every_predecessor_and_phase_five_row",
    }[through_phase]
    run(prefix + [
        f"milestone_3141_phase1_ledger::{closure}",
        "--", "--exact", "--ignored", "--nocapture",
    ], candidate_ledger)
    return 2


def main() -> int:
    arguments = parse_args()
    revision = source_revision(ROOT)
    state_digest = source_state_digest(revision)
    if arguments.artifact is None:
        recorded_revision, recorded_digest = retained_source_binding(
            arguments.through_phase
        )
        if (recorded_revision, recorded_digest) != (revision, state_digest):
            print(
                "[portfolio:revalidate] retained source binding is historical; "
                "executing current-source portfolio",
                file=sys.stderr,
                flush=True,
            )
            execute_current_portfolio(arguments, revision, state_digest)
            print(
                f"Worth UI milestone 3.14.1 operationally revalidated through "
                f"Phase {arguments.through_phase}",
                flush=True,
            )
            return 0
        persist_referenced_receipts(
            ROOT, ledger_identity(), arguments.through_phase, recorded_digest
        )
        try:
            validate_retained_portfolio(
                ROOT,
                ledger_identity(),
                arguments.through_phase,
                recorded_revision,
                recorded_digest,
            )
        except RunnerProvenanceUnavailable as error:
            print(
                f"[portfolio:revalidate] {error}; executing current-source portfolio",
                file=sys.stderr,
                flush=True,
            )
            execute_current_portfolio(arguments, revision, state_digest)
            print(
                f"Worth UI milestone 3.14.1 operationally revalidated through "
                f"Phase {arguments.through_phase}",
                flush=True,
            )
            return 0
        closure_tests(arguments.through_phase, ledger_identity())
        print(
            f"Worth UI milestone 3.14.1 retained portfolio validated through "
            f"Phase {arguments.through_phase}",
            flush=True,
        )
        return 0
    observations, closure_count = execute_current_portfolio(
        arguments, revision, state_digest
    )
    if arguments.artifact is not None:
        write_artifact(
            arguments.artifact,
            predecessor_artifact(
                arguments.through_phase,
                revision,
                state_digest,
                observations,
                closure_count,
            ),
        )
    print(
        f"Worth UI milestone 3.14.1 ledger operationally verified through Phase "
        f"{arguments.through_phase}: {len(observations)} fresh rows"
    )
    return 0


def execute_current_portfolio(
    arguments: object, revision: str, state_digest: str
) -> tuple[list[dict[str, object]], int]:
    previous_cache = os.environ.get(CACHE_ENV)
    os.environ[CACHE_ENV] = str(
        TARGET / "milestone-3141-execution-cache" / state_digest
    )
    try:
        with operational_source_snapshot(revision, state_digest):
            observations, closure_count = execute_portfolio(arguments)
    finally:
        if previous_cache is None:
            os.environ.pop(CACHE_ENV, None)
        else:
            os.environ[CACHE_ENV] = previous_cache
    if source_revision(ROOT) != revision or source_state_digest(revision) != state_digest:
        raise RuntimeError("governed source changed during operational verification")
    return observations, closure_count


class OperationalPortfolio:
    def __init__(self, arguments: argparse.Namespace) -> None:
        self.arguments = arguments
        self.observations: list[dict[str, object]] = []

    def execute(self) -> tuple[list[dict[str, object]], int]:
        TARGET.mkdir(parents=True, exist_ok=True)
        with tempfile.TemporaryDirectory(
            prefix="worth-ui-3141-verify-", dir=TARGET
        ) as directory:
            return self.execute_at(Path(directory))

    def execute_at(self, temporary: Path) -> tuple[list[dict[str, object]], int]:
        governed = rows(self.arguments.through_phase)
        candidate = temporary / "dependency-ledger.csv"
        replacements = operational_reopenings(
            self.arguments.through_phase, governed
        )
        write_candidate_ledger(ledger_identity(), candidate, replacements)
        phase_two = PhaseTwoPortfolioExecution(
            ROOT, ledger_identity(), temporary, candidate,
            [row for row in governed if int(row["phase"]) <= 2],
            replacements, rerun_row, source_revision,
        )
        self.observations.extend(phase_two.execute())
        if self.arguments.through_phase == 2:
            return self.observations, closure_tests(2, candidate)
        self.write_handoff(2, temporary, candidate, "p3-predecessor-handoff.json")
        self.execute_phase_three(
            governed, temporary, candidate, replacements, phase_two.fresh_compile
        )
        if self.arguments.through_phase == 3:
            return self.observations, closure_tests(3, candidate)
        self.write_handoff(3, temporary, candidate, "p4-predecessor-handoff.json")
        PhaseFourPortfolioExecution(
            ROOT, ledger_identity(), temporary, candidate,
            [row for row in governed if int(row["phase"]) == 4],
            replacements, rerun_row, phase_two.fresh_compile, self.observations,
        ).execute()
        if self.arguments.through_phase == 4:
            return self.observations, closure_tests(4, candidate)
        self.write_handoff(4, temporary, candidate, "p5-predecessor-handoff.json")
        PhaseFivePortfolioExecution(
            ROOT, ledger_identity(), temporary, candidate,
            [row for row in governed if int(row["phase"]) == 5],
            replacements, rerun_row, phase_two.fresh_compile, self.observations,
        ).execute()
        return self.observations, closure_tests(5, candidate)

    def execute_phase_three(
        self, governed: list[dict[str, str]], temporary: Path, candidate: Path,
        replacements: dict[str, dict[str, str]], compile_artifact: str,
    ) -> None:
        predecessor_requirements = frozenset(
            row["requirement"] for row in governed if int(row["phase"]) <= 2
        )
        PhaseThreePortfolioExecution(
            ROOT, ledger_identity(), temporary, candidate,
            [row for row in governed if int(row["phase"]) == 3],
            replacements, rerun_row, compile_artifact, self.observations,
            predecessor_requirements,
        ).execute()

    def write_handoff(
        self, through_phase: int, temporary: Path, candidate: Path, name: str
    ) -> None:
        closure_count = closure_tests(through_phase, candidate)
        write_artifact(
            (temporary / name).relative_to(ROOT).as_posix(),
            predecessor_artifact(
                through_phase, revision_for_portfolio(), state_for_portfolio(),
                self.observations, closure_count,
            ),
        )


def execute_portfolio(
    arguments: argparse.Namespace,
) -> tuple[list[dict[str, object]], int]:
    return OperationalPortfolio(arguments).execute()


def revision_for_portfolio() -> str:
    return source_revision(ROOT)


def state_for_portfolio() -> str:
    revision = revision_for_portfolio()
    return source_state_digest(revision)


if __name__ == "__main__":
    raise SystemExit(main())

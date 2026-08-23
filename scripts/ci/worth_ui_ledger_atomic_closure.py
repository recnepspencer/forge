from __future__ import annotations

import os
import subprocess
import sys
import tempfile
from dataclasses import dataclass
from enum import Enum
from pathlib import Path
import csv
import io

from worth_ui_3141_proof_plan import COMPILE_ARTIFACT
from worth_ui_ledger_acceptance import (
    bind_current_result_mapping,
    close_row,
    retain_portfolio_artifact,
)
from worth_ui_ledger_artifact_transaction import ArtifactTransaction, replace_bytes
from worth_ui_ledger_closure_storage import (
    render_requirement_update,
    synchronize_historical_rows,
    transaction_extra_identities,
)
from worth_ui_ledger_command import claim_digest_for_row, source_revision
from worth_ui_ledger_execution_observation_store import CACHE_ENV
from worth_ui_ledger_portfolio_snapshot import DIGEST_ENV, REVISION_ENV
from worth_ui_ledger_retained_portfolio import publish
from worth_ui_ledger_row_cache import RowEvidenceCache
from worth_ui_ledger_row_execution import execute_or_restore, run_row
from worth_ui_ledger_source_state import source_state_digest
from worth_ui_predecessor_candidate import import_candidate_prefix


ROOT = Path(__file__).resolve().parents[2]
LEDGER = ROOT / "_docs/worth-ui/milestone-3.14.1-proof-ledger.csv"


class ClosurePreparation(Enum):
    NONE = "none"
    CURRENT_COMPILE_CONTRACTS = "current-compile-contracts"


@dataclass(frozen=True)
class AtomicClosurePlan:
    selected: tuple[dict[str, str], ...]
    verify_phase: int | None
    preparation: ClosurePreparation = ClosurePreparation.NONE


@dataclass
class _ClosureExecution:
    rows: list[dict[str, str]]
    fields: list[str]
    plan: AtomicClosurePlan
    candidate: Path
    original: str
    cache: RowEvidenceCache
    root: Path
    revision: str
    state_digest: str
    prepared_requirements: set[str]

    def execute(self) -> None:
        requirements = {
            row["requirement"] for row in self.plan.selected
        } | self.prepared_requirements
        retained: dict[str, bytes] = {}
        for row in self.plan.selected:
            if row["requirement"].endswith("-CLOSE-01"):
                _restore_portfolio_evidence(self.root, retained)
            claim = claim_digest_for_row(row)
            result = execute_or_restore(
                row,
                self.candidate,
                self.cache,
                claim,
                lambda command, ledger=None: run_row(self.root, command, ledger),
                lambda payload, selected=row, digest=claim: bind_current_result_mapping(
                    selected,
                    payload,
                    self.root,
                    digest,
                    self.revision,
                    self.state_digest,
                ),
                restore=not row["requirement"].endswith("-PREDECESSOR-01"),
            )
            if row["requirement"].endswith("-PREDECESSOR-01"):
                requirements.update(
                    import_candidate_prefix(
                        self.rows, self.candidate, int(row["phase"])
                    )
                )
            close_row(row, result)
            retain_portfolio_artifact(row, result, retained, self.root)
            self.candidate.write_text(
                render_requirement_update(
                    self.original, self.rows, self.fields, requirements
                ),
                encoding="utf-8",
                newline="",
            )


def close_atomically(
    rows: list[dict[str, str]],
    fields: list[str],
    plan: AtomicClosurePlan,
    root: Path = ROOT,
    ledger: Path = LEDGER,
) -> None:
    original = ledger.read_text(encoding="utf-8")
    revision = source_revision()
    state_digest = source_state_digest(revision)
    cache_root = root / "workspaces/worth-ui/target/milestone-3141-execution-cache" / state_digest
    cache = RowEvidenceCache(root, cache_root, original.encode(), revision, state_digest)
    previous = _install_execution_environment(cache_root, revision, state_digest)
    extra = set(transaction_extra_identities(rows, list(plan.selected), plan.verify_phase))
    if plan.preparation is ClosurePreparation.CURRENT_COMPILE_CONTRACTS:
        extra.add(COMPILE_ARTIFACT)
    artifacts = ArtifactTransaction(
        root,
        ledger,
        [row["exact_command"] for row in plan.selected],
        tuple(sorted(extra)),
    )
    candidate = _candidate_ledger(root, original)
    prepared_requirements = changed_requirements(original, rows)
    candidate.write_text(
        render_requirement_update(original, rows, fields, prepared_requirements),
        encoding="utf-8",
        newline="",
    )
    try:
        _prepare(plan.preparation, root)
        _ClosureExecution(
            rows,
            fields,
            plan,
            candidate,
            original,
            cache,
            root,
            revision,
            state_digest,
            prepared_requirements,
        ).execute()
        synchronize_historical_rows(candidate, root)
        validate_ledger_posture(root, candidate)
        if source_revision() != revision or source_state_digest(revision) != state_digest:
            raise RuntimeError("governed source changed during phase closure")
        if plan.verify_phase is not None:
            publish(root, candidate, plan.verify_phase, revision, state_digest)
            verify_closed_prefix(plan.verify_phase, root, candidate)
        candidate_bytes = candidate.read_bytes()
        artifacts.prepare_commit(candidate_bytes)
        replace_bytes(ledger, candidate_bytes)
        artifacts.commit()
    except BaseException:
        artifacts.rollback()
        raise
    finally:
        _restore_execution_environment(previous)
        candidate.unlink(missing_ok=True)


def changed_requirements(
    original: str, rows: list[dict[str, str]]
) -> set[str]:
    retained = {
        row["requirement"]: row
        for row in csv.DictReader(io.StringIO(original, newline=""))
    }
    return {
        row["requirement"]
        for row in rows
        if retained.get(row["requirement"]) != row
    }


def _prepare(preparation: ClosurePreparation, root: Path) -> None:
    if preparation is ClosurePreparation.NONE:
        return
    subprocess.run(
        [
            sys.executable,
            "scripts/ci/run_worth_ui_compile_contracts.py",
            "--artifact",
            COMPILE_ARTIFACT,
        ],
        cwd=root,
        check=True,
    )


def _candidate_ledger(root: Path, original: str) -> Path:
    directory = root / "workspaces/worth-ui/target/milestone-3141-candidates"
    directory.mkdir(parents=True, exist_ok=True)
    with tempfile.NamedTemporaryFile(
        "w", encoding="utf-8", newline="", dir=directory, delete=False
    ) as stream:
        stream.write(original)
        return Path(stream.name)


def _restore_portfolio_evidence(root: Path, retained: dict[str, bytes]) -> None:
    for identity, content in retained.items():
        replace_bytes(root / identity, content)


def validate_ledger_posture(root: Path = ROOT, candidate: Path | None = None) -> None:
    environment = None
    if candidate is not None:
        environment = dict(os.environ)
        environment["WORTH_UI_MILESTONE_3141_LEDGER"] = str(candidate.resolve())
    subprocess.run(
        [
            "cargo", "test", "--manifest-path", "workspaces/worth-ui/Cargo.toml",
            "-p", "worth-ui-certification", "--test", "topology_contracts",
            "milestone_3141_phase1_ledger::mutation_tests::milestone_ledger_has_exact_schema_inventory_and_honest_posture",
            "--", "--exact", "--nocapture",
        ],
        cwd=root,
        env=environment,
        check=True,
    )


def verify_closed_prefix(
    phase: int, root: Path = ROOT, candidate: Path | None = None
) -> None:
    environment = dict(os.environ)
    if candidate is not None:
        environment["WORTH_UI_MILESTONE_3141_LEDGER"] = str(candidate.resolve())
    subprocess.run(
        [
            sys.executable,
            "scripts/ci/verify_worth_ui_3141_ledger.py",
            "--through-phase",
            str(phase),
        ],
        cwd=root,
        env=environment,
        check=True,
    )


def _install_execution_environment(
    cache_root: Path, revision: str, state_digest: str
) -> tuple[str | None, str | None, str | None]:
    previous = os.environ.get(CACHE_ENV), os.environ.get(REVISION_ENV), os.environ.get(DIGEST_ENV)
    os.environ[CACHE_ENV] = str(cache_root)
    os.environ[REVISION_ENV] = revision
    os.environ[DIGEST_ENV] = state_digest
    return previous


def _restore_execution_environment(
    previous: tuple[str | None, str | None, str | None]
) -> None:
    for name, value in zip((CACHE_ENV, REVISION_ENV, DIGEST_ENV), previous):
        if value is None:
            os.environ.pop(name, None)
        else:
            os.environ[name] = value

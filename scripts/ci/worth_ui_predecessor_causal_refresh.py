from __future__ import annotations

import csv
import hashlib
import json
import os
import shutil
import sys
import tempfile
import time
from dataclasses import dataclass
from pathlib import Path

from worth_ui_3141_proof_plan import prepare_claim, proofs
from worth_ui_ledger_causal_revalidation import (
    revalidate_joined_predecessor_payload,
    revalidate_row_payload,
)
from worth_ui_ledger_command import claim_digest_for_row, source_revision
from worth_ui_ledger_artifact_identity import ArtifactIdentity, predecessor_handoff
from worth_ui_ledger_execution_observation_store import CACHE_ENV
from worth_ui_ledger_execution_observation_retention import retain_payload_observations
from worth_ui_ledger_portfolio_row import PortfolioRowExecutor
from worth_ui_ledger_portfolio_snapshot import DIGEST_ENV, REVISION_ENV
from worth_ui_ledger_row_cache import RowEvidenceCache
from worth_ui_ledger_source_state import source_state_digest
from worth_ui_ledger_candidate_basis import from_path
from worth_ui_predecessor_handoff_currentness import (
    PredecessorVerification,
    expected_identity,
    is_current,
)
from worth_ui_predecessor_candidate import (
    import_refreshed_observations,
    publish_refreshed_prefix,
    read_refreshed_prefix,
    retain_current_artifact,
    write_candidate_ledger,
)
from worth_ui_predecessor_handoff import predecessor_artifact, write_artifact
from worth_ui_predecessor_refresh_order import ordered_rows


@dataclass(frozen=True)
class RefreshMode:
    root_phase: int | None = None

    @classmethod
    def root(cls, phase: int) -> "RefreshMode":
        if phase < 1:
            raise ValueError("root refresh phase must be positive")
        return cls(phase)

    def is_root(self) -> bool:
        return self.root_phase is not None

    def name(self) -> str:
        return (
            f"root-phase-{self.root_phase}"
            if self.root_phase is not None
            else "historical-prefix"
        )

    def preserves_historical_claim(self, row: dict[str, str]) -> bool:
        del row
        return self.is_root()


@dataclass(frozen=True)
class RefreshContext:
    root: Path
    candidate: Path
    revision: str
    state_digest: str
    executor: PortfolioRowExecutor
    row_cache: RowEvidenceCache
    retained: dict[str, dict[str, object]]
    refresh_mode: RefreshMode = RefreshMode()


from worth_ui_predecessor_refresh_runtime import (
    closure_tests,
    ensure_compile_artifact,
    environment_snapshot,
    execute_row,
    governed_rows,
    restore_environment,
)
def refresh_handoff(
    root: Path,
    ledger: Path,
    phase: int,
    identity: ArtifactIdentity | None = None,
    *,
    temporary: bool = False,
) -> list[dict[str, object]]:
    if identity is not None and identity.phase != phase:
        raise ValueError("predecessor publication phase does not match the refresh")
    revision = source_revision()
    state_digest = source_state_digest(revision)
    previous = environment_snapshot()
    os.environ[CACHE_ENV] = str(
        root / "workspaces/worth-ui/target/milestone-3141-execution-cache" / state_digest
    )
    os.environ[REVISION_ENV] = revision
    os.environ[DIGEST_ENV] = state_digest
    try:
        ensure_compile_artifact(root, revision, state_digest)
        observations, reused, executed, closure_count = current_observations(
            root,
            ledger,
            phase - 1,
            revision,
            state_digest,
            retained_observations(
                root,
                (identity or predecessor_handoff(phase)).relative_path,
                phase - 1,
            ),
            refresh_mode=RefreshMode.root(phase),
        )
        persist_observation_receipts(root, state_digest, observations)
        publish_refreshed_prefix(
            root,
            ledger,
            observations,
            lambda row: prepared_row(row, RefreshMode.root(phase)),
        )
        basis = from_path(ledger, phase - 1)
        verification = PredecessorVerification(
            root, ledger, phase, revision, state_digest
        )
        publication = (
            expected_identity(verification)[0]
            if temporary
            else identity or predecessor_handoff(phase)
        )
        artifact = predecessor_artifact(
            phase - 1, revision, state_digest, observations, closure_count, basis
        )
        artifact["causal_reused_requirement_count"] = reused
        artifact["executed_requirement_count"] = executed
        write_artifact(root, publication, artifact)
        print(
            f"[predecessor:complete] through_phase={phase - 1} "
            f"reused={reused} executed={executed}",
            file=sys.stderr,
            flush=True,
        )
        return observations
    finally:
        restore_environment(previous)


def persist_observation_receipts(
    root: Path, state_digest: str, observations: list[dict[str, object]]
) -> None:
    for observation in observations:
        retain_payload_observations(root, state_digest, observation)


def current_observations(
    root: Path,
    ledger: Path,
    through_phase: int,
    revision: str,
    state_digest: str,
    retained: dict[str, dict[str, object]] | None = None,
    refresh_mode: RefreshMode | None = None,
) -> tuple[list[dict[str, object]], int, int, int]:
    rows = governed_rows(ledger, through_phase)
    ordered = ordered_rows(rows)
    target = root / "workspaces/worth-ui/target"
    target.mkdir(parents=True, exist_ok=True)
    executor = PortfolioRowExecutor(root, target)
    row_cache = RowEvidenceCache(
        root,
        root / "workspaces/worth-ui/target/milestone-3141-execution-cache" / state_digest,
        ledger.read_bytes(),
        revision,
        state_digest,
    )
    observations: dict[str, dict[str, object]] = {}
    settled: dict[str, tuple[dict[str, str], dict[str, object]]] = {}
    rows_by_requirement = {row["requirement"]: row for row in rows}
    reused = executed = 0
    retained = retained or {}
    mode = refresh_mode or RefreshMode()
    with tempfile.NamedTemporaryFile(delete=False, dir=target) as stream:
        candidate = Path(stream.name)
    shutil.copyfile(ledger, candidate)
    try:
        context = RefreshContext(
            root,
            candidate,
            revision,
            state_digest,
            executor,
            row_cache,
            retained,
            mode,
        )
        for original in ordered:
            started = time.perf_counter_ns()
            row, observation, disposition, refreshed = settle_row(context, original)
            import_refreshed_observations(
                rows_by_requirement,
                refreshed,
                observations,
                settled,
                lambda row: prepared_row(row, context.refresh_mode),
            )
            executed += disposition == "execute"
            reused += disposition == "reuse"
            observations[row["requirement"]] = observation
            settled[row["requirement"]] = (row, observation)
            persist_observation_receipts(root, state_digest, [observation])
            write_candidate_ledger(ledger, candidate, settled)
            elapsed = max(1, (time.perf_counter_ns() - started + 999_999) // 1_000_000)
            print(f"[predecessor:row] {row['requirement']} disposition={disposition} "
                  f"duration_ms={elapsed}", file=sys.stderr, flush=True)
        closure_count = closure_tests(root, candidate, through_phase)
        return [observations[row["requirement"]] for row in rows], reused, executed, closure_count
    finally:
        candidate.unlink(missing_ok=True)


def settle_row(
    context: RefreshContext, original: dict[str, str]
) -> tuple[dict[str, str], dict[str, object], str, list[dict[str, object]]]:
    row = prepared_row(original, context.refresh_mode)
    if row["requirement"].endswith("-PREDECESSOR-01"):
        context.row_cache.bind_ledger(context.candidate.read_bytes())
    proof = proofs().get(row["requirement"])
    claim = claim_digest_for_row(row)
    artifact, payload, artifact_payload, artifact_sha256, retained_payload = (
        retained_payload_for_row(context, row, claim)
    )
    observation = revalidate_row_payload(
        context.root, row, payload, artifact_sha256, claim,
        context.revision, context.state_digest,
    )
    if observation is None and retained_payload is not None:
        observation = revalidate_joined_predecessor_payload(
            context.root, row, payload, artifact_payload, artifact_sha256, claim,
            context.revision, context.state_digest,
            retained_execution_mapping_matches(payload, proof),
        )
    if observation is None:
        predecessor_handoff = temporary_predecessor_handoff(context, row)
        observation = execute_row(
            context.root,
            context.candidate,
            context.executor,
            row,
            artifact,
            context.refresh_mode,
            predecessor_handoff,
        )
        refreshed = (
            read_refreshed_prefix(context.root, context.candidate, int(row["phase"]))
            if row["requirement"] in {
                "P3-PREDECESSOR-01",
                "P4-PREDECESSOR-01",
                "P5-PREDECESSOR-01",
                "P6-PREDECESSOR-01",
            }
            else []
        )
        return row, observation, "execute", refreshed
    return row, retain_current_artifact(artifact, observation), "reuse", []


def temporary_predecessor_handoff(
    context: RefreshContext, row: dict[str, str]
) -> Path | None:
    requirement = row["requirement"]
    if not (
        context.refresh_mode.is_root()
        and requirement.endswith("-PREDECESSOR-01")
    ):
        return None
    phase = int(row["phase"])
    verification = PredecessorVerification(
        context.root, context.candidate, phase, context.revision, context.state_digest
    )
    publication, _ = expected_identity(verification)
    if not is_current(publication, verification):
        refresh_handoff(
            context.root, context.candidate, phase, temporary=True
        )
        publication, _ = expected_identity(verification)
    if not is_current(publication, verification):
        raise RuntimeError("temporary predecessor handoff is not current")
    return publication.destination(context.root)


def prepared_row(
    original: dict[str, str], refresh_mode: RefreshMode = RefreshMode()
) -> dict[str, str]:
    row = dict(original)
    if refresh_mode.preserves_historical_claim(row):
        return row
    proof = proofs().get(row["requirement"])
    if proof is not None:
        prepare_claim(row, proof)
    return row


def retained_payload_for_row(
    context: RefreshContext, row: dict[str, str], claim: str
) -> tuple[Path, dict[str, object], dict[str, object], str, dict[str, object] | None]:
    cached = context.row_cache.restore(
        row["requirement"], row["exact_command"], claim
    )
    artifact = context.root / row["retained_result_artifact"]
    content = artifact.read_bytes()
    artifact_payload = json.loads(content.decode("utf-8"))
    retained = context.retained.get(row["requirement"])
    if cached is not None:
        return artifact, cached, artifact_payload, str(cached["artifact_sha256"]), retained
    return (
        artifact,
        artifact_payload,
        artifact_payload,
        hashlib.sha256(content).hexdigest(),
        retained,
    )


def retained_execution_mapping_matches(
    payload: dict[str, object], proof: object
) -> bool:
    target = getattr(proof, "target", None)
    control = getattr(proof, "control", None)
    if (
        not isinstance(target, tuple)
        or len(target) != 2
        or payload.get("package") != getattr(proof, "package", None)
        or payload.get("target_kind") != target[0]
        or payload.get("target_name") != target[1]
        or payload.get("test_name") != getattr(proof, "test_name", None)
        or payload.get("features") != list(getattr(proof, "features", ()))
    ):
        return False
    observed_control = payload.get("hostile_control")
    if control is None:
        return observed_control is None
    control_target = getattr(control, "target", None)
    return isinstance(observed_control, dict) and (
        isinstance(control_target, tuple)
        and len(control_target) == 2
        and observed_control.get("package") == getattr(control, "package", None)
        and observed_control.get("target_kind") == control_target[0]
        and observed_control.get("target_name") == control_target[1]
        and observed_control.get("test_name") == getattr(control, "test_name", None)
        and observed_control.get("features") == list(getattr(control, "features", ()))
    )


def retained_observations(
    root: Path, identity: str, through_phase: int
) -> dict[str, dict[str, object]]:
    source = root / identity
    if not source.is_file():
        return {}
    try:
        payload = json.loads(source.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError):
        return {}
    rows = payload.get("rows")
    if payload.get("through_phase") != through_phase or not isinstance(rows, list):
        return {}
    retained: dict[str, dict[str, object]] = {}
    for row in rows:
        if not isinstance(row, dict) or not isinstance(row.get("requirement"), str):
            return {}
        retained[row["requirement"]] = row
    return retained

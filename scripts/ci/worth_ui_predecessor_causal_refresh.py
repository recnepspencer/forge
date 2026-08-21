from __future__ import annotations

import csv
import hashlib
import json
import os
import shutil
import subprocess
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
from worth_ui_ledger_command import CLAIM_FIELDS, source_revision
from worth_ui_ledger_execution_cache import CACHE_ENV, COMPILE_ARTIFACT
from worth_ui_ledger_durable_receipts import harvest_referenced_receipts
from worth_ui_ledger_portfolio_row import PortfolioRowExecutor
from worth_ui_ledger_portfolio_snapshot import DIGEST_ENV, REVISION_ENV
from worth_ui_ledger_row_cache import RowEvidenceCache
from worth_ui_ledger_source_state import source_state_digest
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
class RefreshContext:
    root: Path
    candidate: Path
    revision: str
    state_digest: str
    executor: PortfolioRowExecutor
    row_cache: RowEvidenceCache
    retained: dict[str, dict[str, object]]


def refresh_handoff(root: Path, ledger: Path, phase: int, identity: str) -> list[dict[str, object]]:
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
            retained_observations(root, identity, phase - 1),
        )
        persist_observation_receipts(root, state_digest, observations)
        artifact = predecessor_artifact(
            phase - 1, revision, state_digest, observations, closure_count
        )
        artifact["causal_reused_requirement_count"] = reused
        artifact["executed_requirement_count"] = executed
        write_artifact(identity, artifact)
        publish_refreshed_prefix(root, ledger, observations, prepared_row)
        print(
            f"[predecessor:complete] through_phase={phase - 1} "
            f"reused={reused} executed={executed}",
            file=sys.stderr,
            flush=True,
        )
        return observations
    finally:
        restore_environment(previous)


def ensure_compile_artifact(root: Path, revision: str, state_digest: str) -> None:
    identity = root / COMPILE_ARTIFACT
    try:
        retained = json.loads(identity.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError):
        retained = {}
    if (
        retained.get("exit_posture") == "passed"
        and retained.get("source_revision") == revision
        and retained.get("source_state_digest") == state_digest
    ):
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


def persist_observation_receipts(
    root: Path, state_digest: str, observations: list[dict[str, object]]
) -> None:
    for observation in observations:
        receipts = observation.get("execution_receipts")
        if not isinstance(receipts, list):
            raise RuntimeError("predecessor row omits its execution receipts")
        reuse = observation.get("causal_reuse")
        receipt_state = (
            reuse.get("predecessor_source_state_digest")
            if isinstance(reuse, dict)
            else state_digest
        )
        if not isinstance(receipt_state, str):
            raise RuntimeError("predecessor row omits its receipt source state")
        harvest_referenced_receipts(root, receipt_state, receipts)


def current_observations(
    root: Path,
    ledger: Path,
    through_phase: int,
    revision: str,
    state_digest: str,
    retained: dict[str, dict[str, object]] | None = None,
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
    with tempfile.NamedTemporaryFile(delete=False, dir=target) as stream:
        candidate = Path(stream.name)
    shutil.copyfile(ledger, candidate)
    try:
        context = RefreshContext(
            root, candidate, revision, state_digest, executor, row_cache, retained
        )
        for original in ordered:
            started = time.perf_counter_ns()
            row, observation, disposition, refreshed = settle_row(context, original)
            import_refreshed_observations(
                rows_by_requirement, refreshed, observations, settled, prepared_row
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
    row = prepared_row(original)
    proof = proofs().get(row["requirement"])
    claim = row_claim_digest(row)
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
        observation = execute_row(
            context.root, context.candidate, context.executor, row, artifact
        )
        refreshed = (
            read_refreshed_prefix(context.root, context.candidate, int(row["phase"]))
            if row["requirement"] in {"P3-PREDECESSOR-01", "P4-PREDECESSOR-01"}
            else []
        )
        return row, observation, "execute", refreshed
    return row, retain_current_artifact(artifact, observation), "reuse", []


def prepared_row(original: dict[str, str]) -> dict[str, str]:
    row = dict(original)
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


def execute_row(
    root: Path,
    ledger: Path,
    executor: PortfolioRowExecutor,
    row: dict[str, str],
    artifact: Path,
) -> dict[str, object]:
    candidate_root = root / "workspaces/worth-ui/target/milestone-3141-candidates"
    candidate_root.mkdir(parents=True, exist_ok=True)
    with tempfile.NamedTemporaryFile(delete=False, dir=candidate_root) as stream:
        candidate = Path(stream.name)
    shutil.copyfile(ledger, candidate)
    try:
        return executor(
            row,
            artifact,
            COMPILE_ARTIFACT,
            candidate_ledger=candidate,
        )
    finally:
        candidate.unlink(missing_ok=True)


def closure_tests(root: Path, ledger: Path, through_phase: int) -> int:
    prefix = [
        "cargo",
        "test",
        "--manifest-path",
        "workspaces/worth-ui/Cargo.toml",
        "-p",
        "worth-ui-certification",
        "--test",
        "topology_contracts",
    ]
    tests = [
        "milestone_3141_phase1_ledger::mutation_tests::milestone_ledger_has_exact_schema_inventory_and_honest_posture",
        {
            2: "phase_two_closure_requires_every_phase_one_and_two_row",
            3: "phase_three_closure_requires_every_predecessor_and_phase_three_row",
            4: "phase_four_closure_requires_every_predecessor_and_phase_four_row",
        }[through_phase],
    ]
    for index, name in enumerate(tests):
        command = [*prefix, f"milestone_3141_phase1_ledger::{name}", "--", "--exact"]
        if index == 1:
            command.append("--ignored")
        command.append("--nocapture")
        environment = dict(os.environ)
        environment["WORTH_UI_MILESTONE_3141_LEDGER"] = str(ledger.resolve())
        completed = subprocess.run(command, cwd=root, env=environment, check=False)
        if completed.returncode != 0:
            raise RuntimeError("current-source predecessor closure check failed")
    return 2


def governed_rows(ledger: Path, through_phase: int) -> list[dict[str, str]]:
    with ledger.open(encoding="utf-8", newline="") as stream:
        rows = [
            row for row in csv.DictReader(stream) if int(row["phase"]) <= through_phase
        ]
    expected = {2: 30, 3: 47, 4: 68}[through_phase]
    if len(rows) != expected or any(
        row["result"] != "PROVED" or row["final_source"] != "true" for row in rows
    ):
        raise RuntimeError("predecessor causal refresh requires a proved prefix")
    return rows


def row_claim_digest(row: dict[str, str]) -> str:
    result = hashlib.sha256()
    for field in CLAIM_FIELDS:
        result.update(field.encode("utf-8"))
        result.update(b"\0")
        result.update(row[field].encode("utf-8"))
        result.update(b"\0")
    return result.hexdigest()


def environment_snapshot() -> dict[str, str | None]:
    return {
        name: os.environ.get(name) for name in (CACHE_ENV, REVISION_ENV, DIGEST_ENV)
    }


def restore_environment(previous: dict[str, str | None]) -> None:
    for name, value in previous.items():
        if value is None:
            os.environ.pop(name, None)
        else:
            os.environ[name] = value

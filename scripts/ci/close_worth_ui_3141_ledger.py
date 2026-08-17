from __future__ import annotations

import argparse
import csv
import os
import subprocess
import sys
import tempfile
from pathlib import Path

from worth_ui_3141_proof_plan import (
    COMPILE_ARTIFACT,
    prepare_claim,
    proofs,
)
from worth_ui_ledger_artifact_transaction import ArtifactTransaction
from worth_ui_ledger_command import claim_digest, source_revision
from worth_ui_ledger_execution_cache import CACHE_ENV
from worth_ui_ledger_portfolio_snapshot import DIGEST_ENV, REVISION_ENV
from worth_ui_ledger_row_cache import RowEvidenceCache
from worth_ui_ledger_row_execution import execute_or_restore, run_row
from worth_ui_ledger_retained_portfolio import portfolio_identity, publish
from worth_ui_ledger_source_state import source_state_digest
from worth_ui_ledger_closure_storage import (
    ledger_lock as acquire_ledger_lock,
    render_requirement_update,
    write_requirements as write_ledger_requirements,
)


ROOT = Path(__file__).resolve().parents[2]
LEDGER = ROOT / "_docs/worth-ui/milestone-3.14.1-proof-ledger.csv"
LEDGER_LOCK = LEDGER.with_suffix(".lock")


def ledger_lock(identity: Path = LEDGER_LOCK):
    return acquire_ledger_lock(identity)


def close_row(row: dict[str, str], result: dict[str, object]) -> None:
    for field in [
        "matched_test_count", "source_revision", "source_digest",
        "source_state_digest", "run_nonce",
    ]:
        row[field] = str(result[field])
    row["command_result"] = "passed"
    row["result_artifact_digest"] = str(result["artifact_sha256"])
    row["result"] = "PROVED"
    row["final_source"] = "true"


def run(command_text: str, candidate_ledger: Path | None = None) -> dict[str, object]:
    return run_row(ROOT, command_text, candidate_ledger)


def write_requirements(
    rows: list[dict[str, str]], fields: list[str], requirements: set[str]
) -> None:
    write_ledger_requirements(LEDGER, rows, fields, requirements)


def main() -> int:
    arguments = parse_arguments()
    with ledger_lock():
        return governed_main(arguments)


def governed_main(arguments: argparse.Namespace) -> int:
    prepare_only = arguments.prepare_only
    through_phase = arguments.through_phase
    requirement = arguments.requirement
    fields, rows = read_ledger()
    configured = phase_proofs(through_phase)
    current_state = source_state_digest(source_revision())
    selected = phase_rows_to_prepare(
        rows, through_phase, requirement, configured, current_state
    )
    if requirement is None:
        require_complete_phase_mapping(rows, through_phase, configured)
    for row in selected:
        reopen_claim(row, configured[row["requirement"]])
    if selected:
        write_requirements(rows, fields, {row["requirement"] for row in selected})
    if prepare_only:
        print(f"prepared {len(selected)} Worth UI milestone 3.14.1 ledger claims as OPEN")
        return 0
    if not selected:
        verify_closed_prefix(through_phase)
        return 0
    if through_phase == 2:
        close_phase_two(rows, fields)
    elif through_phase == 3:
        close_phase_three(rows, fields, selected)
    elif through_phase == 4:
        close_phase_four(rows, fields, selected)
    elif through_phase == 5:
        close_phase_five(rows, fields, selected)
    else:
        raise RuntimeError("unsupported Worth UI milestone phase")
    return 0


def reopen_claim(row: dict[str, str], proof: object) -> None:
    previous_artifact = row.get("result_artifact_digest")
    prepare_claim(row, proof)
    row.update(
        {
            "matched_test_count": "0",
            "command_result": "not-run",
            "source_revision": "not-bound",
            "source_digest": "not-bound",
            "source_state_digest": "not-bound",
            "run_nonce": "not-bound",
            "result_artifact_digest": "not-bound",
            "result": "OPEN",
            "final_source": "false",
            "reopen_lineage": (
                f"supersedes:{previous_artifact}"
                if previous_artifact and previous_artifact != "not-bound"
                else row.get("reopen_lineage", "none")
            ),
        }
    )


def read_ledger() -> tuple[list[str], list[dict[str, str]]]:
    with LEDGER.open(encoding="utf-8", newline="") as stream:
        reader = csv.DictReader(stream)
        fields = list(reader.fieldnames or ())
        return fields, list(reader)


def phase_proofs(phase: int) -> dict[str, object]:
    return {
        requirement: proof
        for requirement, proof in proofs().items()
        if int(requirement[1]) == phase
    }


def phase_rows_to_prepare(
    rows: list[dict[str, str]],
    through_phase: int,
    requirement: str | list[str] | None,
    configured: dict[str, object],
    current_state: str | None = None,
) -> list[dict[str, str]]:
    predecessor = [row for row in rows if int(row["phase"]) < through_phase]
    if any(row["result"] != "PROVED" or row["final_source"] != "true" for row in predecessor):
        raise RuntimeError("cannot prepare a phase before predecessor closure")
    candidates = [
        row for row in rows
        if int(row["phase"]) == through_phase
        and (
            (row["result"] == "OPEN" and row["final_source"] == "false")
            or (
                through_phase > 2
                and current_state is not None
                and row["result"] == "PROVED"
                and row["final_source"] == "true"
                and row.get("source_state_digest") != current_state
            )
        )
    ]
    if requirement is None:
        return candidates
    requested = [requirement] if isinstance(requirement, str) else list(requirement)
    if len(requested) != len(set(requested)):
        raise RuntimeError("duplicate Phase requirement selection")
    unmapped = [identity for identity in requested if identity not in configured]
    if unmapped:
        raise RuntimeError(f"{unmapped[0]} has no governed proof mapping")
    selected = [row for row in candidates if row["requirement"] in requested]
    selected_identities = {row["requirement"] for row in selected}
    unavailable = [identity for identity in requested if identity not in selected_identities]
    if unavailable:
        raise RuntimeError(
            f"{unavailable[0]} is not one open Phase {through_phase} row"
        )
    return selected


def require_complete_phase_mapping(
    rows: list[dict[str, str]], phase: int, configured: dict[str, object]
) -> None:
    inventory = {row["requirement"] for row in rows if int(row["phase"]) == phase}
    if set(configured) != inventory:
        missing = sorted(inventory - set(configured))
        raise RuntimeError(f"Phase {phase} proof mappings are incomplete: {missing}")


def close_selected(
    rows: list[dict[str, str]], fields: list[str], selected: list[dict[str, str]]
) -> None:
    for row in selected:
        close_row(row, run(row["exact_command"]))
        write_requirements(rows, fields, {row["requirement"]})


def close_selected_atomically(
    rows: list[dict[str, str]],
    fields: list[str],
    selected: list[dict[str, str]],
    verify_phase: int | None = None,
) -> None:
    original = LEDGER.read_text(encoding="utf-8")
    revision = source_revision()
    state_digest = source_state_digest(revision)
    cache_root = (
        ROOT / "workspaces/worth-ui/target/milestone-3141-execution-cache" / state_digest
    )
    row_cache = RowEvidenceCache(
        ROOT, cache_root, original.encode("utf-8"), revision, state_digest
    )
    previous_cache = os.environ.get(CACHE_ENV)
    previous_revision = os.environ.get(REVISION_ENV)
    previous_digest = os.environ.get(DIGEST_ENV)
    os.environ[CACHE_ENV] = str(cache_root)
    os.environ[REVISION_ENV] = revision
    os.environ[DIGEST_ENV] = state_digest
    artifacts = ArtifactTransaction(
        ROOT,
        LEDGER,
        [row["exact_command"] for row in selected],
        () if verify_phase is None else (portfolio_identity(verify_phase),),
    )
    candidate_root = ROOT / "workspaces/worth-ui/target/milestone-3141-candidates"
    candidate_root.mkdir(parents=True, exist_ok=True)
    with tempfile.NamedTemporaryFile(
        "w", encoding="utf-8", newline="", dir=candidate_root, delete=False
    ) as stream:
        stream.write(original)
        candidate = Path(stream.name)
    try:
        requirements = {row["requirement"] for row in selected}
        for row in selected:
            claim = claim_digest(row["requirement"])
            result = execute_or_restore(row, candidate, row_cache, claim, run)
            close_row(row, result)
            candidate.write_text(
                render_requirement_update(original, rows, fields, requirements),
                encoding="utf-8",
                newline="",
            )
        validate_ledger_posture(candidate)
        if source_revision() != revision or source_state_digest(revision) != state_digest:
            raise RuntimeError("governed source changed during phase closure")
        if verify_phase is not None:
            publish(ROOT, candidate, verify_phase, revision, state_digest)
            verify_closed_prefix(verify_phase, candidate)
        candidate_bytes = candidate.read_bytes()
        artifacts.prepare_commit(candidate_bytes)
        candidate.replace(LEDGER)
        artifacts.commit()
    except BaseException:
        artifacts.rollback()
        raise
    finally:
        if previous_cache is None:
            os.environ.pop(CACHE_ENV, None)
        else:
            os.environ[CACHE_ENV] = previous_cache
        restore_environment(REVISION_ENV, previous_revision)
        restore_environment(DIGEST_ENV, previous_digest)
        candidate.unlink(missing_ok=True)


def restore_environment(name: str, value: str | None) -> None:
    if value is None:
        os.environ.pop(name, None)
    else:
        os.environ[name] = value


def close_phase_two(rows: list[dict[str, str]], fields: list[str]) -> None:
    subprocess.run(
        [sys.executable, "scripts/ci/run_worth_ui_compile_contracts.py", "--artifact", COMPILE_ARTIFACT],
        cwd=ROOT,
        check=True,
    )
    excluded = {"P1-CLOSE-01", "P1-WORLDS-01", "P1-HEADLESS-COST-01"}
    phase_one = [row for row in rows if row["phase"] == "1" and row["requirement"] not in excluded]
    named = {row["requirement"]: row for row in rows}
    phase_two = [
        row for row in rows
        if row["phase"] == "2" and row["requirement"] != "P2-WORLD-01"
    ]
    ordered = phase_one + [
        named["P1-WORLDS-01"], named["P1-HEADLESS-COST-01"], named["P1-CLOSE-01"],
        named["P2-WORLD-01"], *phase_two,
    ]
    close_selected(rows, fields, ordered)
    verify_closed_prefix(2)


def close_phase_three(
    rows: list[dict[str, str]], fields: list[str], selected: list[dict[str, str]]
) -> None:
    close_phase_with_priority(
        rows, fields, selected, ["P3-PREDECESSOR-01", "P3-DELTA-SOURCE-01", "P3-HP02-WORLD-01"],
        "P3-CLOSE-01", 3,
    )


def close_phase_four(
    rows: list[dict[str, str]], fields: list[str], selected: list[dict[str, str]]
) -> None:
    close_phase_with_priority(
        rows, fields, selected, ["P4-PREDECESSOR-01", "P4-TEXT-PROFILE-01"], "P4-CLOSE-01", 4
    )


def close_phase_five(
    rows: list[dict[str, str]], fields: list[str], selected: list[dict[str, str]]
) -> None:
    requirements = {row["requirement"] for row in selected}
    verify_phase = 5 if "P5-CLOSE-01" in requirements else None
    close_selected_atomically(rows, fields, selected, verify_phase=verify_phase)


def close_phase_with_priority(
    rows: list[dict[str, str]],
    fields: list[str],
    selected: list[dict[str, str]],
    priority: list[str],
    close_requirement: str,
    verify_phase: int,
) -> None:
    by_requirement = {row["requirement"]: row for row in selected}
    ordered = [by_requirement.pop(identity) for identity in priority if identity in by_requirement]
    close = by_requirement.pop(close_requirement, None)
    ordered.extend(by_requirement.values())
    if close is not None:
        ordered.append(close)
    close_selected_atomically(rows, fields, ordered, verify_phase=verify_phase)


def validate_ledger_posture(candidate: Path | None = None) -> None:
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
        cwd=ROOT,
        env=environment,
        check=True,
    )


def verify_closed_prefix(phase: int, candidate: Path | None = None) -> None:
    environment = dict(os.environ)
    if candidate is not None:
        environment["WORTH_UI_MILESTONE_3141_LEDGER"] = str(candidate.resolve())
    subprocess.run(
        [sys.executable, "scripts/ci/verify_worth_ui_3141_ledger.py", "--through-phase", str(phase)],
        cwd=ROOT,
        env=environment,
        check=True,
    )


def parse_arguments() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Atomically close Worth UI milestone 3.14.1 ledger evidence"
    )
    parser.add_argument("--through-phase", type=int, choices=(2, 3, 4, 5), default=2)
    parser.add_argument(
        "--requirement",
        action="append",
        help="repeat to refresh multiple Phase rows in one atomic transaction",
    )
    parser.add_argument("--prepare-only", action="store_true")
    return parser.parse_args()


if __name__ == "__main__":
    raise SystemExit(main())

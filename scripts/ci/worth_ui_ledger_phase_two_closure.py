from __future__ import annotations

import hashlib
import json
import re
from pathlib import Path

from worth_ui_ledger_atomic_closure import AtomicClosurePlan, ClosurePreparation


LINEAGE_PATTERN = re.compile(
    r"(?:^|;)invalidation:(?P<path>[^;@]+)@(?P<digest>[0-9a-f]{64})(?:;|$)"
)


def phase_two_closure_plan(
    rows: list[dict[str, str]], root: Path
) -> AtomicClosurePlan:
    admit_invalidated_phase_one_prefix(rows, root)
    return AtomicClosurePlan(
        tuple(ordered_phase_one_two_rows(rows)),
        2,
        ClosurePreparation.CURRENT_COMPILE_CONTRACTS,
    )


def ordered_phase_one_two_rows(
    rows: list[dict[str, str]],
) -> list[dict[str, str]]:
    named = {row["requirement"]: row for row in rows}
    excluded = {"P1-CLOSE-01", "P1-WORLDS-01", "P1-HEADLESS-COST-01"}
    independent = [
        row
        for row in rows
        if row["phase"] == "1" and row["requirement"] not in excluded
    ]
    phase_two = [
        row
        for row in rows
        if row["phase"] == "2" and row["requirement"] != "P2-WORLD-01"
    ]
    return independent + [
        named["P1-WORLDS-01"],
        named["P1-HEADLESS-COST-01"],
        named["P1-CLOSE-01"],
        named["P2-WORLD-01"],
        *phase_two,
    ]


def admit_invalidated_phase_one_prefix(
    rows: list[dict[str, str]], root: Path
) -> list[dict[str, str]]:
    phase_one = [row for row in rows if row["phase"] == "1"]
    if all(_proved(row) for row in phase_one):
        return []
    if not phase_one or not all(_open(row) for row in phase_one):
        raise RuntimeError("Phase 1 predecessor has a mixed or ungoverned open posture")
    if any(_proved(row) for row in rows if int(row["phase"]) > 1):
        raise RuntimeError("proved descendants cannot bypass invalidated Phase 1")
    receipt_path, receipt_digest = _shared_receipt_binding(phase_one)
    receipt_bytes = _repository_file(root, receipt_path).read_bytes()
    if hashlib.sha256(receipt_bytes).hexdigest() != receipt_digest:
        raise RuntimeError("Phase 1 invalidation receipt digest does not match lineage")
    receipt = json.loads(receipt_bytes)
    _validate_receipt(receipt, phase_one, rows, root)
    return phase_one


def _shared_receipt_binding(rows: list[dict[str, str]]) -> tuple[str, str]:
    bindings = set()
    for row in rows:
        matched = LINEAGE_PATTERN.search(row.get("reopen_lineage", ""))
        if matched is None:
            raise RuntimeError("open Phase 1 row lacks governed invalidation lineage")
        bindings.add((matched.group("path"), matched.group("digest")))
    if len(bindings) != 1:
        raise RuntimeError("open Phase 1 rows do not share one invalidation receipt")
    return bindings.pop()


def _validate_receipt(
    receipt: dict[str, object],
    phase_one: list[dict[str, str]],
    rows: list[dict[str, str]],
    root: Path,
) -> None:
    if receipt.get("schema") != "worth-ui-ledger-phase-invalidation-v2":
        raise RuntimeError("Phase 1 bootstrap requires a v2 invalidation receipt")
    if receipt.get("phase") != 1:
        raise RuntimeError("invalidation receipt does not govern Phase 1")
    retained = receipt.get("invalidated_rows")
    if not isinstance(retained, list):
        raise RuntimeError("Phase 1 invalidation receipt omits invalidated rows")
    by_requirement = {
        item.get("requirement"): item for item in retained if isinstance(item, dict)
    }
    if set(by_requirement) != {row["requirement"] for row in phase_one}:
        raise RuntimeError("Phase 1 invalidation receipt has the wrong row inventory")
    for row in phase_one:
        prior = by_requirement[row["requirement"]].get("prior_result_artifact_digest")
        if not isinstance(prior, str) or f"supersedes:{prior}" not in row["reopen_lineage"]:
            raise RuntimeError("Phase 1 invalidation lineage omits its prior digest")
    _validate_incident_archive(receipt, root)
    expected_open = {
        row["requirement"] for row in rows if int(row["phase"]) > 1 and _open(row)
    }
    preserved = receipt.get("preserved_open_requirements")
    if not isinstance(preserved, list) or set(preserved) != expected_open:
        raise RuntimeError("Phase 1 invalidation receipt has the wrong open suffix")


def _validate_incident_archive(receipt: dict[str, object], root: Path) -> None:
    incident = receipt.get("incident")
    if not isinstance(incident, dict):
        raise RuntimeError("Phase 1 invalidation receipt omits its incident")
    archive = incident.get("superseded_artifact")
    digest = incident.get("observed_artifact_sha256")
    if not isinstance(archive, str) or not isinstance(digest, str):
        raise RuntimeError("Phase 1 invalidation incident is incomplete")
    if hashlib.sha256(_repository_file(root, archive).read_bytes()).hexdigest() != digest:
        raise RuntimeError("Phase 1 invalidation archive digest does not match")


def _repository_file(root: Path, identity: str) -> Path:
    candidate = (root / identity).resolve()
    candidate.relative_to(root.resolve())
    return candidate


def _proved(row: dict[str, str]) -> bool:
    return row["result"] == "PROVED" and row["final_source"] == "true"


def _open(row: dict[str, str]) -> bool:
    return row["result"] == "OPEN" and row["final_source"] == "false"

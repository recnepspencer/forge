from __future__ import annotations

import csv
import hashlib
import json
from pathlib import Path

from worth_ui_3141_proof_plan import prepare_claim, proofs
from worth_ui_ledger_artifact_identity import requirement_phase
from worth_ui_ledger_command import CLAIM_FIELDS, claim_digest_for_row, source_digest
from worth_ui_ledger_causal_revalidation import reusable_payload, validate_causal_reuse


ROOT = Path(__file__).resolve().parents[2]
LEDGER = ROOT / "_docs/worth-ui/milestone-3.14.1-proof-ledger.csv"
ROW_EVIDENCE_DEPENDENCIES = {
    "P5-PREDECESSOR-01": ("P5-TEXT-ASYNC-PRESENTATION-01",),
    "P5-ATLAS-PINNING-01": ("P5-ATLAS-01",),
}
CURRENT_SOURCE_PHASE = 6


def reopen_claim(row: dict[str, str], proof: object) -> None:
    previous_artifact = row.get("result_artifact_digest")
    prepare_claim(row, proof)
    reopen_prepared_claim(row, previous_artifact)


def reopen_prepared_claim(
    row: dict[str, str], previous_artifact: str | None
) -> None:
    lineage = row.get("reopen_lineage", "none")
    if previous_artifact and previous_artifact != "not-bound":
        successor = f"supersedes:{previous_artifact}"
        lineage = successor if lineage == "none" else f"{lineage};{successor}"
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
            "reopen_lineage": lineage,
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
        if requirement_phase(requirement) == phase
    }


def phase_rows_to_prepare(
    rows: list[dict[str, str]],
    through_phase: int,
    requirement: str | list[str] | None,
    configured: dict[str, object],
    current_state: str | None = None,
    prepare: object = prepare_claim,
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
                and row["result"] == "PROVED"
                and row["final_source"] == "true"
                and not row_has_current_causal_binding(
                    row, configured[row["requirement"]], current_state, prepare
                )
            )
        )
    ]
    if requirement is None:
        candidates = include_row_evidence_dependencies(rows, candidates)
        return include_phase_close_dependency(rows, through_phase, candidates)
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
    selected = include_row_evidence_dependencies(rows, selected)
    return include_phase_close_dependency(rows, through_phase, selected)


def row_has_current_causal_binding(
    row: dict[str, str], proof: object, current_state: str | None, prepare: object = prepare_claim
) -> bool:
    if (
        int(row["phase"]) == CURRENT_SOURCE_PHASE
        and (current_state is None or row.get("source_state_digest") != current_state)
    ):
        return False
    expected = dict(row)
    prepare(expected, proof)
    exact_fields = (*CLAIM_FIELDS, "exact_command", "retained_result_artifact")
    if any(row.get(field) != expected.get(field) for field in exact_fields):
        return False
    identities = expected.get("source_identity")
    if not identities:
        return current_state is not None and row.get("source_state_digest") == current_state
    try:
        sources = identities.split(";")
        current_source_digest = source_digest(tuple(sources))
        if row.get("source_digest") != current_source_digest:
            return False
        artifact = ROOT / expected["retained_result_artifact"]
        content = artifact.read_bytes()
        payload = json.loads(content.decode("utf-8"))
        if row.get("result_artifact_digest") != hashlib.sha256(content).hexdigest():
            return False
        if not reusable_payload(
            ROOT,
            expected,
            payload,
            claim_digest_for_row(expected),
            sources,
            current_source_digest,
        ):
            return False
        causal_reuse = validate_causal_reuse(
            ROOT,
            payload,
            str(payload.get("source_revision", "")),
            str(payload.get("source_state_digest", "")),
        )
        if (
            current_state is None
            or (
                payload.get("source_state_digest") != current_state
                and causal_reuse is None
            )
        ):
            return False
        return True
    except (OSError, RuntimeError, TypeError, ValueError, json.JSONDecodeError):
        return False


def include_phase_close_dependency(
    rows: list[dict[str, str]],
    phase: int,
    selected: list[dict[str, str]],
) -> list[dict[str, str]]:
    close_requirement = f"P{phase}-CLOSE-01"
    if not selected or all(row["requirement"] == close_requirement for row in selected):
        return selected
    if any(row["requirement"] == close_requirement for row in selected):
        return selected
    close = next(
        (row for row in rows if row["requirement"] == close_requirement), None
    )
    if close is None:
        return selected
    if close["result"] == "PROVED" and close["final_source"] == "true":
        return [*selected, close]
    return selected


def include_row_evidence_dependencies(
    rows: list[dict[str, str]], selected: list[dict[str, str]]
) -> list[dict[str, str]]:
    required = {row["requirement"] for row in selected}
    pending = list(required)
    while pending:
        requirement = pending.pop()
        for dependency in ROW_EVIDENCE_DEPENDENCIES.get(requirement, ()):
            if dependency not in required:
                required.add(dependency)
                pending.append(dependency)
    return [row for row in rows if row["requirement"] in required]


def require_complete_phase_mapping(
    rows: list[dict[str, str]], phase: int, configured: dict[str, object]
) -> None:
    inventory = {row["requirement"] for row in rows if int(row["phase"]) == phase}
    if set(configured) != inventory:
        missing = sorted(inventory - set(configured))
        raise RuntimeError(f"Phase {phase} proof mappings are incomplete: {missing}")

def reopen_proved_downstream(
    rows: list[dict[str, str]], through_phase: int
) -> list[dict[str, str]]:
    configured = proofs()
    downstream = [
        row
        for row in rows
        if int(row["phase"]) > through_phase
        and (row["result"] == "PROVED" or row["final_source"] == "true")
    ]
    for row in downstream:
        reopen_claim(row, configured[row["requirement"]])
    return downstream

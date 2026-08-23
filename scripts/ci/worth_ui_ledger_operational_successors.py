from __future__ import annotations

import csv
import json
from pathlib import Path

from worth_ui_3141_proof_plan import prepare_claim, proofs


def prepared_open_rows(rows: list[dict[str, str]]) -> dict[str, dict[str, str]]:
    prepared = {}
    configured = proofs()
    for row in rows:
        current = dict(row)
        proof = configured.get(row["requirement"])
        if proof is not None:
            prepare_claim(current, proof)
        current["result"] = "OPEN"
        current["final_source"] = "false"
        prepared[row["requirement"]] = current
    return prepared


def proved_execution_row(
    row: dict[str, str], artifact: str, payload: dict[str, object]
) -> dict[str, str]:
    current = execution_claim_row(
        row,
        artifact,
        exact_command=str(payload["executed_exact_command"]),
        sources=payload["source_identity"],
    )
    for field in (
        "matched_test_count", "source_revision", "source_digest",
        "source_state_digest", "run_nonce",
    ):
        current[field] = str(payload[field])
    current["result_artifact_digest"] = str(payload["artifact_sha256"])
    current["command_result"] = "passed"
    current["result"] = "PROVED"
    current["final_source"] = "true"
    return current


def execution_claim_row(
    row: dict[str, str],
    artifact: str,
    exact_command: str | None = None,
    sources: object | None = None,
    preserve_claim: bool = False,
) -> dict[str, str]:
    current = dict(row)
    proof = proofs().get(row["requirement"])
    if proof is not None and not preserve_claim:
        prepare_claim(current, proof)
    if exact_command is None:
        words = current["exact_command"].split()
        words[words.index("--artifact") + 1] = artifact
        exact_command = " ".join(words)
    current["exact_command"] = exact_command
    if sources is not None and not preserve_claim:
        if not isinstance(sources, list) or not all(
            isinstance(source, str) for source in sources
        ):
            raise RuntimeError("executed claim has invalid source identities")
        current["source_identity"] = ";".join(sources)
    current["retained_failure_artifact"] = artifact
    current["retained_result_artifact"] = artifact
    current["result"] = "OPEN"
    current["final_source"] = "false"
    return current


def stage_execution_claim(
    candidate: Path,
    row: dict[str, str],
    artifact: str,
    command: list[str],
    *,
    preserve_claim: bool = False,
) -> None:
    with candidate.open(encoding="utf-8", newline="") as stream:
        reader = csv.DictReader(stream)
        fields = list(reader.fieldnames or ())
        complete = list(reader)
    matches = [item for item in complete if item["requirement"] == row["requirement"]]
    if len(matches) != 1:
        raise RuntimeError("candidate ledger omits the exact staged requirement")
    sources = [command[index + 1] for index, word in enumerate(command) if word == "--source"]
    staged = execution_claim_row(
        row,
        artifact,
        " ".join(command),
        sources,
        preserve_claim=preserve_claim,
    )
    with candidate.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=fields, lineterminator="\n")
        writer.writeheader()
        for item in complete:
            writer.writerow(staged if item["requirement"] == row["requirement"] else item)


def record_proved_execution(
    row: dict[str, str],
    artifact: str,
    payload: dict[str, object],
    replacements: dict[str, dict[str, str]],
    ledger: Path,
    candidate: Path,
) -> None:
    replacements[row["requirement"]] = proved_execution_row(row, artifact, payload)
    write_candidate_ledger(ledger, candidate, replacements)


def write_candidate_ledger(
    ledger: Path, identity: Path, replacements: dict[str, dict[str, str]]
) -> None:
    with ledger.open(encoding="utf-8", newline="") as stream:
        reader = csv.DictReader(stream)
        fields = list(reader.fieldnames or ())
        complete = list(reader)
    with identity.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=fields, lineterminator="\n")
        writer.writeheader()
        for row in complete:
            writer.writerow(replacements.get(row["requirement"], row))


def predecessor_observations(
    observation: dict[str, object],
    temporary: Path,
    root: Path,
    expected_requirements: set[str],
) -> list[dict[str, object]]:
    sources = observation.get("source_identity")
    if not isinstance(sources, list):
        raise RuntimeError("predecessor execution omits its source inventory")
    matches = [
        source for source in sources
        if isinstance(source, str) and source.endswith("p3-predecessor-handoff.json")
    ]
    if len(matches) != 1:
        raise RuntimeError("predecessor execution does not bind one fresh handoff")
    identity = (root / matches[0]).resolve()
    try:
        identity.relative_to(temporary.resolve())
    except ValueError as error:
        raise RuntimeError("predecessor handoff is outside the operational portfolio") from error
    payload = json.loads(identity.read_text(encoding="utf-8"))
    rows = payload.get("rows")
    through_phase = payload.get("through_phase")
    expected_schema = "worth-ui-phase-predecessor-handoff-v4"
    if (
        payload.get("schema") != expected_schema
        or through_phase != 2
        or payload.get("source_revision") != observation.get("source_revision")
        or payload.get("source_state_digest") != observation.get("source_state_digest")
        or not isinstance(rows, list)
    ):
        raise RuntimeError("predecessor handoff does not describe this fresh portfolio")
    requirements = {
        row.get("requirement") for row in rows if isinstance(row, dict)
    }
    if (
        len(rows) != len(expected_requirements)
        or requirements != expected_requirements
        or any(
            not isinstance(row, dict) or row.get("exit_posture") != "passed"
            for row in rows
        )
    ):
        raise RuntimeError("predecessor handoff rows are incomplete or non-passing")
    return rows


def relative(root: Path, identity: Path) -> str:
    return identity.resolve().relative_to(root.resolve()).as_posix()


def find(rows: list[dict[str, str]], requirement: str) -> dict[str, str] | None:
    return next((row for row in rows if row["requirement"] == requirement), None)


def shared_artifact(
    requirement: str, mixed_world: str | None, native_world: str | None
) -> str | None:
    if requirement in {"P3-HEADLESS-COST-01", "P3-PRODUCER-SLOPE-01"}:
        return mixed_world
    return native_world if proofs()[requirement].shared_main else None

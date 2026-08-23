from __future__ import annotations

import csv
import hashlib
import json
import tempfile
from pathlib import Path
from typing import Callable


def retain_current_artifact(
    identity: Path, observation: dict[str, object]
) -> dict[str, object]:
    retained = dict(observation)
    retained.pop("artifact_sha256", None)
    identity.parent.mkdir(parents=True, exist_ok=True)
    with tempfile.NamedTemporaryFile(
        "w", encoding="utf-8", dir=identity.parent, delete=False
    ) as stream:
        json.dump(retained, stream, indent=2)
        stream.write("\n")
        temporary = Path(stream.name)
    temporary.replace(identity)
    retained["artifact_sha256"] = hashlib.sha256(identity.read_bytes()).hexdigest()
    return retained


def write_candidate_ledger(
    source: Path,
    candidate: Path,
    settled: dict[str, tuple[dict[str, str], dict[str, object]]],
) -> None:
    with source.open(encoding="utf-8", newline="") as stream:
        reader = csv.DictReader(stream)
        fields = list(reader.fieldnames or ())
        rows = list(reader)
    settled_phase = max(int(prepared["phase"]) for prepared, _ in settled.values())
    for row in rows:
        if int(row["phase"]) > settled_phase:
            reopen_downstream(row)
        current = settled.get(row["requirement"])
        if current is not None:
            settle_current(row, *current)
    with candidate.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=fields, lineterminator="\r\n")
        writer.writeheader()
        writer.writerows(rows)


def publish_refreshed_prefix(
    root: Path,
    candidate: Path,
    observations: list[dict[str, object]],
    prepare: Callable[[dict[str, str]], dict[str, str]],
) -> None:
    canonical = root / "_docs/worth-ui/milestone-3.14.1-proof-ledger.csv"
    if candidate.resolve() == canonical.resolve():
        return
    with candidate.open(encoding="utf-8", newline="") as stream:
        rows = list(csv.DictReader(stream))
    by_requirement = {row["requirement"]: row for row in rows}
    settled = {}
    for observation in observations:
        requirement = observation.get("requirement")
        if not isinstance(requirement, str) or requirement not in by_requirement:
            raise RuntimeError("refreshed predecessor returned an unknown requirement")
        settled[requirement] = (prepare(by_requirement[requirement]), observation)
    write_candidate_ledger(candidate, candidate, settled)


def import_refreshed_observations(
    rows_by_requirement: dict[str, dict[str, str]],
    refreshed: list[dict[str, object]],
    observations: dict[str, dict[str, object]],
    settled: dict[str, tuple[dict[str, str], dict[str, object]]],
    prepare: Callable[[dict[str, str]], dict[str, str]],
) -> None:
    for observation in refreshed:
        requirement = observation.get("requirement")
        if not isinstance(requirement, str) or requirement not in rows_by_requirement:
            raise RuntimeError("nested predecessor returned an unknown requirement")
        row = prepare(rows_by_requirement[requirement])
        observations[requirement] = observation
        settled[requirement] = (row, observation)


def read_refreshed_prefix(
    root: Path, candidate: Path, phase: int
) -> list[dict[str, object]]:
    with candidate.open(encoding="utf-8", newline="") as stream:
        rows = [row for row in csv.DictReader(stream) if int(row["phase"]) < phase]
    observations = []
    for row in rows:
        identity = root / row["retained_result_artifact"]
        content = identity.read_bytes()
        payload = json.loads(content.decode("utf-8"))
        digest = hashlib.sha256(content).hexdigest()
        if payload.get("requirement") != row["requirement"]:
            raise RuntimeError("refreshed predecessor artifact has the wrong identity")
        payload["artifact_sha256"] = digest
        observations.append(payload)
    return observations


def import_candidate_prefix(
    rows: list[dict[str, str]], candidate: Path, phase: int
) -> set[str]:
    with candidate.open(encoding="utf-8", newline="") as stream:
        current = {
            row["requirement"]: row for row in csv.DictReader(stream)
            if int(row["phase"]) < phase
        }
    imported = set()
    for row in rows:
        refreshed = current.get(row["requirement"])
        if refreshed is not None:
            row.update(refreshed)
            imported.add(row["requirement"])
    return imported


def reopen_downstream(row: dict[str, str]) -> None:
    row.update({
        "matched_test_count": "0", "command_result": "not-run",
        "source_revision": "not-bound", "source_digest": "not-bound",
        "source_state_digest": "not-bound", "run_nonce": "not-bound",
        "result_artifact_digest": "not-bound", "result": "OPEN",
        "final_source": "false",
    })


def settle_current(
    row: dict[str, str], prepared: dict[str, str], observation: dict[str, object]
) -> None:
    row.update(prepared)
    for field in (
        "matched_test_count", "source_revision", "source_digest",
        "source_state_digest", "run_nonce",
    ):
        row[field] = str(observation[field])
    row.update({
        "command_result": "passed",
        "result_artifact_digest": str(observation["artifact_sha256"]),
        "result": "PROVED", "final_source": "true",
    })

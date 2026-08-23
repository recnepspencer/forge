from __future__ import annotations

import contextlib
import csv
import hashlib
import io
import json
import os
import tempfile
from pathlib import Path

from worth_ui_3141_proof_plan import COMPILE_ARTIFACT
from worth_ui_ledger_retained_portfolio import portfolio_identity


@contextlib.contextmanager
def ledger_lock(identity: Path):
    identity.parent.mkdir(parents=True, exist_ok=True)
    with identity.open("a+b") as stream:
        if os.fstat(stream.fileno()).st_size == 0:
            stream.write(b"0")
            stream.flush()
        stream.seek(0)
        if os.name == "nt":
            import msvcrt

            msvcrt.locking(stream.fileno(), msvcrt.LK_LOCK, 1)
            try:
                yield
            finally:
                stream.seek(0)
                msvcrt.locking(stream.fileno(), msvcrt.LK_UNLCK, 1)
        else:
            import fcntl

            fcntl.flock(stream.fileno(), fcntl.LOCK_EX)
            try:
                yield
            finally:
                fcntl.flock(stream.fileno(), fcntl.LOCK_UN)


def write_requirements(
    ledger: Path,
    rows: list[dict[str, str]],
    fields: list[str],
    requirements: set[str],
) -> None:
    original = ledger.read_text(encoding="utf-8")
    rendered = render_requirement_update(original, rows, fields, requirements)
    with tempfile.NamedTemporaryFile(
        "w", encoding="utf-8", newline="", dir=ledger.parent, delete=False
    ) as stream:
        stream.write(rendered)
        temporary = Path(stream.name)
    temporary.replace(ledger)


def render_requirement_update(
    original: str,
    rows: list[dict[str, str]],
    fields: list[str],
    requirements: set[str],
) -> str:
    mutable = {row["requirement"]: row for row in rows if row["requirement"] in requirements}
    if set(mutable) != requirements:
        raise RuntimeError("ledger update names an unknown requirement")
    lines = original.splitlines(keepends=True)
    requirement_index = fields.index("requirement")
    for index, line in enumerate(lines[1:], 1):
        record = next(csv.reader([line]))
        requirement = record[requirement_index]
        if requirement in mutable:
            lines[index] = serialize_row(mutable[requirement], fields)
    return "".join(lines)


def serialize_row(row: dict[str, str], fields: list[str]) -> str:
    stream = io.StringIO(newline="")
    csv.DictWriter(stream, fieldnames=fields, lineterminator="\n").writerow(row)
    return stream.getvalue()


def csv_rows(content: str):
    return csv.DictReader(io.StringIO(content))


def synchronize_historical_rows(candidate: Path, root: Path, through_phase: int = 5) -> None:
    with candidate.open(encoding="utf-8", newline="") as stream:
        reader = csv.DictReader(stream)
        fields = list(reader.fieldnames or ())
        rows = list(reader)
    for row in rows:
        if int(row["phase"]) > through_phase or row["result"] != "PROVED":
            continue
        identity = root / row["retained_result_artifact"]
        content = identity.read_bytes()
        payload = json.loads(content.decode("utf-8"))
        if payload.get("requirement") != row["requirement"]:
            raise RuntimeError("historical artifact requirement does not match its ledger row")
        for field in ("source_revision", "source_digest", "source_state_digest", "run_nonce"):
            value = payload.get(field)
            if not isinstance(value, str):
                raise RuntimeError(f"historical artifact omits {field}")
            row[field] = value
        row["result_artifact_digest"] = hashlib.sha256(content).hexdigest()
    with candidate.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=fields, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)


def transaction_extra_identities(
    rows: list[dict[str, str]],
    selected: list[dict[str, str]],
    verify_phase: int | None,
) -> tuple[str, ...]:
    identities = set(() if verify_phase is None else (portfolio_identity(verify_phase),))
    if verify_phase is not None:
        identities.update(
            row["retained_result_artifact"]
            for row in rows
            if row.get("phase") == str(verify_phase)
        )
    predecessor_phases = [
        int(row["requirement"][1])
        for row in selected
        if row["requirement"].endswith("-PREDECESSOR-01")
    ]
    for phase in predecessor_phases:
        identities.add(COMPILE_ARTIFACT)
        identities.update(
            row["retained_result_artifact"]
            for row in rows
            if int(row["phase"]) < phase
        )
        for nested in range(3, phase + 1):
            identities.add(
                f"_docs/worth-ui/milestone-3.14.1-evidence/"
                f"p{nested}-predecessor-handoff.json"
            )
    return tuple(sorted(identities))

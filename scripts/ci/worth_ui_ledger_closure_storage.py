from __future__ import annotations

import contextlib
import csv
import io
import os
import tempfile
from pathlib import Path


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

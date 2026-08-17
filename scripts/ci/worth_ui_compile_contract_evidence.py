from __future__ import annotations

import hashlib
import secrets
from pathlib import Path

from worth_ui_ledger_command import source_revision
from worth_ui_ledger_source_state import source_state_digest


ROOT = Path(__file__).resolve().parents[2]


def compile_source_snapshot(
    cases: list[object], ignore_snapshot_updates: bool = False
) -> dict[str, object]:
    revision = source_revision()
    excluded_snapshots = {
        case.snapshot.relative_to(ROOT).as_posix()
        for case in cases
        if ignore_snapshot_updates and case.kind == "fail"
    }
    records = [
        case_record(case, ignore_snapshot_updates)
        for case in sorted(cases, key=lambda item: (item.owner, item.kind, item.target))
    ]
    return {
        "source_revision": revision,
        "source_state_digest": source_state_digest(revision, excluded_snapshots),
        "cases": records,
    }


def case_record(case: object, ignore_snapshot_updates: bool) -> dict[str, object]:
    failed = case.kind == "fail"
    return {
        "owner": case.owner,
        "kind": case.kind,
        "target": case.target,
        "source": case.source.relative_to(ROOT).as_posix(),
        "source_sha256": file_digest(case.source),
        "snapshot": case.snapshot.relative_to(ROOT).as_posix() if failed else None,
        "snapshot_sha256": (
            file_digest(case.snapshot) if failed and not ignore_snapshot_updates else None
        ),
    }


def result_artifact(
    cases: list[object], source_snapshot: dict[str, object]
) -> dict[str, object]:
    return {
        "schema": "worth-ui-compile-contract-result-v1",
        "exit_posture": "passed",
        "run_nonce": secrets.token_hex(16),
        "source_revision": source_snapshot["source_revision"],
        "source_state_digest": source_snapshot["source_state_digest"],
        "cargo_sessions": 2,
        "fail_targets": sum(case.kind == "fail" for case in cases),
        "pass_targets": sum(case.kind == "pass" for case in cases),
        "cases": source_snapshot["cases"],
    }


def file_digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()

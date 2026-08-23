from __future__ import annotations

import argparse
import hashlib
import json
import os
import shutil
import tempfile
from pathlib import Path
from typing import Any

from worth_ui_ledger_execution_binding import digest_json
from worth_ui_ledger_execution_observation_migration import (
    LEGACY_ROOT,
    validate_embedded_migration,
)
from worth_ui_ledger_retained_portfolio import portfolio_identity, validate


ARCHIVE_SCHEMA = "worth-ui-ledger-legacy-execution-archive-v1"
ARCHIVE_ROOT = Path(
    "workspaces/worth-ui/target/milestone-3141-legacy-execution-archives"
)


def audit(
    root: Path, ledger: Path, through_phase: int
) -> tuple[dict[str, Any], Path | None]:
    portfolio_path = root / portfolio_identity(through_phase)
    portfolio = json.loads(portfolio_path.read_text(encoding="utf-8"))
    validated = validate(
        root,
        ledger,
        through_phase,
        str(portfolio.get("source_revision", "")),
        str(portfolio.get("source_state_digest", "")),
    )
    active = active_legacy_keys(validated)
    state_digest = str(validated["source_state_digest"])
    for key in active:
        validate_embedded_migration(root, key, state_digest)
    legacy_root = root / LEGACY_ROOT
    entries = inventory(legacy_root, active)
    body = {
        "schema": ARCHIVE_SCHEMA,
        "through_phase": through_phase,
        "portfolio": portfolio_path.relative_to(root).as_posix(),
        "portfolio_sha256": hashlib.sha256(portfolio_path.read_bytes()).hexdigest(),
        "active_migrated_execution_count": len(active),
        "legacy_execution_count": len(entries),
        "legacy_execution_bytes": sum(entry["bytes"] for entry in entries),
        "unreachable_execution_count": sum(
            entry["classification"] == "unreachable" for entry in entries
        ),
        "entries": entries,
    }
    manifest = {**body, "archive_sha256": digest_json(body)}
    destination = None if not entries else root / ARCHIVE_ROOT / manifest["archive_sha256"]
    return manifest, destination


def archive(root: Path, ledger: Path, through_phase: int) -> dict[str, Any]:
    manifest, destination = audit(root, ledger, through_phase)
    if destination is None:
        return {**manifest, "archive_root": None, "posture": "legacy-root-empty"}
    legacy_root = root / LEGACY_ROOT
    destination.parent.mkdir(parents=True, exist_ok=True)
    if destination.exists():
        raise RuntimeError("legacy execution archive identity already exists")
    preparing = Path(tempfile.mkdtemp(prefix=".preparing-", dir=destination.parent))
    staged_root = preparing / "executions"
    moved = False
    try:
        write_json(preparing / "manifest.json", manifest)
        os.replace(legacy_root, staged_root)
        moved = True
        os.replace(preparing, destination)
    except BaseException:
        if moved and staged_root.exists() and not legacy_root.exists():
            legacy_root.parent.mkdir(parents=True, exist_ok=True)
            os.replace(staged_root, legacy_root)
        if preparing.exists():
            shutil.rmtree(preparing)
        raise
    return {
        **manifest,
        "archive_root": destination.relative_to(root).as_posix(),
        "posture": "archived",
    }


def active_legacy_keys(portfolio: dict[str, Any]) -> set[str]:
    migrations = portfolio.get("execution_observation_migrations")
    if not isinstance(migrations, list):
        raise RuntimeError("retained portfolio omits its migration inventory")
    keys = {
        item.get("legacy_execution_key")
        for item in migrations
        if isinstance(item, dict)
    }
    if None in keys or any(not valid_key(key) for key in keys):
        raise RuntimeError("retained portfolio has a malformed migration inventory")
    return {str(key) for key in keys}


def inventory(legacy_root: Path, active: set[str]) -> list[dict[str, Any]]:
    if not legacy_root.exists():
        return []
    entries = []
    for identity in sorted(legacy_root.rglob("*.json")):
        relative = identity.relative_to(legacy_root)
        key = identity.stem
        if (
            len(relative.parts) != 2
            or relative.parts[0] != key[:2]
            or not valid_key(key)
        ):
            raise RuntimeError("legacy execution identity is malformed")
        content = identity.read_bytes()
        entries.append({
            "identity": relative.as_posix(),
            "sha256": hashlib.sha256(content).hexdigest(),
            "bytes": len(content),
            "classification": "active-migrated" if key in active else "unreachable",
        })
    discovered = {entry["identity"].split("/")[-1][:-5] for entry in entries}
    missing = active - discovered
    if missing:
        raise RuntimeError("active legacy execution envelope is absent before archive")
    return entries


def valid_key(value: object) -> bool:
    return (
        isinstance(value, str)
        and len(value) == 64
        and all(character in "0123456789abcdef" for character in value)
    )


def write_json(identity: Path, payload: object) -> None:
    identity.write_text(
        json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", default=".")
    parser.add_argument("--ledger", default="_docs/worth-ui/milestone-3.14.1-proof-ledger.csv")
    parser.add_argument("--through-phase", type=int, required=True)
    parser.add_argument("--archive", action="store_true")
    arguments = parser.parse_args()
    root = Path(arguments.root).resolve()
    ledger = (root / arguments.ledger).resolve()
    result = (
        archive(root, ledger, arguments.through_phase)
        if arguments.archive
        else audit(root, ledger, arguments.through_phase)[0]
    )
    print(json.dumps(result, indent=2, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

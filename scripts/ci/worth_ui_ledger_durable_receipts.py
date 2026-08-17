from __future__ import annotations

import json
import os
import tempfile
from pathlib import Path
from typing import Any

from worth_ui_ledger_runner_authentication import authenticates


EVIDENCE_ROOT = Path("_docs/worth-ui/milestone-3.14.1-evidence")
CACHE_RELATIVE = Path("workspaces/worth-ui/target/milestone-3141-execution-cache")


def durable_receipt_identity(root: Path, key: str) -> Path:
    return root / EVIDENCE_ROOT / "executions" / key[:2] / f"{key}.json"


def cache_receipt_identity(root: Path, state_digest: str, key: str) -> Path:
    return root / CACHE_RELATIVE / state_digest / "executions" / key[:2] / f"{key}.json"


def persist_envelope(root: Path, key: str, envelope: dict[str, Any]) -> None:
    identity = durable_receipt_identity(root, key)
    identity.parent.mkdir(parents=True, exist_ok=True)
    descriptor, temporary = tempfile.mkstemp(prefix=".receipt-", dir=identity.parent)
    try:
        with os.fdopen(descriptor, "w", encoding="utf-8") as stream:
            json.dump(envelope, stream, sort_keys=True)
            stream.write("\n")
        os.replace(temporary, identity)
    finally:
        if os.path.exists(temporary):
            os.unlink(temporary)


def read_durable_envelope(root: Path, key: str) -> dict[str, Any]:
    identity = durable_receipt_identity(root, key)
    try:
        return json.loads(identity.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        raise RuntimeError("retained execution receipt is absent") from error


def persist_from_cache_if_absent(root: Path, state_digest: str, key: str) -> None:
    if durable_receipt_identity(root, key).is_file():
        return
    cache_identity = cache_receipt_identity(root, state_digest, key)
    try:
        envelope = json.loads(cache_identity.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        raise RuntimeError("retained execution receipt is absent") from error
    persist_envelope(root, key, envelope)


def harvest_referenced_receipts(
    root: Path,
    state_digest: str,
    receipts: list[dict[str, Any]],
) -> None:
    for receipt in receipts:
        key = receipt.get("key")
        if not isinstance(key, str):
            raise RuntimeError("retained execution receipt is malformed")
        persist_from_cache_if_absent(root, state_digest, key)


def authenticates_envelope(root: Path, envelope: dict[str, Any]) -> bool:
    record = envelope.get("record")
    return isinstance(record, dict) and authenticates(
        record, envelope.get("runner_authentication"), root
    )

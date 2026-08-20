from __future__ import annotations

import csv
import hashlib
import json
import os
from pathlib import Path
from typing import Any

from worth_ui_ledger_command import CLAIM_FIELDS
from worth_ui_ledger_runner_authentication import authenticates


DEFAULT_LEDGER = Path("_docs/worth-ui/milestone-3.14.1-proof-ledger.csv")
REBINDABLE_SOURCE_IDENTITIES = {
    "_docs/worth-ui/milestone-3.14.1-evidence/compile-contracts.json",
    "_docs/worth-ui/milestone-3.14.1-evidence/p1-worlds-01.json",
    "_docs/worth-ui/milestone-3.14.1-evidence/p2-world-01.json",
    "_docs/worth-ui/milestone-3.14.1-evidence/p3-delta-source-01.json",
    "_docs/worth-ui/milestone-3.14.1-evidence/p3-hp02-world-01.json",
    "_docs/worth-ui/milestone-3.14.1-evidence/p3-predecessor-handoff.json",
    "_docs/worth-ui/milestone-3.14.1-evidence/p4-predecessor-handoff.json",
    "_docs/worth-ui/milestone-3.14.1-evidence/p5-predecessor-handoff.json",
    "_docs/worth-ui/milestone-3.14.1-evidence/p5-atlas-01.json",
}


def require_proved_artifact(
    root: Path, requirement: str, identity: str, artifact: dict[str, Any]
) -> str:
    ledger = Path(os.environ.get("WORTH_UI_MILESTONE_3141_LEDGER", DEFAULT_LEDGER.as_posix()))
    if not ledger.is_absolute():
        ledger = root / ledger
    with ledger.open(encoding="utf-8", newline="") as stream:
        rows = list(csv.DictReader(stream))
    matches = [row for row in rows if row["requirement"] == requirement]
    if len(matches) != 1:
        raise ValueError(f"dependency ledger omits exact producer {requirement}")
    row = matches[0]
    if row["result"] != "PROVED" or row["final_source"] != "true":
        raise ValueError(f"dependency producer {requirement} is not final-source proved")
    if row["retained_result_artifact"] != identity:
        raise ValueError(f"dependency producer {requirement} names another artifact")
    raw = (root / identity).read_bytes()
    digest = hashlib.sha256(raw).hexdigest()
    if row["result_artifact_digest"] != digest:
        raise ValueError(f"dependency producer {requirement} artifact digest drifted")
    unsigned = {
        key: value for key, value in artifact.items() if key != "runner_authentication"
    }
    if not authenticates(unsigned, artifact.get("runner_authentication"), root):
        raise ValueError(f"dependency producer {requirement} lacks runner provenance")
    claim = hashlib.sha256()
    for field in CLAIM_FIELDS:
        claim.update(field.encode("utf-8"))
        claim.update(b"\0")
        claim.update(row[field].encode("utf-8"))
        claim.update(b"\0")
    if artifact.get("claim_digest") != claim.hexdigest():
        raise ValueError(f"dependency producer {requirement} drifted claim_digest")
    for column, field in [
        ("source_revision", "source_revision"),
        ("source_digest", "source_digest"),
        ("source_state_digest", "source_state_digest"),
        ("run_nonce", "run_nonce"),
    ]:
        if artifact.get(field) != row[column]:
            raise ValueError(f"dependency producer {requirement} drifted {field}")
    mapped_sources = artifact.get("mapping_source_identity", artifact.get("source_identity"))
    executed_sources = artifact.get("source_identity")
    if not isinstance(executed_sources, list) or len(executed_sources) != len(mapped_sources):
        raise ValueError(f"dependency producer {requirement} has invalid executed sources")
    if executed_sources != row["source_identity"].split(";"):
        raise ValueError(f"dependency producer {requirement} drifted sources")
    validate_source_rebindings(root, requirement, mapped_sources, executed_sources, artifact)
    return digest


def validate_source_rebindings(
    root: Path,
    requirement: str,
    mapped_sources: list[str],
    executed_sources: list[str],
    artifact: dict[str, Any],
) -> None:
    records = artifact.get("source_rebindings", [])
    if not isinstance(records, list):
        raise ValueError(f"dependency producer {requirement} has invalid source rebindings")
    expected = []
    target = (root / "workspaces/worth-ui/target").resolve()
    for canonical, executed in zip(mapped_sources, executed_sources, strict=True):
        if canonical == executed:
            continue
        if canonical not in REBINDABLE_SOURCE_IDENTITIES:
            raise ValueError(f"dependency producer {requirement} substituted production source")
        identity = (root / executed).resolve()
        try:
            relative = identity.relative_to(target)
        except ValueError as error:
            raise ValueError(
                f"dependency producer {requirement} source escaped governed target"
            ) from error
        if not relative.parts or not relative.parts[0].startswith("worth-ui-3141-verify-"):
            raise ValueError(
                f"dependency producer {requirement} source is not verifier-owned"
            )
        expected.append({
            "canonical": canonical,
            "executed": executed,
            "sha256": hashlib.sha256(identity.read_bytes()).hexdigest(),
        })
    if records != expected:
        raise ValueError(f"dependency producer {requirement} drifted source rebindings")

"""Hash-bound classification overlay loading and inventory validation."""

from __future__ import annotations

import csv
import hashlib
import re
from pathlib import Path


OVERRIDE_FIELDS = {
    "target_crate", "proposed_action", "candidate_action", "target_subsystem",
    "target_directory", "target_path", "mechanical_ready", "confidence",
    "content_review_required", "review_batch", "reviewed_content_sha256", "rationale",
}
MILESTONE_TARGET = re.compile(r"(^|[/_.-])(s\d+(?:_\d+)?|phase\d+|milestone\d+)([/_.-]|$)")


def load_overrides(directory: Path) -> dict[str, dict[str, str]]:
    overrides: dict[str, dict[str, str]] = {}
    if not directory.exists():
        return overrides
    for batch_path in sorted(directory.glob("*.csv")):
        with batch_path.open(newline="", encoding="utf-8") as source:
            for row in csv.DictReader(source):
                current_path = row.get("current_path", "")
                if not current_path:
                    raise ValueError(f"missing current_path in {batch_path}")
                if current_path in overrides:
                    raise ValueError(f"duplicate classification for {current_path}")
                overrides[current_path] = row
    return overrides


def apply_override(row: dict[str, str | int], path: Path, override: dict[str, str]) -> None:
    expected_hash = override.get("reviewed_content_sha256", "")
    actual_hash = hashlib.sha256(path.read_bytes()).hexdigest()
    if not expected_hash or expected_hash != actual_hash:
        raise ValueError(f"stale or unstamped classification for {row['current_path']}")
    for field in OVERRIDE_FIELDS:
        if field in override:
            row[field] = override[field]


def validate_rows(rows: list[dict[str, str | int]]) -> None:
    current_paths = [str(row["current_path"]) for row in rows]
    if len(current_paths) != len(set(current_paths)):
        raise ValueError("inventory contains duplicate current paths")
    classified = [row for row in rows if row["review_batch"]]
    milestone_targets = [str(row["target_path"]) for row in classified if row["target_path"] and MILESTONE_TARGET.search(str(row["target_path"]))]
    if milestone_targets:
        raise ValueError(f"classified targets retain milestone vocabulary: {milestone_targets[:10]}")
    targets = [str(row["target_path"]) for row in classified if row["target_path"] and row["proposed_action"] in {"move", "keep"}]
    if len(targets) != len(set(targets)):
        raise ValueError("classified move/keep targets collide")

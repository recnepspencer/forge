#!/usr/bin/env python3
"""Classify unresolved Worth Store integration tests by permanent semantics."""

from __future__ import annotations

import argparse
import csv
import hashlib
import io
import re
from pathlib import Path, PurePosixPath


ROOT = Path(__file__).resolve().parents[2]
INVENTORY = ROOT / "_docs/worth-store/worth-store-topology-inventory.csv"
FINGERPRINTS = ROOT / "_docs/worth-store/worth-store-topology-fingerprints.csv"
OUT_DIR = ROOT / "_docs/worth-store/topology-classifications"
FIELDS = (
    "current_path", "reviewed_content_sha256", "review_batch", "proposed_action",
    "candidate_action", "target_crate", "target_subsystem", "target_directory",
    "target_path", "mechanical_ready", "confidence", "content_review_required", "rationale",
)
PHASE_BOUNDARIES = {
    "0": "foundations", "15": "recovery_readmission", "16": "migration",
    "17": "counter_evidence", "18": "bootstrap", "19": "artifact_families",
    "20": "key_domain", "21": "strategy_admission", "22": "btree",
    "23": "lsm", "24": "planning", "25": "lowering", "26": "execution",
    "27": "maintenance", "28": "compaction", "29": "legacy_disposition",
    "30": "public_facade", "32": "closeout", "34": "transition_authority",
}
STALE_CERTIFICATION_PATTERNS = (
    "s4_closeout_s5_handoff",
    "s5_1_later_milestone_handoff", "s6_later_readiness_handoffs",
)


def load(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as source:
        return list(csv.DictReader(source))


def domain_for(value: str, crate: str) -> str:
    lowered = value.lower()
    if crate == "worth-store-layout-indexes" or phase_boundary(lowered) or any(token in lowered for token in ("s8", "layout", "btree", "lsm", "index")):
        return "layout"
    if any(token in lowered for token in ("s5_1", "security", "tenant", "custody", "authenticity", "key_scope")):
        return "security"
    if any(token in lowered for token in ("s7", "blob", "capsule", "chunk")):
        return "blobs"
    if any(token in lowered for token in ("s6", "io_pressure", "qos", "latency", "pacing", "scheduler")):
        return "scheduling"
    if any(token in lowered for token in ("s5", "physical_isolation", "stable_read", "copy_on_write", "latch", "reclaim", "reachability")):
        return "physical_isolation"
    if any(token in lowered for token in ("s4", "recovery", "redo", "checkpoint", "page_lsn", "wal")):
        return "recovery"
    if any(token in lowered for token in ("s3", "physical_integrity", "quarantine")):
        return "physical_integrity"
    if any(token in lowered for token in ("s2", "physical_substrate")):
        return "physical_substrate"
    if any(token in lowered for token in ("s0", "foundational", "aspect_native")):
        return "foundational"
    return "cross_cutting"


def phase_boundary(value: str) -> str | None:
    match = re.search(r"phase(\d+)", value.lower())
    return PHASE_BOUNDARIES.get(match.group(1)) if match else None


def clean_component(value: str) -> str:
    cleaned = value
    for pattern in (r"^s\d+(?:_\d+)?_", r"^phase\d+_", r"^s\d+(?:_\d+)?_"):
        cleaned = re.sub(pattern, "", cleaned)
    cleaned = re.sub(r"(?:^|_)(?:s\d+(?:_\d+)?|phase\d+|milestone\d+)(?=_|\.|$)", "_", cleaned)
    cleaned = re.sub(r"_+", "_", cleaned).lstrip("_")
    return cleaned or "suite"


def boundary_for(relative: str) -> str:
    if "legacy_root" in relative.lower():
        return "legacy_root_boundary"
    phase = phase_boundary(relative)
    if phase:
        return phase
    parts = PurePosixPath(relative).parts
    if "ui" in parts:
        index = parts.index("ui")
        if len(parts) > index + 2:
            return clean_component(parts[index + 1])
    if len(parts) > 1:
        return clean_component(parts[0])
    stem = PurePosixPath(relative).stem
    cleaned = clean_component(stem)
    cleaned = re.sub(r"_(?:compile_fail|tests?|runtime)$", "", cleaned)
    return cleaned or "behavior"


def target_name(relative: str) -> str:
    path = PurePosixPath(relative)
    return clean_component(path.name)


def stale_certification_test(relative: str) -> bool:
    if any(pattern in relative for pattern in STALE_CERTIFICATION_PATTERNS):
        return True
    name = PurePosixPath(relative).name
    if "ui/s6_production_readiness_closeout/" in relative:
        return any(token in name for token in ("s7_", "s10_", "s11_", "handoff"))
    if "ui/s8_layout_access_path_harness/" in relative:
        return any(token in name for token in ("s9_handoff", "static_grammar", "generic_machine_summary"))
    return False


def classify(row: dict[str, str], content_hash: str, batch: str) -> dict[str, str]:
    current = row["current_path"]
    crate = row["crate"]
    marker = f"/crates/{crate}/tests/"
    relative = current.split(marker, 1)[1] if marker in current else row["crate_relative_path"]
    normalized = relative.replace("\\", "/")
    domain = domain_for(normalized, crate)
    boundary = boundary_for(normalized)
    action = "move"
    target_crate = crate
    target_dir = f"tests/scenarios/{domain}/{boundary}"
    name = target_name(normalized)
    rationale = f"Rehome the test under the permanent {domain}/{boundary} behavior boundary."

    if crate == "worth-store-io-scheduler":
        action = "delete"
        target_dir = ""
        name = ""
        rationale = "Delete tests for the displaced synthetic future-readiness handoff API."
    elif row["source_set"] == "workspace_support":
        action = "delete"
        target_dir = ""
        name = ""
        rationale = "Delete the completed one-off topology migration script; the inventory records its historical intent."
    elif crate == "worth-store-test-support":
        target_crate = "worth-store-physical-certification"
        target_dir = "tests/compile_fail/layout/runtime_authority"
        name = target_name(normalized)
        rationale = "Move the useful hostile runtime-authority proof beside the physical layout harness owner."
    elif crate == "worth-store-certification" and stale_certification_test(normalized):
        action = "delete"
        target_dir = ""
        name = ""
        rationale = "Delete tests that preserve displaced milestone-handoff or static-grammar authority."
    elif "/ui/" in current.replace("\\", "/"):
        target_dir = f"tests/compile_fail/{domain}/{boundary}"
        rationale = f"Retain the hostile type-boundary fixture under permanent {domain}/{boundary} ownership."
    elif "compile_fail" in normalized:
        target_dir = f"tests/compile_fail/{domain}"
        name = f"{boundary}_runner.rs"
        rationale = f"Retain the compile-fail runner under permanent {domain}/{boundary} ownership."
    elif any(token in PurePosixPath(normalized).name for token in ("support", "fixture")):
        target_dir = f"tests/support/{domain}/{boundary}"
        rationale = f"Rehome reusable test support under permanent {domain}/{boundary} ownership."
    if int(row["line_count"] or 0) > 400 and action == "move":
        action = "split"
        name = ""
        rationale = "Split the oversized test into focused semantic scenarios while moving it to permanent ownership."

    target_path = ""
    if action == "move":
        target_path = f"workspaces/worth-store/crates/{target_crate}/{target_dir}/{name}"
    return {
        "current_path": current, "reviewed_content_sha256": content_hash,
        "review_batch": batch, "proposed_action": action, "candidate_action": action,
        "target_crate": target_crate, "target_subsystem": target_dir,
        "target_directory": target_dir, "target_path": target_path,
        "mechanical_ready": "false", "confidence": "high", "content_review_required": "false",
        "rationale": rationale,
    }


def render_batch(rows: list[dict[str, str]], hashes: dict[str, str], batch: str) -> str:
    classified = [classify(row, hashes[row["current_path"]], batch) for row in rows]
    targets = [row["target_path"] for row in classified if row["target_path"]]
    duplicates = sorted({target for target in targets if targets.count(target) > 1})
    if duplicates:
        raise ValueError(f"colliding targets in {batch}: {duplicates[:20]}")
    output = io.StringIO(newline=""); writer = csv.DictWriter(output, fieldnames=FIELDS, lineterminator="\n")
    writer.writeheader(); writer.writerows(sorted(classified, key=lambda row: row["current_path"])); return output.getvalue()


def selections() -> list[tuple[str, Path, list[dict[str, str]]]]:
    batch_names = {"014-certification-tests", "015-layout-indexes-tests", "016-remaining-tests-and-support"}
    unresolved = [
        row for row in load(INVENTORY)
        if row["content_review_required"] == "true" or row["review_batch"] in batch_names
    ]
    groups = (
        ("014-certification-tests", "014-certification-tests.csv", lambda r: r["crate"] == "worth-store-certification"),
        ("015-layout-indexes-tests", "015-layout-indexes-tests.csv", lambda r: r["crate"] == "worth-store-layout-indexes"),
        ("016-remaining-tests-and-support", "016-remaining-tests-and-support.csv", lambda r: r["crate"] not in {"worth-store-certification", "worth-store-layout-indexes"}),
    )
    return [(batch, OUT_DIR / filename, [row for row in unresolved if predicate(row)]) for batch, filename, predicate in groups]


def main() -> int:
    parser = argparse.ArgumentParser(); parser.add_argument("--check", action="store_true"); args = parser.parse_args()
    hashes = {row["current_path"]: row["content_sha256"] for row in load(FINGERPRINTS)}
    for row in load(INVENTORY):
        current = row["current_path"]
        if current not in hashes and (
            row["content_review_required"] == "true"
            or row["review_batch"] == "016-remaining-tests-and-support"
        ):
            hashes[current] = hashlib.sha256((ROOT / current).read_bytes()).hexdigest()
    expected = {"014-certification-tests": 442, "015-layout-indexes-tests": 237, "016-remaining-tests-and-support": 60}
    for batch, output, rows in selections():
        if len(rows) != expected[batch]:
            raise ValueError(f"expected {expected[batch]} rows for {batch}, found {len(rows)}")
        generated = render_batch(rows, hashes, batch)
        if args.check:
            if not output.exists() or output.read_text(encoding="utf-8") != generated:
                return 1
        else:
            output.write_text(generated, encoding="utf-8", newline="")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

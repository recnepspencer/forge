#!/usr/bin/env python3
"""Generate the reviewed Forge Store test-support production topology batch."""

from __future__ import annotations

import argparse
import csv
import io
from pathlib import Path, PurePosixPath


ROOT = Path(__file__).resolve().parents[2]
INVENTORY = ROOT / "_docs/forge-store/forge-store-topology-inventory.csv"
FINGERPRINTS = ROOT / "_docs/forge-store/forge-store-topology-fingerprints.csv"
OUTPUT = ROOT / "_docs/forge-store/topology-classifications/013-test-support-production.csv"
BATCH = "013-test-support-production"
CRATE = "forge-store-test-support"
PREFIX = f"workspaces/forge-store/crates/{CRATE}/src/"
FIELDS = (
    "current_path", "reviewed_content_sha256", "review_batch", "proposed_action",
    "candidate_action", "target_crate", "target_subsystem", "target_directory",
    "target_path", "mechanical_ready", "confidence", "content_review_required", "rationale",
)


def load(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as source:
        return list(csv.DictReader(source))


def target(relative: str) -> tuple[str, str, str, str, str]:
    path = PurePosixPath(relative)
    name = path.name
    recovery_prefix = "harness/milestone/s4_recovery_physics/"
    if relative.startswith(recovery_prefix):
        clean = name.removeprefix("s4_")
        if name == "mod.rs":
            return "split", CRATE, "harness", "", "Split the milestone barrel across permanent recovery harness roles."
        role = "fixtures"
        if "fault_scheduler" in name:
            role = "faults"
        elif any(token in name for token in ("crash_harness", "fresh_runtime", "storage_interposer")):
            role = "drivers"
        return "move", CRATE, f"{role}/recovery", clean, "Rehome recovery support by its reusable fixture, fault, or driver role."

    security_prefix = "harness/milestone/s5_1_security_scope_harness/"
    if relative.startswith(security_prefix):
        if name == "mod.rs":
            return "split", CRATE, "harness", "", "Split security inputs from physical execution machinery."
        if name == "fixtures.rs":
            return "move", CRATE, "fixtures/security", "scope.rs", "Keep raw native security fixtures in test support."
        if name == "scenarios.rs":
            return "move", CRATE, "inputs/security", "scope_scenarios.rs", "Keep declarative hostile scenario inputs in test support."
        return "move", "forge-store-physical-certification", "drivers/security", "scope_execution.rs", "Move executable security simulation from test support to the physical harness."

    if relative == "harness/milestone/s5_physical_isolation/mod.rs":
        return "split", CRATE, "harness", "", "Split the boundary fixture and yield schedule into their permanent support roles."
    if relative.endswith("s6_interference_profiles.rs"):
        return "move", CRATE, "inputs/scheduling", "interference_profiles.rs", "Keep reusable scheduling profile declarations as test inputs."
    if relative.endswith("s6_io_pressure_profiles.rs"):
        return "move", CRATE, "inputs/scheduling", "io_pressure_profiles.rs", "Keep reusable scheduling profile declarations as test inputs."
    if relative.endswith("s7_blob_harness_execution.rs"):
        return "delete", CRATE, "", "", "Delete the redundant forwarding execution facade; physical certification owns execution."
    if relative.endswith("s7_blob_harness_heavy_fixture.rs"):
        return "move", CRATE, "scale/blobs", "multi_gb_fixture.rs", "Keep opt-in scale fixture selection in test support."
    if relative.endswith("s7_blob_harness_profiles.rs"):
        return "move", CRATE, "inputs/blobs", "profiles.rs", "Keep reusable blob scenario profiles as test inputs."

    layout_prefix = "harness/milestone/s8_layout_access/"
    if relative.startswith(layout_prefix):
        if name == "executed_runtime.rs":
            return "move", "forge-store-physical-certification", "drivers/layout", "access_runtime.rs", "Move production-facade execution into the physical certification harness."
        return "delete", CRATE, "", "", "Delete the milestone wrapper around canonical physical-certification scenario catalogs."
    raise ValueError(f"unclassified test-support path: {relative}")


def render() -> str:
    rows = [r for r in load(INVENTORY) if r["crate"] == CRATE and r["source_set"] == "production" and (r["content_review_required"] == "true" or r["review_batch"] == BATCH)]
    hashes = {r["current_path"]: r["content_sha256"] for r in load(FINGERPRINTS)}
    if len(rows) != 27:
        raise ValueError(f"expected 27 test-support rows, found {len(rows)}")
    classified = []
    for row in rows:
        current = row["current_path"]
        action, crate, directory, name, rationale = target(current.removeprefix(PREFIX))
        target_path = f"workspaces/forge-store/crates/{crate}/src/{directory}/{name}" if action == "move" else ""
        classified.append({
            "current_path": current, "reviewed_content_sha256": hashes[current], "review_batch": BATCH,
            "proposed_action": action, "candidate_action": action, "target_crate": crate,
            "target_subsystem": directory, "target_directory": directory, "target_path": target_path,
            "mechanical_ready": "false", "confidence": "high", "content_review_required": "false",
            "rationale": rationale,
        })
    targets = [r["target_path"] for r in classified if r["target_path"]]
    duplicates = sorted({item for item in targets if targets.count(item) > 1})
    if duplicates:
        raise ValueError(f"colliding test-support targets: {duplicates}")
    output = io.StringIO(newline=""); writer = csv.DictWriter(output, fieldnames=FIELDS, lineterminator="\n")
    writer.writeheader(); writer.writerows(sorted(classified, key=lambda r: r["current_path"])); return output.getvalue()


def main() -> int:
    parser = argparse.ArgumentParser(); parser.add_argument("--check", action="store_true"); args = parser.parse_args()
    generated = render()
    if args.check:
        return 0 if OUTPUT.exists() and OUTPUT.read_text(encoding="utf-8") == generated else 1
    OUTPUT.write_text(generated, encoding="utf-8", newline=""); return 0


if __name__ == "__main__":
    raise SystemExit(main())

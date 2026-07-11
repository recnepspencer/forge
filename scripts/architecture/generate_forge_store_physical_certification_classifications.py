#!/usr/bin/env python3
"""Generate the reviewed physical-certification production topology batch."""

from __future__ import annotations

import argparse
import csv
import io
import re
from pathlib import Path, PurePosixPath


ROOT = Path(__file__).resolve().parents[2]
INVENTORY = ROOT / "_docs/forge-store/forge-store-topology-inventory.csv"
FINGERPRINTS = ROOT / "_docs/forge-store/forge-store-topology-fingerprints.csv"
OUTPUT = ROOT / "_docs/forge-store/topology-classifications/012-physical-certification-production.csv"
BATCH = "012-physical-certification-production"
CRATE = "forge-store-physical-certification"
PREFIX = f"workspaces/forge-store/crates/{CRATE}/src/"
FIELDS = (
    "current_path", "reviewed_content_sha256", "review_batch", "proposed_action",
    "candidate_action", "target_crate", "target_subsystem", "target_directory",
    "target_path", "mechanical_ready", "confidence", "content_review_required", "rationale",
)


def load_csv(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as source:
        return list(csv.DictReader(source))


def clean(name: str) -> str:
    return re.sub(r"^(?:s\d+(?:_\d+)?_|s45_)", "", name)


def row_target(relative: str) -> tuple[str, str, str, str, str]:
    """Return action, crate, directory, filename, rationale."""
    path = PurePosixPath(relative)
    name = clean(path.name)
    lowered = relative.lower()
    rationale = "Rehome reusable physical certification machinery by permanent harness role and domain."

    if relative == "lib.rs":
        return "keep", CRATE, "src", "lib.rs", "Keep the crate facade; later edits should expose permanent harness roles only."
    if relative == "coverage/registration.rs":
        return "split", CRATE, "coverage", "", "Split the oversized cross-domain coverage registry into domain registrations."
    if lowered.startswith("harness/by_milestone/s8_layout_access/"):
        return "delete", CRATE, "", "", "Delete the milestone compatibility re-export; layout_harness already owns the real capability."

    if lowered.startswith("harness/by_milestone/s6/s6_backend_qualification/"):
        return "move", CRATE, "qualification/scheduling/backends", name, rationale
    if lowered.startswith("harness/by_milestone/s6/tests/"):
        return "move", CRATE, "tests/scheduling", name, rationale
    if lowered.startswith("harness/by_milestone/s6/s6_io_pressure_test_support/"):
        return "move", CRATE, "fixtures/scheduling/io_pressure", name, rationale
    if lowered.startswith("harness/by_milestone/s6/"):
        role = "scenarios"
        if "coverage" in lowered:
            role = "coverage"
        elif "execution" in lowered:
            role = "drivers"
        elif "replay" in lowered:
            role = "replay"
        return "move", CRATE, f"{role}/scheduling", name, rationale

    if lowered.startswith("harness/by_milestone/s7_blob_harness/"):
        role = "scenarios"
        if any(token in lowered for token in ("fixture", "profile")):
            role = "fixtures"
        elif "qualification" in lowered:
            role = "qualification"
        elif "oracle" in lowered:
            role = "observations"
        elif "replay" in lowered:
            role = "replay"
        elif "lowering" in lowered:
            role = "drivers"
        elif "shortcut" in lowered:
            role = "faults"
        elif path.name == "tests.rs":
            role = "tests"
        elif path.name == "mod.rs":
            return "split", CRATE, "scenarios/blobs", "", "Split the milestone barrel across permanent blob harness roles."
        return "move", CRATE, f"{role}/blobs", name, rationale

    if lowered.startswith("oracles/s5_"):
        return "move", CRATE, "oracles/physical_isolation", clean(path.name), rationale
    if lowered.startswith("planning/requirements/s5_"):
        return "move", CRATE, "planning/physical_isolation", clean(path.name), rationale
    if lowered.startswith("planning/requirements/s6_"):
        return "move", CRATE, "planning/scheduling", clean(path.name), rationale
    if lowered.startswith("planning/requirements/s7_"):
        return "move", CRATE, "planning/blobs", clean(path.name), rationale
    if lowered.startswith("s45_entry/"):
        return "move", CRATE, "admission/simulation", clean(path.name), "Rename the roadmap entry vocabulary as reusable simulation admission."
    if lowered.startswith("s5_1_security_scope_harness/"):
        role = "scenarios"
        if "counter" in lowered or "evidence" in lowered:
            role = "observations"
        elif "oracle" in lowered:
            role = "oracles"
        elif "replay" in lowered:
            role = "replay"
        elif "schedule" in lowered:
            role = "schedules"
        return "move", CRATE, f"{role}/security", name, rationale
    if lowered.startswith("s5_handoff/"):
        return "move", CRATE, "qualification/physical_isolation/readiness", name, "Preserve the harness qualification capability without milestone handoff vocabulary."
    if lowered.startswith("s7_closeout/"):
        if name in {"denial.rs", "mod.rs"}:
            return "merge", "forge-store-certification", "courtroom/blobs/closeout", name, "Merge duplicate closeout vocabulary into the courtroom-owned blob closeout family."
        return "move", "forge-store-certification", "courtroom/blobs/closeout", name, "Move closeout judgment from the reusable harness into the certification courtroom."
    if relative == "s5_executed_isolation_contract.rs":
        return "move", CRATE, "observations/physical_isolation", "executed_isolation_contract.rs", rationale
    if relative == "s5_executed_isolation_source.rs":
        return "move", CRATE, "observations/physical_isolation", "executed_isolation_source.rs", rationale
    if relative == "s5_physical_isolation_mutation.rs":
        return "move", CRATE, "drivers/physical_isolation", "mutation.rs", rationale
    raise ValueError(f"unclassified physical certification path: {relative}")


def render() -> str:
    rows = [r for r in load_csv(INVENTORY) if r["crate"] == CRATE and r["source_set"] == "production" and (r["content_review_required"] == "true" or r["review_batch"] == BATCH)]
    hashes = {r["current_path"]: r["content_sha256"] for r in load_csv(FINGERPRINTS)}
    if len(rows) != 90:
        raise ValueError(f"expected 90 physical certification rows, found {len(rows)}")
    output_rows = []
    for row in rows:
        current = row["current_path"]
        action, crate, directory, name, rationale = row_target(current.removeprefix(PREFIX))
        target = ""
        if action in {"move", "keep", "merge"}:
            target = f"workspaces/forge-store/crates/{crate}/src/{directory}/{name}"
            target = target.replace("/src/src/", "/src/")
        output_rows.append({
            "current_path": current, "reviewed_content_sha256": hashes[current],
            "review_batch": BATCH, "proposed_action": action, "candidate_action": action,
            "target_crate": crate, "target_subsystem": directory,
            "target_directory": directory, "target_path": target,
            "mechanical_ready": "false", "confidence": "high",
            "content_review_required": "false", "rationale": rationale,
        })
    targets = [r["target_path"] for r in output_rows if r["target_path"]]
    duplicates = sorted({target for target in targets if targets.count(target) > 1})
    if duplicates:
        raise ValueError(f"colliding physical certification targets: {duplicates}")
    output = io.StringIO(newline="")
    writer = csv.DictWriter(output, fieldnames=FIELDS, lineterminator="\n")
    writer.writeheader(); writer.writerows(sorted(output_rows, key=lambda r: r["current_path"]))
    return output.getvalue()


def main() -> int:
    parser = argparse.ArgumentParser(); parser.add_argument("--check", action="store_true"); args = parser.parse_args()
    generated = render()
    if args.check:
        return 0 if OUTPUT.exists() and OUTPUT.read_text(encoding="utf-8") == generated else 1
    OUTPUT.write_text(generated, encoding="utf-8", newline=""); return 0


if __name__ == "__main__":
    raise SystemExit(main())

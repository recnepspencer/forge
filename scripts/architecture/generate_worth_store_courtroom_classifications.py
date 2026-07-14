#!/usr/bin/env python3
"""Generate the reviewed Worth Store certification production batch."""

from __future__ import annotations

import argparse
import csv
import io
import re
from pathlib import Path, PurePosixPath


ROOT = Path(__file__).resolve().parents[2]
INVENTORY = ROOT / "_docs/worth-store/worth-store-topology-inventory.csv"
FINGERPRINTS = ROOT / "_docs/worth-store/worth-store-topology-fingerprints.csv"
OUTPUT = ROOT / "_docs/worth-store/topology-classifications/011-certification-production.csv"
BATCH = "011-certification-production"
PREFIX = "workspaces/worth-store/crates/worth-store-certification/src/"

FIELDS = (
    "current_path",
    "reviewed_content_sha256",
    "review_batch",
    "proposed_action",
    "candidate_action",
    "target_crate",
    "target_subsystem",
    "target_directory",
    "target_path",
    "mechanical_ready",
    "confidence",
    "content_review_required",
    "rationale",
)

DELETE_PATHS = {
    "courtroom/closeout/s8_layout.rs",
    "s6_later_readiness_handoffs.rs",
    "s6_phase.rs",
    "s7_closeout/handoffs.rs",
    "s8_layout_closeout/handoffs.rs",
}


def load_rows() -> list[dict[str, str]]:
    with INVENTORY.open(newline="", encoding="utf-8") as source:
        return [
            row
            for row in csv.DictReader(source)
            if row["crate"] == "worth-store-certification"
            and row["source_set"] == "production"
            and (row["content_review_required"] == "true" or row["review_batch"] == BATCH)
        ]


def load_hashes() -> dict[str, str]:
    with FINGERPRINTS.open(newline="", encoding="utf-8") as source:
        return {row["current_path"]: row["content_sha256"] for row in csv.DictReader(source)}


def domain_for(relative: str) -> str:
    lowered = relative.lower()
    components = set(PurePosixPath(lowered).parts)
    if lowered.startswith("s0_") or "foundational" in lowered or "aspect_native" in lowered:
        return "foundational"
    if lowered.startswith("s2_") or "physical_substrate" in lowered or "pre_decode" in lowered:
        return "physical_substrate"
    if lowered.startswith("s3_") or "s3" in components or "physical_integrity" in lowered or "integrity" in lowered or "quarantine" in lowered:
        return "physical_integrity"
    if lowered.startswith("s4_") or "recovery" in lowered:
        return "recovery"
    if lowered.startswith("s5_1") or "authenticity" in lowered or "security_scope" in lowered:
        return "security"
    if lowered.startswith("s5_") or "physical_isolation" in lowered:
        return "physical_isolation"
    if lowered.startswith("s6") or "s6" in components or any(token in lowered for token in ("pacing", "queue", "latency", "qos")):
        return "scheduling"
    if lowered.startswith("s7") or any(token in lowered for token in ("blob", "capsule")):
        return "blobs"
    if lowered.startswith("s8") or "layout" in lowered:
        return "layout"
    return "cross_cutting"


def clean_name(name: str) -> str:
    cleaned = re.sub(r"^(?:s\d+(?:_\d+)?_|phase\d+_)", "", name)
    return cleaned.replace("roadmap2_", "")


def harness_target(relative: str, domain: str) -> tuple[str, str, str] | None:
    path = PurePosixPath(relative)
    parts = path.parts
    lowered = relative.lower()
    if lowered.startswith("courtroom/harness/test_support/"):
        target_dir = f"fixtures/{domain}"
        return "worth-store-physical-certification", target_dir, clean_name(path.name)
    if lowered.startswith("s4_recovery_harness/certification/"):
        target_dir = "courtroom/recovery/harness"
        return "worth-store-certification", target_dir, clean_name(path.name)
    if lowered.startswith("s4_recovery_harness/evidence/"):
        target_dir = "observation/recovery"
        return "worth-store-physical-certification", target_dir, clean_name(path.name)
    if lowered.startswith("s4_recovery_harness/"):
        role = parts[1] if len(parts) > 2 else ""
        target_dir = "faults/recovery" if role == "mutation" else "scenarios/recovery"
        return "worth-store-physical-certification", target_dir, clean_name(path.name)
    if lowered == "s4_recovery_harness_exports.rs":
        return "worth-store-physical-certification", "scenarios/recovery", "facade.rs"
    if lowered.startswith("s5_physical_isolation_harness/"):
        suffix = PurePosixPath(*parts[1:-1]).as_posix()
        target_dir = "scenarios/physical_isolation" + (f"/{suffix}" if suffix and suffix != "." else "")
        return "worth-store-physical-certification", target_dir, clean_name(path.name)
    return None


def category_for(relative: str, domain: str, role: str) -> str:
    lowered = relative.lower()
    if "evidence_materialization" in lowered or role in {"evidence_projection", "receipt_projection", "measurement"}:
        return f"evidence/{domain}"
    if "replay" in lowered:
        return f"replay/{domain}"
    if "harness" in lowered or "scenario" in lowered:
        return f"scenario/{domain}"
    if "closeout" in lowered:
        return f"courtroom/{domain}/closeout"
    return f"courtroom/{domain}"


def target_name(relative: str) -> str:
    path = PurePosixPath(relative)
    name = clean_name(path.name)
    if name in {"s6.rs", "6.rs"}:
        return "program.rs"
    return name


def classify(row: dict[str, str], hashes: dict[str, str]) -> dict[str, str]:
    current = row["current_path"]
    relative = current.removeprefix(PREFIX)
    domain = domain_for(relative)
    action = "move"
    target_crate = "worth-store-certification"
    target_dir = category_for(relative, domain, row["semantic_role"])
    name = target_name(relative)
    confidence = "medium"
    rationale = f"Rehome milestone-shaped certification code under the permanent {domain} {target_dir.split('/')[0]} boundary."

    harness = harness_target(relative, domain)
    if harness:
        target_crate, target_dir, name = harness
        rationale = "Move reusable harness fixtures, execution, or observations to physical certification while keeping verdict policy in the courtroom."
    if relative == "public_api.rs":
        action = "keep"
        target_dir = "src"
        name = "public_api.rs"
        confidence = "high"
        rationale = "This is an aggregation-only lifecycle-ordered public facade."
    elif relative == "courtroom/closeout/mod.rs":
        action = "split"
        target_dir = "courtroom"
        name = ""
        confidence = "high"
        rationale = "The generic closeout barrel couples unrelated courtroom domains; split exports into domain-owned closeout facades."
    elif relative == "s4_recovery_harness/mod.rs":
        action = "split"
        target_dir = "scenarios/recovery"
        name = ""
        confidence = "high"
        rationale = "Split the milestone harness barrel across permanent recovery scenarios, faults, observations, and courtroom checks."
    elif relative in DELETE_PATHS:
        action = "delete"
        target_dir = "displaced_milestone_scaffolding"
        name = ""
        confidence = "high"
        rationale = "The file certifies or re-exports a future-milestone handoff already removed from production authority."
    elif int(row["line_count"] or 0) > 400:
        action = "split"
        name = ""
        confidence = "high"
        rationale = "The certification file exceeds the line cap and contains multiple scenario or assertion families."

    target_path = ""
    if action in {"move", "keep"}:
        target_path = f"workspaces/worth-store/crates/{target_crate}/src/{target_dir}/{name}"
        if target_dir == "src":
            target_path = f"workspaces/worth-store/crates/{target_crate}/src/{name}"

    content_hash = hashes.get(current) or row.get("reviewed_content_sha256", "")
    if not content_hash:
        raise ValueError(f"missing fingerprint for {current}")
    return {
        "current_path": current,
        "reviewed_content_sha256": content_hash,
        "review_batch": BATCH,
        "proposed_action": action,
        "candidate_action": action,
        "target_crate": target_crate,
        "target_subsystem": target_dir,
        "target_directory": target_dir if action != "delete" else "",
        "target_path": target_path,
        "mechanical_ready": "false",
        "confidence": confidence,
        "content_review_required": "false",
        "rationale": rationale,
    }


def render() -> str:
    hashes = load_hashes()
    rows = [classify(row, hashes) for row in load_rows()]
    if len(rows) != 120:
        raise ValueError(f"expected 120 certification rows, found {len(rows)}")
    targets = [row["target_path"] for row in rows if row["target_path"]]
    duplicates = sorted({target for target in targets if targets.count(target) > 1})
    if duplicates:
        raise ValueError(f"colliding certification targets: {duplicates}")
    output = io.StringIO(newline="")
    writer = csv.DictWriter(output, fieldnames=FIELDS, lineterminator="\n")
    writer.writeheader()
    writer.writerows(sorted(rows, key=lambda row: row["current_path"]))
    return output.getvalue()


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    generated = render()
    if args.check:
        return 0 if OUTPUT.exists() and OUTPUT.read_text(encoding="utf-8") == generated else 1
    OUTPUT.write_text(generated, encoding="utf-8", newline="")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
